# Universal Dictionary Conversion for ODict

This repo contains a collection of Rust-based converters for various dictionary formats and is a port of the older, Python-based converter repository. Currently, the following dictionaries are supported:

- ✅ English Wiktionary (via [Kaikki](https://kaikki.org))
  - ✅ Arabic (ara) to English (eng)
  - ✅ Chinese (cmn) to English (eng)
  - ✅ English (eng) to English (eng)
  - ✅ French (fra) to English (eng)
  - ✅ German (ger) to English (eng)
  - ✅ Italian (ita) to English (eng)
  - ✅ Japanese (jpn) to English (eng)
  - ✅ Polish (pol) to English (eng)
  - ✅ Portuguese (por) to English (eng)
  - ✅ Russian (rus) to English (eng)
  - ✅ Spanish (spa) to English (eng)
  - ✅ Swedish (swe) to English (eng)
- ✅ CEDict (via [MDBG](https://www.mdbg.net/chinese))
  - ✅ Chinese (cmn) to English (eng)

To run this code, you'll need [`mise`](https://mise.jdx.dev/) installed alongside Rust. You can get started by running:

```bash
mise install
mise run convert <dictionary> <language_code>
```

So for example, to download English Wiktionary, you would run:

```bash
mise run convert wiktionary eng
```