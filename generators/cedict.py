import gzip
from pathlib import Path
import requests
import re

from os import path
from xml.etree.ElementTree import Element, tostring
from tempfile import TemporaryDirectory
from theopendictionary import Dictionary

url = "https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz"

with TemporaryDirectory() as dirpath:
    try:
        file_name = url.split("/")[-1]
        blob = requests.get(url).content
        output_path = path.join(dirpath, file_name)

        new_file = open(output_path, "w+b")
        new_file.write(blob)
        new_file.close()

        g = gzip.open(output_path, "rb")

        root = Element("dictionary")

        root.attrib["name"] = "CC-CEDICT"
        entries = {}

        while True:
            line = g.readline().decode("utf-8")

            if not line:
                break

            m = re.match("(.*?)\s(.*?)\s\[(.*?)]\s/(.*)/", line)

            if m is not None:
                traditional = m.group(1)
                simplified = m.group(2)
                pronunciation = m.group(3)
                definitions = m.group(4).split("/")

                print("Processing word %s..." % simplified)

                entry = entries.get(
                    simplified,
                    Element(
                        "entry",
                        attrib={
                            "term": simplified,
                            "pronunciation": pronunciation,
                        },
                    ),
                )

                ety = Element("ety")
                usage = Element("usage")

                for deff in definitions:
                    d = Element("definition", attrib={"value": deff})
                    usage.append(d)

                ety.append(usage)
                entry.append(ety)

                entries[simplified] = entry

        g.close()

        print('Writing to "cedict.odict"...')

        [root.append(e) for e in entries.values()]

        xml = tostring(root).decode("utf-8")

        dict_base = "dictionaries/cedict"

        Path(dict_base).mkdir(parents=True, exist_ok=True)

        with open("%s/zho-eng.xml" % dict_base, "w") as f:
            f.write(xml)

        Dictionary.write(xml, "%s/zho-eng.odict" % dict_base)
    except Exception as e:
        print(e)
