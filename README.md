# ODict <> FreeDict Converter 

This repo contains a tiny Python 3 script for converting the latest [FreeDict](https://freedict.org) dictionaries to the
ODict format. 

To run in an isolated Python 3 virtual environment, simply run:

```bash
$ python3 -m venv .venv
$ source .venv/bin/activate
$ pip install -r requirements.txt
$ python3 main.py
```

The compiled dictionaries are available under `./dictionaries`.