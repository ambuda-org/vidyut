import glob
from pathlib import Path

def make_mapping() -> dict:
    entries = {}
    with open("data/dhatupatha.tsv") as f:
        for i, line in enumerate(f):
            if i == 0:
                continue
            try:
                code, au, meaning = line.strip().split('\t')
            except ValueError:
                continue

            au_clean = au.replace("\\", "").replace("^", "").replace("~", "")
            au = au.replace("\\", "\\\\")
            if au_clean not in entries:
                entries[au_clean] = {au}
            else:
                entries[au_clean].add(au)

    mapping = {}
    for raw_key, vs in sorted(entries.items()):
        if len(vs) > 1:
            for i, value in enumerate(sorted(vs)):
                num_key = f"{raw_key}_{i+1}"
                mapping[num_key] = value
        else:
            for i, value in enumerate(vs):
                mapping[raw_key] = value

    assert 'dhatu' not in mapping
    return mapping


def find_and_replace_files(mapping):
    mapping = {k: v for k, v in mapping.items()
               if v in {"guhU~^", "du\\\\za~", "vancu~", "hi\\\\", "vida~", "o~pyAyI~\\\\"}}
    used = {}
    for path in glob.glob("src/**/*.rs", recursive=True):
        content = Path(path).read_text()
        for k, v in mapping.items():
            new_content = content.replace(f'"{v}"', f"Au::{k}")
            if new_content != content:
                used[k] = v
            content = new_content
        Path(path).write_text(content)
    for k, v in used.items():
        print(f'{k} => "{v}"')


def find_and_replace_text(content):
    # content = content.replace("\\", "\\\\")
    used = {}
    for k, v in mapping.items():
        new_content = content.replace(f'"{v}"', f"Au::{k}")
        if new_content != content:
            used[k] = v
        content = new_content
    print(content)
    for k, v in used.items():
        print(f'{k} => "{v}",')


import sys
mapping = make_mapping()
find_and_replace_files(mapping)
