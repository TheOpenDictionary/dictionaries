
import requests
import asyncio

from os import path
from pathlib import Path
from python.odict import Dictionary
from ctypes import *
from os import path
from tempfile import TemporaryDirectory
from tei import tei_to_odxml, read_tei_archive


async def process_dict(language_pair, url):
    with TemporaryDirectory() as dirpath:
        print("> Processing language pair %s..." % language_pair)

        file_name = url.split('/')[-1]
        output_path = path.join(dirpath, file_name)

        if not path.exists(output_path):
            print("> Downloading dictionary from %s..." % url)
            blob = requests.get(url).content
            new_file = open(output_path, 'w+b')
            new_file.write(blob)
            new_file.close()
            print("> Download complete!")

        content = read_tei_archive(output_path)
        dictionary = tei_to_odxml(content)
        dict_path = "dictionaries/%s.odict" % language_pair

        print('> Writing to "%s"...' % dict_path)

        Dictionary.write(dictionary, dict_path)


async def process():

    Path('dictionaries').mkdir(parents=True, exist_ok=True)

    json = requests.get("https://freedict.org/freedict-database.json").json()

    tasks = []

    for j in json:
        if "name" in j:
            language_pair = j["name"]
            if language_pair == "spa-deu":
                for release in j["releases"]:
                    if release["platform"] == "src":
                        url = release["URL"]
                        tasks.append(asyncio.ensure_future(
                            process_dict(language_pair, url)))

    await asyncio.gather(*tasks)

if __name__ == "__main__":
    asyncio.run(process())
