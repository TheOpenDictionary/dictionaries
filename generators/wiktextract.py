from json import loads as load_json
from os import makedirs, path
import sys
from tempfile import TemporaryDirectory

from alive_progress import alive_bar
import requests
from theopendictionary import Dictionary as ODictionary
from utils import Dictionary, Definition, DefinitionNode, Group, Usage, Etymology, Entry
from lxml import etree

entries = {}
data = []
dict = Dictionary(name="English Wiktionary")
dict_base = "dictionaries/wiktionary"

if not path.exists(dict_base):
    makedirs(dict_base)

pos_map = {
    "abbrev": "abv",
    "adv_phrase": "phr_adv",
    "affix": "aff",
    "article": "art",
    "character": "chr",
    "circumfix": "cf",
    "infix": "inf",
    "interfix": "intf",
    "noun": "n",
    "particle": "part",
    "phrase": "phr",
    "prefix": "pref",
    "prep_phrase": "phr_prep",
    "punct": "punc",
    "suffix": "suff",
    "symbol": "sym",
    "verb": "v",
}

lang_map = {
    "eng": "English",
    "fra": "French",
    "ger": "German",
    "ita": "Italian",
    "pol": "Polish",
    "spa": "Spanish",
    "swe": "Swedish",
    "jpn": "Japanese",
    "rus": "Russian",
    "ara": "Arabic",
    "cmn": "Chinese",
}


pos_tags = set()
entries = {}


def download_dictionary(lang: str, outdir: str):
    print("> Downloading dictionary for %s..." % lang)

    lang_name = lang_map.get(lang)

    if lang_name is None:
        raise Exception(
            "Language not supported: %s. Must be one of %s" % (lang, lang_map.keys())
        )

    url = "https://kaikki.org/dictionary/%s/kaikki.org-dictionary-%s.json" % (
        lang_name,
        lang_name,
    )

    output_path = path.join(outdir, lang + ".gz")
    blob = requests.get(url).content

    new_file = open(output_path, "w+b")
    new_file.write(blob)
    new_file.close()

    return output_path


def run(target_lang: str):
    with TemporaryDirectory() as dirpath:
        dict_path = download_dictionary(target_lang, dirpath)

        with open(dict_path, "r") as f:
            with alive_bar(title="> Processing entries for %s..." % target_lang) as bar:
                for line in f.readlines():
                    try:
                        json = load_json(line)
                        raw_pos = json.get("pos")
                        pos = pos_map.get(raw_pos) or raw_pos
                        term = json.get("word")
                        should_add = False
                        pronunciation = (
                            json.get("sounds")[0].get("ipa")
                            if json.get("sounds") and len(json.get("sounds")) > 0
                            else None
                        ) or ""

                        etymology_description = json.get("etymology_text")

                        root = DefinitionNode()

                        senses = json.get("senses")

                        for sense in senses:
                            glosses = sense.get("glosses") or []
                            raw_glosses = sense.get("raw_glosses") or glosses
                            examples = [
                                ex
                                for item in filter(
                                    lambda x: x.get("type") == "example"
                                    or x.get("type") is None,
                                    sense.get("examples") or [],
                                )
                                for ex in item.get("text").split("\n")
                            ]

                            if raw_glosses:
                                definition_str = raw_glosses[-1]
                                node = root

                                for gloss in glosses:
                                    if gloss not in node.definitions:
                                        should_add = True
                                        node.definitions[gloss] = DefinitionNode(
                                            text=gloss
                                        )
                                    node = node.definitions[gloss]

                                node.text = definition_str
                                node.examples = examples

                        if should_add:
                            groups_and_defs = [
                                node.convert() for node in root.definitions.values()
                            ]

                            groups = filter(
                                lambda x: isinstance(x, Group), groups_and_defs
                            )

                            defs = filter(
                                lambda x: isinstance(x, Definition), groups_and_defs
                            )

                            usage = Usage(
                                partOfSpeech=pos,
                                groups=groups,
                                definitions=defs,
                            )

                            ety = Etymology(
                                usages=[usage], description=etymology_description
                            )

                            if term in entries:
                                entries[term].etymologies.append(ety)
                            else:
                                entries[term] = Entry(
                                    term=term,
                                    pronunciation=pronunciation,
                                    etymologies=[ety],
                                )

                        bar()
                    except Exception as e:
                        print(e)
                        print("Received on line: " + line)

        print("> Writing dictionary to file...")

        for entry in entries.values():
            dict.entries.append(entry)

        dictionary = etree.tostring(dict.xml(), pretty_print=True).decode("utf-8")

        output_path = "%s/%s" % (dict_base, "%s-eng" % target_lang)

        with open(output_path + ".xml", "w") as f:
            f.write(dictionary)

        ODictionary.write(dictionary, output_path + ".odict")


lang = sys.argv[1] if len(sys.argv) > 1 else "all"
langs = lang_map.keys()

if lang == "all":
    for lc in langs:
        print("> Generating dictionary for %s..." % lc)
        run(lc)
    exit(0)
else:
    if lang not in langs:
        print(
            "Invalid target language! Must be one of the following: " + ", ".join(langs)
        )
        sys.exit(1)

    run(lang)
    exit(0)
