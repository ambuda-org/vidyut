#!/usr/bin/env python3
from pathlib import Path
from collections import Counter
import re
import glob

RULE_OK = "✅"
RULE_UNTESTED = "⚠️ "
RULE_MISSING = "❌"

def print_legend():
    print("===== Legend ======")
    print(f"{RULE_OK}\t\tSutra is tested.")
    print(f"{RULE_UNTESTED}\t\tSutra is implemented but untested.")
    print(f"{RULE_MISSING}\t\tSutra is missing.")
    print()
    print("These statuses are heuristics. Verify them by checking the underlying code.")
    print("===================")

base = Path(__file__).parent.parent
src = base / "src"
tests = base / "tests"

all_rules = []
with open(base / "data/sutrapatha.tsv") as f:
    for line in f:
        code, text = line.split('\t')
        all_rules.append(code)


implemented_rules = set()
for path in glob.glob("**/*.rs", root_dir=src, recursive=True):
    with open(src / path) as f:
        for line in f:
            if m := re.search(r'"(\d+\.\d+\.\d+)', line):
                implemented_rules.add(m.group(1))

tested_rules = set()
for path in glob.glob("**/*.rs", root_dir=tests, recursive=True):
    with open(tests / path) as f:
        for line in f:
            for match in re.findall(r"(\d+_\d+_\d+)", line):
                tested_rules.add(match.replace('_', '.'))

had_ok = False
for rule in all_rules:
    if rule in tested_rules:
        if had_ok:
            continue
        print(f"{RULE_OK}\t\t[...]")
        had_ok = True
    elif rule in implemented_rules:
        print(f"{RULE_UNTESTED}\t\t{rule}")
        had_ok = False
    else:
        status = RULE_MISSING
        print(f"{RULE_MISSING}\t\t{rule}")
        had_ok = False

print_legend()

pada_total = Counter()
pada_written = Counter()
pada_tested = Counter()
pada_missing = Counter()
for rule in all_rules:
    ap, _, sutra = rule.rpartition('.')
    pada_total[ap] += 1
    if rule in tested_rules:
        pada_tested[ap] += 1
    elif rule in implemented_rules:
        pada_written[ap] += 1
    else:
        pada_missing[ap] += 1

print("Coverage by pada:")
print()
print(f"+---------+------------+------------+------------+------------+")
print(f"| Pada    |    Written |     Tested |    Missing |      Total |")
print(f"+---------+------------+------------+------------+------------+")
for key, total in pada_total.items():
    written = pada_written[key]
    tested = pada_tested[key]
    missing = pada_missing[key]
    print(f"| {key}     | {written:>10} | {tested:>10} | {missing:>10} | {total:>10} |")
written = pada_written.total()
total = pada_total.total()
tested = pada_tested.total()
missing = pada_missing.total()
print(f"+---------+------------+------------+------------+------------+")
print(f"| All     | {written:>10} | {tested:>10} | {missing:>10} | {total:>10} |")
print(f"+---------+------------+------------+------------+------------+")

print()
num_ok = total - pada_missing.total()
print("Num tested or implemented: {}".format(num_ok))
