#!/usr/bin/env python3
"""Create schemes for vidyut-lipi and writes them to `src/autogen_schemes.rs`.

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
    "BALINESE",
    "BENGALI",
    "BRAHMI",
    "BURMESE",
    "DEVANAGARI",
    "GUJARATI",
    "GURMUKHI",
    "GRANTHA",
    "JAVANESE",
    "KANNADA",
    "MALAYALAM",
    "ORIYA",
    "SHARADA",
    "SIDDHAM",
    "SINHALA",
    "TAMIL",
    "TELUGU",
    "TIBETAN",

    "BARAHA",
    "HK",
    "IAST",
    "ISO",
    "ITRANS",
    "SLP1",
    "VELTHUIS",
    "WX",
}


def _sanitize(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


def _maybe_override(name: str, deva: str, raw: str) -> str | None:
    overrides = {}

    if name in {"BRAHMI", "BALINESE", "BURMESE", "SIDDHAM", "TIBETAN"}:
        if deva in {"\u0946", "\u094a", "\u090e", "\u0912"}:
            # - short e mark
            # - short o mark
            # - short e vowel
            # - short o vowel
            # TODO: keep short vowels and reorder them after long vowels
            return None
    elif name == "BARAHA":
        # Existing accent marks seem to be mostly wrong -- delete so that we
        # can redefine them elsewhere.
        overrides = {
            "\u1ce1": None,
            "\ua8e1": None,
            "\ua8e2": None,
            "\ua8e3": None,
        }
    elif name == "GRANTHA":
        # vowel sign AU
        overrides = {
            "\u094c": "\U0001134c",
        }
    elif name == "HK":
        if raw == "|":
            return "."
        if raw == "||":
            return ".."
    elif name == "ISO":
        overrides = {
            "।": ".",
            "॥": "..",
        }
    elif name == "IAST":
        overrides = {
            "ळ": "ḻ",
            "ऴ": None,
            "।": ".",
            "॥": "..",
            # candrabindu
            "\u0901": "m̐",
        }
    elif name == "VELTHUIS":
        # These are part of the Velthuis spec but are errors in indic-transliteration.
        overrides = {
            # Short e and o, plus vowel marks
            "\u0946": None,
            "\u094a": None,
            "\u090e": None,
            "\u0912": None,
            "ॠ": ".R",
            "ॡ": ".L",
            # Should be .o, per spec
            "ॐ": ".o",
        }
    elif name == "WX":
        overrides = {
            "ऎ": "eV",
            "ऒ": "oV",
            "ॡ": "LV",
            "ळ": "lY",
            "ऽ": "Z",
        }

    return overrides.get(deva, raw)


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
                        alt = _maybe_override(scheme_name, deva, alt)
                        if alt is not None:
                            scheme_items.append((deva, alt))
                    mark = VOWEL_TO_MARK.get(deva)
                    if mark:
                        assert isinstance(mark, str)
                        for alt in alts:
                            alt = _maybe_override(scheme_name, mark, alt)
                            if alt is not None:
                                scheme_items.append((mark, alt))
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
                        if raw is None:
                            continue
                        mark = VOWEL_TO_MARK.get(vowel)
                        if mark:
                            assert isinstance(mark, str)
                            assert isinstance(raw, str)
                            scheme_items.append((mark, raw))

        if scheme_name == "BARAHA":
            scheme_items.extend([
                ("\u0914", "ou"),
                ("\u094c", "ou"),
                ("\u0939", "~h"),
                # Corrected accent marks:
                # - Horizontal line above ()
                ("\u1cd2", "Q"),
                # - Dot below
                ("\u1cdd", "V"),
                # - TODO: Dot above (can't find the right Unicode for it)
                # TODO: ("", "W"),
                # - Double vertical line above (double svarita)
                ("\u1cda", "$"),
            ])
        elif scheme_name == "DEVANAGARI":
            scheme_items.extend([
                # DEVANAGARI VOWEL SIGN PRISHTHAMATRA E (U+094E)
                # See comments on U+094E for details.
                ("\u0948", "\u0947\u094e"),
                ("\u094b", "\u093e\u094e"),
                ("\u094c", "\u094b\u094e"),
                # Consonants with nuqtas.
                ("\u0931", "\u0931"),
                ("\u0958", "\u0958"),
                ("\u0959", "\u0959"),
                ("\u095a", "\u095a"),
                ("\u095b", "\u095b"),
                ("\u095c", "\u095c"),
                ("\u095d", "\u095d"),
                ("\u095e", "\u095e"),
                ("\u095f", "\u095f"),
                # Vedic accents
                ("\u1cd2", "\u1cd2"),
                ("\u1cda", "\u1cda"),
                ("\u1cdd", "\u1cdd"),
                # Punctuation
                ("\u0970", "\u0970"),
                ("\u0971", "\u0971"),
            ])
        elif scheme_name == "GRANTHA":
            scheme_items.extend([
                # OO (EE + AA length mark)
                ("\u094b", "\U00011347\U0001133e"),
                # AU length mark
                ("\u094c", "\U00011357"),
                # AU (AA + AU length mark)
                ("\u094c", "\U00011347\U00011357"),
            ])
        elif scheme_name == "SINHALA":
            # Sinhala chandrabindu is not supported in the fonts I tried, so
            # use anusvara instead.
            scheme_items.append(("\u0901", "\u0d82"))
        elif scheme_name == "SLP1":
            scheme_items.extend([
                # Jihvamuliya
                ("\u1cf5", "Z"),
                # Upadhmaniya
                ("\u1cf6", "V"),
                # Lha
                ("ळ्ह", "|"),
                # Svarita
                ("\u0951", "^"),
                # Anudatta
                ("\u0952", "\\"),
            ])
        elif scheme_name == "VELTHUIS":
            scheme_items.extend([
                # Virama
                ("\u094d", "&"),
                # Chandrabindu variant
                ("\u0901", "/"),
                ("\u0945", "~a"),
                ("\u0949", "~o"),
                # Punctuation
                ("\u0970", "@"),
                ("\u0971", "#"),
                # Consonants with nuqtas
                ("\u0931", "^r"),
                ("\u0958", "q"),
                ("\u0959", ".kh"),
                ("\u095a", ".g"),
                ("\u095b", "z"),
                ("\u095c", "R"),
                ("\u095d", "Rh"),
                ("\u095e", "f"),
            ])

        buf.append(create_scheme_str(scheme_name, scheme_items))

    with open(CRATE_DIR / "src/autogen_schemes.rs", "w") as f:
        f.write("\n".join(buf))

    print("Cleaning up ...")
    shutil.rmtree(common_maps)

    print("Done.")


if __name__ == "__main__":
    main()
