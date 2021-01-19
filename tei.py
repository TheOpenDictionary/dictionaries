import tarfile
import xml.etree.ElementTree as ET

from bs4 import BeautifulSoup


def tei_to_odxml(tei_doc):
    document = BeautifulSoup(tei_doc, 'lxml')
    entries = document.body.findAll('entry')
    root = ET.Element("dictionary", attrib={'name': 'FreeDict'})

    for entry in entries:
        term = entry.orth.getText()
        entry_attr = {'term': term}

        print("Processing word \"%s\"..." % term)

        if entry.pron is not None:
            entry_attr['pronunciation'] = entry.pron.getText()

        entry_node = ET.Element("entry", attrib=entry_attr)
        ety_node = ET.Element("ety")
        senses = entry.findAll('sense')

        for sense in senses:
            usage_node = ET.Element("usage")
            citations = sense.findAll('cit')

            for cit in citations:
                def_node = ET.Element("definition")

                def_node.text = cit.getText().strip()

                usage_node.append(def_node)

                if len(citations) > 0:
                    entry_node.append(usage_node)

            ety_node.append(usage_node)
            entry_node.append(ety_node)

            if len(senses) > 0:
                root.append(entry_node)

    return ET.tostring(root).decode('utf-8')


def read_tei_archive(path):
    with tarfile.open(path) as tar:
        for member in tar.getmembers():
            f = tar.extractfile(member)

            if ".tei" in member.name:
                return f.read()
    return None
