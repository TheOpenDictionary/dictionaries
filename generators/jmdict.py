import gzip
from os import path
import re
import sys
from tempfile import TemporaryDirectory
from alive_progress import alive_bar
from bs4 import BeautifulSoup
from lxml import etree
import requests
from ftputil import FTPHost
from theopendictionary import Dictionary as ODictionary

from utils import Dictionary, Entry, Etymology, Usage, Definition

url = "ftp://ftp.edrdg.org/pub/Nihongo//JMdict.gz"

target_lang = sys.argv[1] if len(sys.argv) > 1 else "eng"


def resolve_pos(pos):
    if pos.startswith("adj-"):
        return "adj"

    if pos.startswith("n-"):
        return "n"

    if pos.startswith("adv-"):
        return "adv"

    if pos.startswith("v"):
        return "v"

    if pos == "int":
        return "intj"

    if pos == "prt":
        return "part"

    if pos == "pn":
        return "pro"

    if pos == "suf":
        return "suff"

    if pos in ["n", "v", "conj"]:
        return pos

    return "un"


with TemporaryDirectory() as dirpath:
    file_name = url.split("/")[-1]
    output_path = path.join(dirpath, "JMdict.gz")

    # if not path.exists(output_path):
    #     print("> Downloading latest JMDict version...")

    #     ftp_host = FTPHost("ftp.edrdg.org", "anonymous", "")
    #     ftp_host.download("/pub/Nihongo//JMdict.gz", output_path)

    #     print("> Download complete!")

    with gzip.open("/Users/tjnickerson/Downloads/JMDict.gz", "rb") as f:
        content = f.read().decode("utf-8")

        print("> Reading into memory (this might take some time)...")

        document = BeautifulSoup(re.sub(r"&(\w+?);", r"\1", content), features="xml")

        entries = document.find_all("entry")

        root = Dictionary(name="JMDict")

        with alive_bar(len(entries), title="> Processing entries...") as bar:
            for entry in entries:
                keb = entry.find("keb")
                reb = entry.find_all("reb")

                if keb:
                    term = keb.text
                    pronunciation = reb[0].text if len(reb) > 0 else None
                    senses = entry.find_all("sense")
                    usages: list[Usage] = []
                    misc = entry.find_all("misc")
                    fallback_pos = entry.find("pos")
                    usually_kana = any(m.text == "uk" for m in misc)

                    if usually_kana:
                        for r in reb:
                            root.entries.append(Entry(term=r.text, see=term))

                    for sense in senses:
                        pos = sense.find("pos") or fallback_pos
                        pos = resolve_pos(pos.text) if pos else None
                        glosses = sense.find_all("gloss")
                        inf = sense.find("s_inf")
                        description = inf.text if inf is not None else ""
                        definitions = [
                            Definition(text=gloss.text)
                            for gloss in glosses
                            if gloss.get("xml:lang")
                            == (None if target_lang == "eng" else target_lang)
                        ]

                        if len(definitions) > 0:
                            usages.append(
                                Usage(
                                    partOfSpeech=pos,
                                    description=description,
                                    definitions=definitions,
                                )
                            )

                    if len(usages) > 0:
                        root.entries.append(
                            Entry(term=term, etymologies=[Etymology(usages=usages)])
                        )

                bar()

        print("> Creating file...")

        xml = etree.tostring(root.xml()).decode("utf-8")

        ODictionary.write(xml, "dictionaries/jmdict/jpn-%s.odict" % target_lang)

        print("> Dictionary written!")
        exit()