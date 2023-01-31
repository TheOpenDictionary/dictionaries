import json

from utils import Dictionary, Definition, DefinitionNode, Group, Usage, Etymology, Entry
from lxml import etree

entries = {}
data = []
dict = Dictionary(name="English Wiktionary")
definitions = {}

with open("/Users/tjnickerson/Downloads/kaikki.org-dictionary-English.json", "r") as f:
    for line in f.readlines():
        map = json.loads(line)

        pos = map.get("pos")

        term = map.get("word")

        pronunciation = (
            map.get("sounds")[0].get("ipa")
            if map.get("sounds") and len(map.get("sounds")) > 0
            else None
        )

        etymology_description = map.get("etymology_text")

        root = DefinitionNode()

        senses = map.get("senses")

        for sense in senses:
            glosses = sense.get("glosses")
            raw_glosses = sense.get("raw_glosses") or glosses or []

            if raw_glosses:
                definition_str = raw_glosses[-1]
                key = glosses[-1]
                definition = Definition(text=definition_str)

                node = root.definitions

                for gloss in glosses:
                    if gloss not in node:
                        print("yes")
                        node[gloss] = {
                            "description": definition_str,
                            "definitions": {},
                        }
                    node = node[gloss]["definitions"]

        if term == "abolitionism":
            print(definitions)
            exit(0)
        # if len(glosses) > 2:
        #     print(glosses)
        #     exit(0)

        # if len(glosses) > 1:
        #     parent = glosses[0]

        #     if parent in definitions:
        #         if isinstance(definitions[parent], Definition):
        #             definitions[parent] = Group(
        #                 description=parent,
        #                 definitions=[definition],
        #             )
        #         else:
        #             definitions[parent].definitions.append(definition)
        #     else:
        #         definitions[parent] = Group(
        #             description=parent,
        #             definitions=[definition],
        #         )
        # elif key in definitions:
        #     definitions[key].description = definition
        # else:
        #     definitions[key] = definition

        # groups_and_defs = definitions.values()

        # groups = filter(lambda x: isinstance(x, Group), groups_and_defs)

        # defs = filter(lambda x: isinstance(x, Definition), groups_and_defs)

        # usage = Usage(
        #     partOfSpeech=pos,
        #     description=etymology_description,
        #     groups=groups,
        #     definitions=defs,
        # )

        # ety = Etymology(usages=[usage], description=etymology_description)

        # entry = Entry(term=term, pronunciation=pronunciation, etymologies=[ety])

        # if term == "dagger":
        # print(line)
        # print(etree.tostring(entry.xml()))
        # exit(0)
        # if len(glosses) > 1:
        #     print(line)
        #     exit(0)

        # for gloss in glosses:
        #     definitions.append(Definition(text=gloss))

        # print(term)
        # print(glosses)
        # data.append()

print(data[0])
