#!/usr/bin/env python3
"""Create schemes for vidyut-lipi and writes them to `src/schemes.rs`.

We create these mappings by modifying the data in the `common_maps` dir from
the indic-transliteration project.
"""

import tomllib
import subprocess
from pathlib import Path
from glob import glob
import shutil

CRATE_DIR = Path(__file__).parent.parent

VOWEL_TO_MARK = {
    "आ": "\u093e",
    "इ": "\u093f",
    "ई": "\u0940",
    "उ": "\u0941",
    "ऊ": "\u0942",
    "ऋ": "\u0943",
    "ॠ": "\u0944",
    "ऌ": "\u0962",
    "ॡ": "\u0963",
    "ऎ": "\u0946",
    "ए": "\u0947",
    "ऐ": "\u0948",
    "ऒ": "\u094a",
    "ओ": "\u094b",
    "औ": "\u094c",
}

ALLOWED = {
    "BENGALI",
    "BRAHMI",
    "DEVANAGARI",
    "GUJARATI",
    "GURMUKHI",
    "GRANTHA",
    "KANNADA",
    "MALAYALAM",
    "ORIYA",
    "SINHALA",
    "TAMIL",
    "TELUGU",
    "TIBETAN",

    "HK",
    "IAST",
    "ITRANS",
    "SLP1",
    "VELTHUIS",
}


def _sanitize(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


def _maybe_override(name: str, deva: str, raw: str) -> str | None:
    if name == "BRAHMI":
        if deva == "\u0946":
            # short e mark
            return None
        if deva == "\u094a":
            # short o mark
            return None
    elif name == "HK":
        if raw == "|":
            return "."
        if raw == "||":
            return ".."
    elif name == "IAST":
        if deva == "ळ":
            return "ḻ"
        if deva == "ऴ":
            return None
        if raw == "|":
            return "."
        if raw == "||":
            return ".."
    elif name == "VELTHUIS":
        # These are part of the Velthuis spec but are errors in indic-transliteration.
        if deva == "ॠ":
            return ".R"
        if deva == "ॡ":
            return ".L"
    return raw


def create_scheme_str(name: str, items: list[tuple[str, str]]) -> str:
    buf = []

    buf.append(f"pub const {name}: &[(&str, &str)] = &[")
    for deva, raw in items:
        deva = _sanitize(deva)
        raw = _sanitize(raw)
        buf.append(f'    ("{deva}", "{raw}"),')
    buf.append("];\n")

    return "\n".join(buf)


def main():
    repo = "https://github.com/indic-transliteration/common_maps.git"
    common_maps = Path("common_maps")
    if not common_maps.exists():
        print("Cloning `common_maps` ...")
        subprocess.run(f"git clone --depth 1 {repo}", shell=True)

    print("Creating schemes ...")
    buf = [
        "#![allow(unused)]",
        "",
        "//! Auto-generated scheme data.",
        "//!",
        "//! These schemes were auto-generated from the `common_maps` repository",
        "//! from the `indic-transliteration` project.",
        "",
    ]
    for path in sorted(glob("common_maps/**/*.toml")):
        with open(path, "rb") as f:
            data = tomllib.load(f)

        scheme_name = Path(path).stem.upper()
        if scheme_name not in ALLOWED:
            continue

        scheme_type = Path(path).parent.stem
        assert scheme_type in {"roman", "brahmic"}, scheme_type

        scheme_items = []
        raw_to_deva = {}

        for category in data:
            if category.startswith("_"):
                # Ignore file comments, etc.
                continue

            if category == "shortcuts":
                # TODO: support these
                continue

            if category.endswith("alternates"):
                for raw_main, alts in data[category].items():
                    deva = raw_to_deva.get(raw_main)
                    if deva is None:
                        continue
                    for alt in alts:
                        assert isinstance(deva, str)
                        assert isinstance(alt, str)
                        alt = _maybe_override(scheme_name, deva, alt)
                        if alt is not None:
                            scheme_items.append((deva, alt))
            else:
                for deva, raw in data[category].items():
                    assert isinstance(deva, str)
                    assert isinstance(raw, str)
                    raw = _maybe_override(scheme_name, deva, raw)
                    if raw is not None:
                        raw_to_deva[raw] = deva
                        scheme_items.append((deva, raw))

                if scheme_type == "roman" and category == "vowels":
                    for vowel, raw in data[category].items():
                        raw = _maybe_override(scheme_name, vowel, raw)
                        mark = VOWEL_TO_MARK.get(vowel)
                        if mark:
                            assert isinstance(mark, str)
                            assert isinstance(raw, str)
                            scheme_items.append((mark, raw))

        buf.append(create_scheme_str(scheme_name, scheme_items))

    with open(CRATE_DIR / "src/schemes.rs", "w") as f:
        f.write("\n".join(buf))

    print("Cleaning up ...")
    shutil.rmtree(common_maps)

    print("Done.")


if __name__ == "__main__":
    main()
