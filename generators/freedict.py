import requests
import asyncio
import tarfile

from os import path
from pathlib import Path
from theopendictionary import Dictionary
from ctypes import *
from os import path
from tempfile import TemporaryDirectory
from lxml import etree
from bs4 import BeautifulSoup
from alive_progress import alive_bar


class Definition:
    def __init__(self, text: str) -> None:
        self.text = text

    def xml(self):
        node = etree.Element("definition")
        node.text = self.text
        return node


class Group:
    def __init__(self, definitions: list[Definition], description: str = "") -> None:
        self.definitions = definitions
        self.description = description

    def xml(self) -> str:
        node = etree.Element("group", attrib={"description": self.description})

        for definition in self.definitions:
            node.append(definition.xml())

        return node


class Usage:
    def __init__(
        self,
        partOfSpeech: str = "",
        description: str = "",
        groups: list[Group] = [],
        definitions: list[Definition] = [],
    ) -> None:
        self.pos = partOfSpeech
        self.groups = groups
        self.description = description
        self.definitions = definitions

    def xml(self):
        node = etree.Element(
            "usage", attrib={"pos": self.pos, "description": self.description}
        )

        for group in self.groups:
            node.append(group.xml())

        for definition in self.definitions:
            node.append(definition.xml())

        return node


class Etymology:
    def __init__(self, usages: list[Usage] = [], description: str = "") -> None:
        self.usages = usages
        self.description = description

    def xml(self):
        node = etree.Element("ety", attrib={"description": self.description})

        for usage in self.usages:
            node.append(usage.xml())

        return node


class Entry:
    def __init__(
        self, term: str, pronunciation: str = "", etymologies: list[Etymology] = []
    ) -> None:
        self.etymologies = etymologies
        self.term = term
        self.pronunciation = pronunciation

    def xml(self):
        node = etree.Element("entry", attrib={"term": self.term})

        for ety in self.etymologies:
            node.append(ety.xml())

        return node


def tei_to_odxml(tei_doc):
    document = BeautifulSoup(tei_doc, features="xml")
    root = etree.Element("dictionary", attrib={"name": "FreeDict"})
    entries = {}

    with alive_bar(title="> Processing entries...") as bar:
        for entry in document.body.findAll("entry"):
            term = entry.orth.getText()
            pron = entry.pron.getText() if entry.pron is not None else ""
            usages: list[Usage] = []

            for sense in entry.findAll("sense"):
                defs: list[Definition] = []

                for cit in sense.findAll("cit"):
                    defs.append(Definition(cit.getText().strip()))

                if len(defs) > 0:
                    usages.append(Usage(definitions=defs))

            if len(usages) > 0:
                entries[term] = Entry(
                    term, pronunciation=pron, etymologies=set([Etymology(usages)])
                )
                bar()

    for entry in entries.values():
        root.append(entry.xml())

    return etree.tostring(root).decode("utf-8")


def read_tei_archive(path):
    with tarfile.open(path) as tar:
        for member in tar.getmembers():
            f = tar.extractfile(member)

            if ".tei" in member.name:
                return f.read()
    return None


async def process_dict(language_pair, url):
    with TemporaryDirectory() as dirpath:
        print("> Processing language pair %s..." % language_pair)
        file_name = url.split("/")[-1]
        output_path = path.join(dirpath, file_name)

        if not path.exists(output_path):
            print("> Downloading dictionary from %s..." % url)
            blob = requests.get(url).content
            new_file = open(output_path, "w+b")
            new_file.write(blob)
            new_file.close()
            print("> Download complete!")

        content = read_tei_archive(output_path)
        dictionary = tei_to_odxml(content)
        dict_path = "dictionaries/freedict/%s.odict" % language_pair

        print('> Writing to "%s"...' % dict_path)

        Dictionary.write(dictionary, dict_path)


async def process():

    Path("dictionaries").mkdir(parents=True, exist_ok=True)

    json = requests.get("https://freedict.org/freedict-database.json").json()

    tasks = []

    for j in json:
        if "name" in j:
            language_pair = j["name"]
            for release in j["releases"]:
                if release["platform"] == "src":
                    url = release["URL"]
                    tasks.append(
                        asyncio.ensure_future(process_dict(language_pair, url))
                    )

    await asyncio.gather(*tasks)


if __name__ == "__main__":
    asyncio.run(process())
