import gzip
from os import path
from pathlib import Path
import re
import sys
from tempfile import TemporaryDirectory
from alive_progress import alive_bar
from bs4 import BeautifulSoup
from lxml import etree
from ftputil import FTPHost
from theopendictionary import Dictionary as ODictionary

from utils import Dictionary, Entry, Etymology, Usage, Definition

url = "ftp://ftp.edrdg.org/pub/Nihongo//JMdict.gz"


pos_resolution = {
    "exp": "expr",
    "int": "intj",
    "pn": "pron",
    "adj-i": "adj",
    "prt": "part",
    "suf": "suff",
    "pre": "pref",
    "unc": "un",
}


def resolve_pos(pos):
    return pos_resolution.get(pos, pos)


with TemporaryDirectory() as dirpath:

    jmdict_path = path.join(dirpath, "JMdict.gz")

    if not path.exists(jmdict_path):
        print("> Downloading latest JMDict version...")

        ftp_host = FTPHost("ftp.edrdg.org", "anonymous", "")
        ftp_host.download("/pub/Nihongo//JMdict.gz", jmdict_path)

        print("> Download complete!")

    with gzip.open(jmdict_path, "rb") as f:
        content = f.read().decode("utf-8")

        print("> Reading into memory (this might take some time)...")

        document = BeautifulSoup(re.sub(r"&(.+?);", r"\1", content), features="xml")

        entries = document.find_all("entry")

        def run(target_lang: str):
            root = Dictionary(name="JMDict")

            with alive_bar(
                len(entries), title="> Processing entries for %s..." % target_lang
            ) as bar:
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
                                root.entries.append(
                                    Entry(
                                        term=r.text,
                                        pronunciation=pronunciation,
                                        see=term,
                                    )
                                )

                        for sense in senses:
                            pos = sense.find("pos") or fallback_pos
                            pos = resolve_pos(pos.text) if pos else ""
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

            xml = etree.tostring(root.xml(), pretty_print=True).decode("utf-8")
            file_dir = "dictionaries/jmdict"
            file_base = "%s/jpn-%s" % (file_dir, target_lang)

            Path(file_dir).mkdir(parents=True, exist_ok=True)

            with open("%s.xml" % file_base, "w") as f:
                f.write(xml)

            ODictionary.write(xml, "%s.odict" % file_base)

            print("> Dictionary written!")

        lang = sys.argv[1] if len(sys.argv) > 1 else "all"
        langs = ["eng", "ger", "spa", "rus", "hun", "fre", "dut", "swe", "slv"]

        if lang == "all":
            for lc in langs:
                print("> Generating dictionary for %s..." % lc)
                run(lc)
            exit(0)
        else:
            if lang not in langs:
                print(
                    "Invalid target language! Must be one of the following: "
                    + ", ".join(langs)
                )
                sys.exit(1)

            run(lang)
            exit(0)
