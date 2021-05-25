from lxml import etree


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
        node = etree.Element("group", attrib={'description': self.description})

        for definition in self.definitions:
            node.append(definition.xml())

        return node


class Usage:
    def __init__(self, partOfSpeech: str = "", description: str = "", groups: list[Group] = [], definitions: list[Definition] = []) -> None:
        self.pos = partOfSpeech
        self.groups = groups
        self.description = description
        self.definitions = definitions

    def xml(self):
        node = etree.Element(
            "usage", attrib={'pos': self.pos, 'description': self.description})

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
        node = etree.Element(
            "ety", attrib={'description': self.description})

        for usage in self.usages:
            node.append(usage.xml())

        return node


class Entry:
    def __init__(self, term: str, pronunciation: str = "", etymologies: list[Etymology] = []) -> None:
        self.etymologies = etymologies
        self.term = term
        self.pronunciation = pronunciation

    def xml(self):
        node = etree.Element(
            "entry", attrib={'term': self.term})

        for ety in self.etymologies:
            node.append(ety.xml())

        return node
