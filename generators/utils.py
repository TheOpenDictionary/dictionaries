from lxml import etree


class Definition:
    def __init__(self, text: str, examples: list[str] = None) -> None:
        self.text = text
        self.examples = examples or []

    def xml(self):
        node = etree.Element("definition", attrib={"value": self.text})

        for example in self.examples:
            example_node = etree.Element("example")
            example_node.text = example
            node.append(example_node)

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


class DefinitionNode:
    """
    A special kind of definition class that will resolve to a Group if it has children,
    and a regular Definition node if it doesn't.
    Used in tree traversal, in generators such as wiktextract.
    """

    def __init__(
        self,
        definitions: dict[str, any] = None,
        examples: list[str] = None,
        text: str = "",
    ) -> None:
        self.definitions = definitions or {}
        self.examples = examples or []
        self.text = text

    def convert(self) -> Group or Definition:
        defs = list(self.definitions.values())

        # There are some weird cases in Wiktextract dumps where a definition has itself as a child
        if len(defs) == 1 and defs[0].text == self.text:
            return Definition(text=defs[0].text, examples=defs[0].examples)
        elif len(self.definitions) > 0:
            return Group(
                description=self.text,
                definitions=filter(
                    # TODO: remove this filter once groups can support sub-groups
                    lambda x: isinstance(x, Definition),
                    [definition.convert() for definition in defs],
                ),
            )
        else:
            return Definition(text=self.text, examples=self.examples)


class Usage:
    def __init__(
        self,
        partOfSpeech: str = "",
        description: str = "",
        groups: list[Group] = None,
        definitions: list[Definition] = None,
    ) -> None:
        self.pos = partOfSpeech
        self.groups = groups or []
        self.description = description
        self.definitions = definitions or []

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
    def __init__(
        self, number: int = 1, usages: list[Usage] = None, description: str = ""
    ) -> None:
        self.usages = usages or []
        self.number = number
        self.description = description

    def xml(self):
        node = etree.Element("ety", attrib={"description": self.description or ""})

        for usage in self.usages:
            node.append(usage.xml())

        return node


class Entry:
    def __init__(
        self,
        term: str,
        see: str = "",
        pronunciation: str = "",
        etymologies: list[Etymology] = None,
    ) -> None:
        self.etymologies = etymologies or []
        self.see = see
        self.term = term
        self.pronunciation = pronunciation

    def xml(self):
        node = etree.Element(
            "entry",
            attrib={
                "see": self.see,
                "pronunciation": self.pronunciation,
                "term": self.term,
            },
        )

        for ety in sorted(self.etymologies, key=lambda x: x.number):
            node.append(ety.xml())

        return node


class Dictionary:
    def __init__(self, name: str, entries: list[Entry] = None) -> None:
        self.name = name
        self.entries = entries or []

    def xml(self):
        node = etree.Element("dictionary", attrib={"name": self.name})

        for entry in self.entries:
            node.append(entry.xml())

        return node
