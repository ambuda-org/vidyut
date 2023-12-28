#!/usr/bin/env python3

import json
import sys
from pathlib import Path


source = Path(sys.argv[1])
with open(source) as f:
    data = json.load(f)

for m in data["metres"]:
    name, data = m

    if source.name == "matra.json":
        regex_str = "/".join(data["regex"])
        print(f"{name}\t{regex_str}")
    elif source.name == "vrtta.json":
        if isinstance(data, list):
            weight_str = "/".join(data)
        else:
            weight_str = data
        print(f"{name}\t{weight_str}")
