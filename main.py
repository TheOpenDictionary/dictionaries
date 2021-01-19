
import requests

from os import path
from pathlib import Path
from python.odict import Dictionary
from ctypes import *
from os import path
from tempfile import TemporaryDirectory
from tei import tei_to_odxml, read_tei_archive

Path('dictionaries').mkdir(parents=True, exist_ok=True)

json = requests.get("https://freedict.org/freedict-database.json").json()

for j in json:
    if "name" in j:
        language_pair = j["name"]

        for release in j["releases"]:
            if release["platform"] == "src":
                url = release["URL"]

                with TemporaryDirectory() as dirpath:
                    try:
                        print("\nProcessing dictionary: %s..." % url)

                        file_name = url.split('/')[-1]
                        blob = requests.get(url).content
                        output_path = path.join(dirpath, file_name)

                        new_file = open(output_path, 'w+b')
                        new_file.write(blob)
                        new_file.close()

                        content = read_tei_archive(output_path)
                        dictionary = tei_to_odxml(content)
                        dict_path = "dictionaries/%s.odict" % language_pair

                        print("Writing to \"%s\"..." % dict_path)

                        Dictionary.write(dictionary, dict_path)
                    except Exception as e:
                        print(e)
