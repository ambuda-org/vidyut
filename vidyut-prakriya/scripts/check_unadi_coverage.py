#!/usr/bin/env python3
from pathlib import Path
from collections import Counter
import re
import glob

RULE_OK = "✅"
RULE_UNTESTED = "⚠️ "
RULE_MISSING = "❌"

base = Path(__file__).parent.parent

all_rules = []
with open(base / "data/unadipatha.tsv") as f:
    for line in f:
        code, text = line.strip().split('\t')
        all_rules.append(code)

tested_rules = set()
with open(base / "tests/kaumudi_67.rs") as f:
    for line in f:
        for match in re.findall(r'unadi_(\d+_\d+)', line):
            tested_rules.add(match.replace('_', '.'))


num_ok = 0
num_missing = 0
for code in all_rules:
    if code in tested_rules:
        num_ok += 1
        print(f"{RULE_OK} {code}")
    else:
        num_missing += 1
        print(f"{RULE_MISSING} {code}")
print()
print(f"{num_ok} implemented")
print(f"{num_missing} missing")
