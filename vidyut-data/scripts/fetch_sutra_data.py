import csv
import json
import urllib.request
from vidyut.lipi import transliterate, Scheme

PHIT = "https://raw.githubusercontent.com/ashtadhyayi-com/data/refs/heads/master/fit/data.txt"
LINGA = "https://raw.githubusercontent.com/ashtadhyayi-com/data/refs/heads/master/linganushasanam/data.txt"


def process_phits():
    f = urllib.request.urlopen(PHIT)
    data = json.load(f)

    rows = []
    for row in data["data"]:
        pada = row["p"]
        number = row["n"]

        code = f"{pada}.{number}"
        text = transliterate(row["s"], Scheme.Devanagari, Scheme.Slp1)
        text = text.strip()
        rows.append((code, text))

    out_file = "../../vidyut-prakriya/data/phit-sutras.tsv"
    with open(out_file, 'w') as f:
        w = csv.writer(f, delimiter="\t")
        w.writerow(["code", "text"])
        for row in rows:
            w.writerow(row)


def process_lingas():
    f = urllib.request.urlopen(LINGA)
    data = json.load(f)

    rows = []
    for row in data["data"]:
        code = str(row["id"])
        text = transliterate(row["sutra"], Scheme.Devanagari, Scheme.Slp1)
        text = text.strip()
        rows.append((code, text))

    out_file = "../../vidyut-prakriya/data/linganushasanam.tsv"
    with open(out_file, 'w') as f:
        w = csv.writer(f, delimiter="\t")
        w.writerow(["code", "text"])
        for row in rows:
            w.writerow(row)

process_phits()
process_lingas()
