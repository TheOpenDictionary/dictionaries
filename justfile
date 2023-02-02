freedict dictionary="all":
	poetry run python generators/freedict.py {{dictionary}}

cedict:
	poetry run python generators/cedict.py

ecdict:
	poetry run python generators/ecdict.py

jmdict language="all":
  poetry run python generators/jmdict.py {{language}}

wiktionary language="all": 
  poetry run python generators/wiktextract.py {{language}}

all: freedict cedict ecdict jmdict wiktionary
