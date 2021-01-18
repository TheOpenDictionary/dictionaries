import tarfile

from bs4 import BeautifulSoup
from xml.sax.saxutils import escape


def tei_to_odxml(tei_doc):
    soup = BeautifulSoup(tei_doc, 'lxml')
    entries = soup.body.findAll('entry')
    od_dictionary = "<dictionary>"

    for entry in entries:
        term = entry.orth.getText()

        print("Processing word \"%s\"..." % term)

        # pronunciation = entry.pron.getText()
        od_entry = "<entry term=\"%s\"><ety>" % escape(term)
        senses = entry.findAll('sense')

        for sense in senses:
            od_usage = "<usage>"
            citations = sense.findAll('cit')

            for cit in citations:
                od_usage += "<definition>%s</definition>" % escape(
                    cit.getText().strip())

            od_usage += "</usage>"

            if len(citations) > 0:
                od_entry += od_usage

        od_entry += "</ety></entry>"

        if len(senses) > 0:
            od_dictionary += od_entry

    od_dictionary += "</dictionary>"

    return od_dictionary


def read_tei_archive(path):
    with tarfile.open(path) as tar:
        for member in tar.getmembers():
            f = tar.extractfile(member)

            if ".tei" in member.name:
                return f.read()
    return None
