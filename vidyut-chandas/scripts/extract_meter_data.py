#!/usr/bin/env python3

import json
import sys
from pathlib import Path

import requests
from indic_transliteration import sanscript

# Reuse metrical data sourced by Dr. Dhaval Patel, via  @shreevata
source = "https://raw.githubusercontent.com/shreevatsa/sanskrit/master/data/vrttaratnakara.json"

resp = requests.get(source)
data = json.loads(resp.content)

for iast_name, pattern in data["metres"]:
    if isinstance(pattern, list):
        weight_str = "/".join(pattern)
    else:
        weight_str = pattern

    # Normalize yati
    weight_str = weight_str.replace("—", "|").replace(" ", "")

    if set(weight_str) == set([".", "/"]):
        # Skip padacaturUrdhva, which is weird
        continue

    slp_name = sanscript.transliterate(iast_name, "iast", "slp1")

    print(f"{slp_name}\tvrtta\t{weight_str}")
