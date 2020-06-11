import requests

odict_version = 1.0
base_url = "https://github.com/odict/odict/releases/download/v%s" % odict_version
so_file = requests.get("%s/odict.so" % base_url)
h_file = requests.get("%s/odict.h" % base_url)

with open('odict.so', 'w+b') as so:
    so.write(so_file.content)

with open('odict.h', 'w+b') as h:
    h.write(h_file.content)

print("Downloaded required files odict.so and odict.h")