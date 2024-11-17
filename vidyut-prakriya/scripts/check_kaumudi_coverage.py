#!/usr/bin/env python3
from pathlib import Path
from collections import Counter
import re
import glob

RULE_OK = "✅"
RULE_UNTESTED = "⚠️ "
RULE_MISSING = "❌"

base = Path(__file__).parent.parent
src = base / "src"
tests = base / "tests"

tested_rules = set()
for path in glob.glob("**/kaumudi*.rs", root_dir=tests, recursive=True):
    with open(tests / path) as f:
        for line in f:
            if not line.startswith("fn "):
                continue

            matches = list(re.findall(r"_(\d+)", line))
            if matches:
                for match in range(int(matches[0]), int(matches[-1]) + 1):
                    tested_rules.add(int(match))

for i in range(178, 2679):
    if i > 446 and i < 2151:
        continue

    if i in tested_rules:
        pass # print(f"{RULE_OK} {i}")
    else:
        print(f"{RULE_MISSING} {i}")
