"""Creates dhatu metadata based on data from ashtadhyayi.com.

Usage:

    uv run fetch_dhatu_metadata.py
"""


import csv
import io
import json
import pprint
import urllib.request
from vidyut.lipi import transliterate, Scheme


def load_metadata() -> dict:
    url = "https://github.com/ashtadhyayi-com/data/raw/refs/heads/master/dhatu/data.txt"
    f = urllib.request.urlopen(url)
    return json.load(f)


data = load_metadata()
dhatus = data["data"]

out = io.StringIO()
w = csv.writer(out)
w.writerow(["code", "artha_en", "artha_hi", "karma", "pada", "settva"])
for dhatu in dhatus:
    artha_en = dhatu["artha_english"]
    artha_hi = dhatu["artha_hindi"]
    code = dhatu["baseindex"]
    karma = dhatu["karma"]
    pada = dhatu["pada"]
    settva = dhatu["settva"]

    assert karma in {"S", "A", "D", '-'}, karma
    assert pada in {"P", "A", "U", '-'}, pada
    assert settva in {"S", "A", "V", '-'}, settva

    if karma == '-':
        assert karma == pada == settva == '-'
        continue

    w.writerow((code, artha_en, artha_hi, karma, pada, settva))

text = out.getvalue()
print(text)
