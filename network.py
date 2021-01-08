import re
import tarfile
import requests
import random

from os import path, mkdir
from tempfile import TemporaryDirectory
from distutils.version import StrictVersion
from urllib import parse
from bs4 import BeautifulSoup

user_agent_list = [
    # Chrome
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36',
    'Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36',
    'Mozilla/5.0 (Windows NT 5.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36',
    'Mozilla/5.0 (Windows NT 6.2; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36',
    'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/44.0.2403.157 Safari/537.36',
    'Mozilla/5.0 (Windows NT 6.3; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36',
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/57.0.2987.133 Safari/537.36',
    'Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/57.0.2987.133 Safari/537.36',
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36',
    'Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36',
    # Firefox
    'Mozilla/4.0 (compatible; MSIE 9.0; Windows NT 6.1)',
    'Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko',
    'Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0)',
    'Mozilla/5.0 (Windows NT 6.1; Trident/7.0; rv:11.0) like Gecko',
    'Mozilla/5.0 (Windows NT 6.2; WOW64; Trident/7.0; rv:11.0) like Gecko',
    'Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko',
    'Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.0; Trident/5.0)',
    'Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; rv:11.0) like Gecko',
    'Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)',
    'Mozilla/5.0 (Windows NT 6.1; Win64; x64; Trident/7.0; rv:11.0) like Gecko',
    'Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; WOW64; Trident/6.0)',
    'Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; Trident/6.0)',
    'Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; .NET CLR 2.0.50727; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729)'
]


def __list_files(url, regex):
    page = requests.get(url, headers={'User-Agent': random.choice(user_agent_list)}).text
    soup = BeautifulSoup(page, 'html.parser')
    file_urls = {}

    for node in soup.findAll('a'):
        right_cells = map(lambda x: x.getText(), node.parent.parent.findAll('td', {"align": "right"}))
        modified_date = [x.strip() for x in right_cells if re.match(r'\d{4}-\d{2}-\d{2} \d{2}:\d{2}', x)]
        href = node.get('href')
        uri = parse.urljoin(url, href).rsplit('?', 1)[0]

        if re.match(regex, uri) and len(modified_date) > 0:
            file_urls[href.replace('/', '').strip()] = uri

    return file_urls

def try_version(x):
  try:
    return StrictVersion(x[0].replace('-', '.'))
  except (ValueError, TypeError):
    return StrictVersion('0.0')

def get_files(on_each):
    with TemporaryDirectory() as dirpath:
        top_level = __list_files(
            'https://download.freedict.org/dictionaries/',
            r'https://download\.freedict\.org/dictionaries/[a-z]{3}-[a-z]{3}'
        )

        for x in top_level:
            url = top_level[x]

            language_pair = re.match(
                r'https://download\.freedict\.org/dictionaries/([a-z]{3}-[a-z]{3})',
                url
            ).group(1).strip()

            language_level = __list_files(
                url,
                r'https://download\.freedict\.org/dictionaries/[a-z]{3}-[a-z]{3}/\d+'
            )

            latest_version_url = sorted(
                language_level.items(),
                reverse=True,
                key=lambda x: try_version(x)
            )[0][1]

            version_level = __list_files(
                latest_version_url,
                r'https://download\.freedict\.org/dictionaries/[a-z]{3}-[a-z]{3}/.+/.+src\.tar\.xz$'
            )

            file_url = list(version_level.values())[0]

            print("\nProcessing file: %s..." % file_url)

            file_name = file_url.split('/')[-1]
            blob = requests.get(file_url).content
            output_path = path.join(dirpath, file_name)

            new_file = open(output_path, 'w+b')
            new_file.write(blob)
            new_file.close()

            with tarfile.open(output_path) as tar:
                for member in tar.getmembers():
                    f = tar.extractfile(member)

                    if ".tei" in member.name:
                        content = f.read()
                        on_each(language_pair, content)
