# ODict Dictionary Converters

This repo contains Python 3 scripts for converting various open-source dictionaries and formats to the ODict format. Currently, the dictionaries supported are [CEDict](https://www.mdbg.net/chinese/dictionary?page=cedict), [ECDict](https://github.com/skywind3000/ECDICT) and [FreeDict](https://freedict.org/). 

If you have a request for a dictionary, please open an issue!

To setup this repo:

1. First clone the repo and install [asdf](https://asdf-vm.com/). 
2. Run `asdf install`.
3. Run `poetry install`.
4. Run `just [freedict|ecdict|cedict]` (depending on the dictionary you wish to generate).

The compiled dictionaries are available under `./dictionaries`.
