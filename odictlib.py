import requests

from ctypes import *
from bs4 import BeautifulSoup
from os import path

odict_version = 1.2

base_url = "https://github.com/odict/odict/releases/download/%s" % odict_version
base_local = path.dirname(__file__)
so_path = path.join(base_local, 'odict.so')
h_path = path.join(base_local, 'odict.h')

if not path.exists(so_path):
    print("Downloading odict.so...")
    with open(so_path, 'w+b') as so:
        so.write(requests.get("%s/odict.so" % base_url).content)

if not path.exists(h_path):
    print("Downloading odict.h...")
    with open(h_path, 'w+b') as h:
        h.write(requests.get("%s/odict.h" % base_url).content)

print("Loading ODict library...")
lib = cdll.LoadLibrary(path.join(base_local, "odict.so"))


def write_odict(tei_doc, output_path):
    print("Creating \"%s\"..." % output_path)

    soup = BeautifulSoup(tei_doc, 'lxml')
    entries = soup.body.findAll('entry')
    od_dictionary = "<dictionary>"

    for entry in entries:
        term = entry.orth.getText()

        print("Processing word \"%s\"..." % term)

        # pronunciation = entry.pron.getText()
        od_entry = "<entry term=\"%s\"><ety>" % term
        senses = entry.findAll('sense')

        for sense in senses:
            od_usage = "<usage>"
            citations = sense.findAll('cit')

            for cit in citations:
                od_usage += "<definition>%s</definition>" % cit.getText().strip()

            od_usage += "</usage>"
            od_entry += od_usage

        od_entry += "</ety></entry>"
        od_dictionary += od_entry

    od_dictionary += "</dictionary>"

    lib.CreateDictionaryFromXML(str.encode(
        od_dictionary), str.encode(output_path))
