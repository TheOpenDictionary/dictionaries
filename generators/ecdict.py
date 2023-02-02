import csv
from os import path
from pathlib import Path
from tempfile import TemporaryDirectory

from xml.etree.ElementTree import Element, tostring
import requests
from theopendictionary import Dictionary


url = "https://raw.githubusercontent.com/TheOpenDictionary/ecdict/master/ecdict.csv"

with TemporaryDirectory() as dirpath:
    try:
        file_name = path.basename(url)
        blob = requests.get(url).content
        output_path = path.join(dirpath, file_name)

        new_file = open(output_path, "w+b")
        new_file.write(blob)
        new_file.close()

        root = Element("dictionary")

        root.attrib["name"] = "ECDICT"

        with open(output_path, newline="") as csvfile:
            reader = csv.reader(csvfile, delimiter=",")
            entries = {}

            for row in reader:
                m = dict(
                    zip(
                        [
                            "word",
                            "phonetic",
                            "definition",
                            "translation",
                            "pos",
                            "collins",
                            "oxford",
                            "tag",
                            "bnc",
                            "frq",
                            "exchange",
                            "detail",
                            "audio",
                        ],
                        row,
                    )
                )

                prn = m["phonetic"]
                attr = {"term": m["word"]}

                if len(prn) > 0:
                    attr["pronunciation"] = prn

                entry = entries.get(m["word"], Element("entry", attrib=attr))
                definitions = m["translation"].splitlines()

                print("Processing word %s..." % m["word"])

                ety = Element("ety")
                usage = Element("usage")

                for deff in definitions:
                    d = Element("definition", attrib={"value": deff})
                    usage.append(d)

                ety.append(usage)
                entry.append(ety)

                entries[m["word"]] = entry

            [root.append(entry) for entry in entries.values()]

            print("Writing dictionary to disk...")

            dict_base = "dictionaries/ecdict"

            Path(dict_base).mkdir(parents=True, exist_ok=True)

            xml = tostring(root).decode("utf-8")

            with open("%s/eng-zho.xml" % dict_base, "w") as f:
                f.write(xml)

            Dictionary.write(xml, "%s/eng-zho.odict" % dict_base)
    except Exception as e:
        print(e)
