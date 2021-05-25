import tarfile

from lxml import etree as ET
from bs4 import BeautifulSoup
from dict import Definition, Entry, Etymology, Usage
from alive_progress import alive_bar


def tei_to_odxml(tei_doc):
    document = BeautifulSoup(tei_doc, 'lxml')
    root = ET.Element("dictionary", attrib={'name': 'FreeDict'})
    entries = {}

    with alive_bar(title="> Processing entries...") as bar:
        for entry in document.body.findAll('entry'):
            term = entry.orth.getText()
            pron = entry.pron.getText() if entry.pron is not None else ""
            usages: list[Usage] = []

            for sense in entry.findAll('sense'):
                defs: list[Definition] = []

                for cit in sense.findAll('cit'):
                    defs.append(Definition(cit.getText().strip()))

                if len(defs) > 0:
                    usages.append(Usage(definitions=defs))

            if len(usages) > 0:
                entries[term] = Entry(term, pronunciation=pron,
                                      etymologies=set([Etymology(usages)]))
                bar()

    for entry in entries.values():
        root.append(entry.xml())

    return ET.tostring(root).decode('utf-8')


def read_tei_archive(path):
    with tarfile.open(path) as tar:
        for member in tar.getmembers():
            f = tar.extractfile(member)

            if ".tei" in member.name:
                return f.read()
    return None
