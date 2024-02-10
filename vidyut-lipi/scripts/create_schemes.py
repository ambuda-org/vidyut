#!/usr/bin/env python3
"""Create schemes for vidyut-lipi and writes them to `src/autogen_schemes.rs`.

We create these mappings by modifying the data in the `common_maps` dir from
the indic-transliteration project.
"""

import tomllib
import subprocess
import unicodedata
from pathlib import Path
from glob import glob
import shutil

CRATE_DIR = Path(__file__).parent.parent

ALLOWED = {
    "AHOM",
    "ASSAMESE",
    "BALINESE",
    "BENGALI",
    "BHAIKSUKI",
    "BRAHMI",
    "BURMESE",
    "CHAM",
    "DEVANAGARI",
    "DOGRA",
    "GONDI_GUNJALA",
    "GONDI_MASARAM",
    "GUJARATI",
    "GURMUKHI",
    "GRANTHA",
    "JAVANESE",
    "KAITHI",
    "KANNADA",
    "KHAROSHTHI",
    "KHMER",
    "KHUDAWADI",
    "LAO",
    "LEPCHA",
    "LIMBU",
    "MALAYALAM",
    "MANIPURI",
    "MODI",
    "MON",
    "NANDINAGARI",
    "NEWA",
    "OL_CHIKI",
    "ORIYA",
    "SHARADA",
    "SIDDHAM",
    "SINHALA",
    "SAURASHTRA",
    "SOYOMBO",
    "TAKRI",
    "TAI_THAM",
    "TAMIL_SUPERSCRIPTED",
    "TELUGU",
    "THAI",
    "TIBETAN",
    "TIRHUTA_MAITHILI",
    "ZANBAZAR_SQUARE",
    "BARAHA",
    "HK",
    "IAST",
    "ISO",
    "ITRANS",
    "SLP1",
    "VELTHUIS",
    "WX",
}


KEY_NAMES = {
    "\u0905": "A",
    "\u0906": "AA",
    "\u0907": "I",
    "\u0908": "II",
    "\u0909": "U",
    "\u090a": "UU",
    "\u090b": "R",
    "\u0960": "RR",
    "\u090c": "L",
    "\u0961": "LL",
    "\u090d": "CANDRA_E",
    "\u090e": "E",
    "\u090f": "EE",
    "\u0910": "AI",
    "\u0911": "CANDRA_O",
    "\u0912": "O",
    "\u0913": "OO",
    "\u0914": "AU",
    "\u093e": "SIGN_AA",
    "\u093f": "SIGN_I",
    "\u0940": "SIGN_II",
    "\u0941": "SIGN_U",
    "\u0942": "SIGN_UU",
    "\u0943": "SIGN_R",
    "\u0944": "SIGN_RR",
    "\u0962": "SIGN_L",
    "\u0963": "SIGN_LL",
    "\u0945": "SIGN_CANDRA_E",
    "\u0946": "SIGN_E",
    "\u0947": "SIGN_EE",
    "\u0948": "SIGN_AI",
    "\u0949": "SIGN_CANDRA_O",
    "\u094a": "SIGN_O",
    "\u094b": "SIGN_OO",
    "\u094c": "SIGN_AU",
    "\u0902": "ANUSVARA",
    "\u0903": "VISARGA",
    "\u0901": "CANDRABINDU",
    "\u094d": "VIRAMA",
    "\u0915": "KA",
    "\u0916": "KHA",
    "\u0917": "GA",
    "\u0918": "GHA",
    "\u0919": "NGA",
    "\u091a": "CA",
    "\u091b": "CHA",
    "\u091c": "JA",
    "\u091d": "JHA",
    "\u091e": "NYA",
    "\u091f": "TTA",
    "\u0920": "TTHA",
    "\u0921": "DDA",
    "\u0922": "DDHA",
    "\u0923": "NNA",
    "\u0924": "TA",
    "\u0925": "THA",
    "\u0926": "DA",
    "\u0927": "DHA",
    "\u0928": "NA",
    "\u092a": "PA",
    "\u092b": "PHA",
    "\u092c": "BA",
    "\u092d": "BHA",
    "\u092e": "MA",
    "\u092f": "YA",
    "\u0930": "RA",
    "\u0932": "LA",
    "\u0935": "VA",
    "\u0936": "SHA",
    "\u0937": "SSA",
    "\u0938": "SA",
    "\u0939": "HA",
    "\u0933": "LLA",
    "\u0931": "RRA",
    "\u0929": "NNNA",
    "\u0934": "LLLA",
    "क्ष": "KSSA",
    "ज्ञ": "JNYA",
    "त्र": "TRA",
    "\u0915\u093c": "QA",
    "\u0916\u093c": "KHHA",
    "\u0917\u093c": "GHHA",
    "\u091c\u093c": "ZA",
    "\u0921\u093c": "DDDHA",
    "\u0922\u093c": "RHA",
    "\u092b\u093c": "FA",
    "\u092f\u093c": "YYA",
    "\u0951": "SVARITA",
    "\u1cda": "DOUBLE_SVARITA",
    "\u0952": "ANUDATTA",
    "\u0966": "DIGIT_0",
    "\u0967": "DIGIT_1",
    "\u0968": "DIGIT_2",
    "\u0969": "DIGIT_3",
    "\u096a": "DIGIT_4",
    "\u096b": "DIGIT_5",
    "\u096c": "DIGIT_6",
    "\u096d": "DIGIT_7",
    "\u096e": "DIGIT_8",
    "\u096f": "DIGIT_9",
    "\u0950": "OM",
    "\u093c": "NUKTA",
    "\u093d": "AVAGRAHA",
    "\u0964": "DANDA",
    "\u0965": "DOUBLE_DANDA",
    "\u0970": "ABBREVIATION_SIGN",
    "\u0971": "HIGH_SPACING_DOT",
    "\u1cd2": "PRENKHA",
    "\u1cdd": "VEDIC_DOT_BELOW",
    "\u1ce1": "ATHARVAVEDA_INDEPENDENT_SVARITA",
    "\u1cf5": "JIHVAMULIYA",
    "\u1cf6": "UPADHMANIYA",
    "\u1cf8": "VEDIC_RING_ABOVE",
    "\ua8e0": "COMBINING_DIGIT_0",
    "\ua8e1": "COMBINING_DIGIT_1",
    "\ua8e2": "COMBINING_DIGIT_2",
    "\ua8e3": "COMBINING_DIGIT_3",
    "\ua8e4": "COMBINING_DIGIT_4",
    "\ua8e5": "COMBINING_DIGIT_5",
    "\ua8e6": "COMBINING_DIGIT_6",
    "\ua8e7": "COMBINING_DIGIT_7",
    "\ua8e8": "COMBINING_DIGIT_8",
    "\ua8e9": "COMBINING_DIGIT_9",
    "\ua8ea": "COMBINING_A",
    "\ua8eb": "COMBINING_U",
    "\ua8ec": "COMBINING_KA",
    "\ua8ed": "COMBINING_NA",
    "\ua8ee": "COMBINING_PA",
    "\ua8ef": "COMBINING_RA",
    "\ua8f0": "COMBINING_VI",
    "\ua8f1": "COMBINING_AVAGRAHA",
    "\ua8f3": "CANDRABINDU_VIRAMA",
    "\u0b83": "TAMIL_AYTHAM",
    "\u200c": "ZERO_WIDTH_NON_JOINER",
    "\u200d": "ZERO_WIDTH_JOINER",
}

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


JIHVAMULIYA = "\u1cf5"
UPADHMANIYA = "\u1cf6"
DANDA = "\u0964"
DOUBLE_DANDA = "\u0965"


OVERRIDES = {
    "BARAHA":
    # Existing accent marks seem to be mostly wrong -- delete so that we
    # can redefine them elsewhere.
    {
        "\u1ce1": None,
        "\ua8e1": None,
        "\ua8e2": None,
        "\ua8e3": None,
    },
    "GRANTHA": {
        # vowel sign AU
        "\u094c": "\U0001134c",
    },
    "GONDI_GUNJALA": {
        # No avagraha defined -- for now, use Devanagari avagraha as placeholder.
        "\u093d": "\u093d",
    },
    "GONDI_MASARAM": {
        # Virama
        "\u094d": "\U00011d45",
        # No avagraha defined -- for now, use Devanagari avagraha as placeholder.
        "\u093d": "\u093d",
        # Conjuncts
        "क्ष": "𑴮",
        "ज्ञ": "𑴯",
    },
    "GURMUKHI": {
        "\u090b": "ਰੁ",  # letter vocalic r
        "\u0960": "ਰੂ",  # letter vocalic rr
        "\u090c": "ਲੁ",  # letter vocalic l
        "\u0961": "ਲੂ",  # letter vocalic ll
        "\u0943": "\u0a4dਰੁ",  # sign vocalic r
        "\u0944": "\u0a4dਰੂ",  # sign vocalic rr
        "\u0962": "\u0a4dਲੁ",  # sign vocalic l
        "\u0963": "\u0a4dਲੂ",  # sign vocalic ll
        "\u090e": "ਏ",  # letter short e
        "\u0912": "ਓ",  # letter short o
        "\u0946": "\u0a47",  # sign short e
        "\u094a": "\u0a4b",  # sign short o
    },
    "HK": {
        DANDA: ".",
        DOUBLE_DANDA: "..",
    },
    "ISO": {
        "।": ".",
        "॥": "..",
        "ख़": "k͟h",
        # Delete -- common_maps maps this to "ḳ", which we need for aytam.
        # We'll add a valid mapping for क़: further below.
        "क़": None,
        # Delete -- mistake, corrected below.
        "ḫ": None,
    },
    "IAST": {
        "ळ": "ḻ",
        "ऴ": None,
        "।": ".",
        "॥": "..",
        # candrabindu
        "\u0901": "m̐",
    },
    "JAVANESE": {
        DANDA: "\ua9c8",
        DOUBLE_DANDA: "\ua9c9",
    },
    "KHAROSHTHI": {
        "।": "\U00010a56",
        "॥": "\U00010a57",
    },
    "KHMER": {
        "।": "។",
        "॥": "៕",
    },
    "KHUDAWADI": {
        "\u090e": "\U000112b6",  # letter short e
        "\u0912": "\U000112b8",  # letter short o
        "\u0946": "\U000112e5",  # sign short e
        "\u094a": "\U000112e7",  # sign short o
    },
    "LIMBU": {
        "\u090a": "\u1900\u1922\u193a",  # letter uu
        "\u0960": "\u1916\u1922\u193a",  # letter vocalic rr
        "\u0961": "\u1917\u1922\u193a",  # letter vocalic ll
        "\u0942": "\u1922\u193a",  # sign uu
        "\u0943": "\u193b\u1916\u1922",  # sign vocalic r
        "\u0944": "\u193b\u1916\u1922\u193a",  # sign vocalic rr
        "\u0962": "\u193b\u1917\u1922",  # sign vocalic l
        "\u0963": "\u193b\u1917\u1922\u193a",  # sign vocalic ll
        "\u090e": "\u1900\u1927",  # letter short e
        "\u0912": "\u1900\u1928",  # letter short o
        "\u0946": "\u1927",  # sign short e
        "\u094a": "\u1928",  # sign short o
    },
    "MANIPURI": {
        DANDA: "꯫",
        DOUBLE_DANDA: "꯫꯫",
    },
    "MALAYALAM": {
        # sign short o
        "\u094a": "\u0d4a",
        # sign au
        "\u094c": "\u0d57",
    },
    "MODI": {
        "\u0907": "\U00011602",  # letter i
        "\u0908": "\U00011603",  # letter ii
        "\u0909": "\U00011604",  # letter u
        "\u090a": "\U00011605",  # letter uu
        "\u090b": "\U00011606",  # letter vocalic r
        "\u090c": "\U00011608",  # letter vocalic l
        "\u093f": "\U00011631",  # sign i
        "\u0940": "\U00011632",  # sign ii
        "\u0941": "\U00011633",  # sign u
        "\u0942": "\U00011634",  # sign uu
        "\u0943": "\U00011635",  # sign vocalic r
        "\u0944": "\U00011636",  # sign vocalic rr
        "\u0960": "\U00011607",  # letter vocalic rr
        "\u0961": "\U00011609",  # letter vocalic ll
        "\u0962": "\U00011637",  # sign vocalic l
        "\u0963": "\U00011638",  # sign vocalic ll
        DANDA: "\U00011641",
        DOUBLE_DANDA: "\U00011642",
    },
    "MON": {
        DANDA: "\u104a",
        DOUBLE_DANDA: "\u104b",
    },
    "NEWA": {
        DANDA: "\U0001144b",
        DOUBLE_DANDA: "\U0001144c",
    },
    "TAKRI": {
        "ख": "𑚸",
    },
    "TAMIL_SUPERSCRIPTED":
    # Use roman digits per Aksharamukha
    {
        "०": "0",
        "१": "1",
        "२": "2",
        "३": "3",
        "४": "4",
        "५": "5",
        "६": "6",
        "७": "7",
        "८": "8",
        "९": "9",
    },
    "TIBETAN": {
        # Virama
        "\u094d": "\u0f84",
        # Use distinct "va" character instead of "ba".
        "व": "\u0f5d",
    },
    "VELTHUIS":
    # These are part of the Velthuis spec but are errors in indic-transliteration.
    {
        "ॠ": ".R",
        "ॡ": ".L",
        # Should be .o, per spec
        "ॐ": ".o",
    },
    "WX": {
        "ऎ": "eV",
        "ऒ": "oV",
        "ॡ": "LV",
        "ळ": "lY",
        "ऽ": "Z",
    },
    "ZANBAZAR_SQUARE": {
        "\u0943": "\U00011A34\U00011A2B\U00011A09",  # sign vocalic r
        "\u0944": "\U00011A34\U00011A2B\U00011A09\U00011A0A",  # sign vocalic rr
        "\u0962": "\U00011A34\U00011A2C\U00011A09",  # sign vocalic l
        "\u0963": "\U00011A34\U00011A2C\U00011A09\U00011A0A",  # sign vocalic ll
    },
}


EXTENSIONS = {
    "BARAHA": [
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
    ],
    "BRAHMI": [
        (JIHVAMULIYA, "\U00011003"),
        (UPADHMANIYA, "\U00011004"),
    ],
    "DEVANAGARI": [
        # DEVANAGARI VOWEL SIGN PRISHTHAMATRA E (U+094E)
        # See comments on U+094E for details.
        ("\u0948", "\u0947\u094e"),
        ("\u094b", "\u093e\u094e"),
        ("\u094c", "\u094b\u094e"),
        # Vedic accents
        ("\u1cd2", "\u1cd2"),
        ("\u1cda", "\u1cda"),
        ("\u1cdd", "\u1cdd"),
        ("\ua8e0", "\ua8e0"),
        # Punctuation
        ("\u0970", "\u0970"),
        ("\u0971", "\u0971"),
    ],
    "GONDI_GUNJALA": [
        ("\u094d", "\U00011D97"),
    ],
    "GONDI_MASARAM": [
        ("त्र", "𑴰"),
    ],
    "GRANTHA": [
        # OO (EE + AA length mark)
        ("\u094b", "\U00011347\U0001133e"),
        # AU length mark
        ("\u094c", "\U00011357"),
        # AU (AA + AU length mark)
        ("\u094c", "\U00011347\U00011357"),
        # Vedic accents
        ("\ua8e0", "\U00011366"),
        ("\ua8e1", "\U00011367"),
        ("\ua8e2", "\U00011368"),
        ("\ua8e3", "\U00011369"),
        ("\ua8e4", "\U0001136a"),
        ("\ua8e5", "\U0001136b"),
        ("\ua8e6", "\U0001136c"),
        # -- 3 reserved chars --
        ("\ua8ea", "\U00011370"),  # a
        ("\ua8ec", "\U00011371"),  # ka
        ("\ua8ed", "\U00011372"),  # na
        ("\ua8e6", "\U00011374"),  # pa
        ("\ua8f0", "\U00011373"),  # vi
    ],
    "ITRANS": [
        # Vedic anusvara (just render as candrabindu)
        ("\u0901", "{\\m+}"),
        ("\u1cda", "\\\""),
    ],
    "ISO": [
        # Aytam
        ("\u0b83", "ḳ"),
        (JIHVAMULIYA, "ẖ"),
        (UPADHMANIYA, "ḫ"),
    ],
    "KANNADA": [
        (JIHVAMULIYA, "\u0cf1"),
        (UPADHMANIYA, "\u0cf2"),
    ],
    "MALAYALAM": [
        # AU archaic mark
        ("\u094c", "\u0d4c"),
    ],
    "NEWA": [
        (JIHVAMULIYA, "\U00011460"),
        (UPADHMANIYA, "\U00011461"),
    ],
    "SHARADA": [
        (JIHVAMULIYA, "\U000111c2"),
        (UPADHMANIYA, "\U000111c3"),
    ],
    "SINHALA":
    # Sinhala chandrabindu is not supported in the fonts I tried, so
    # use anusvara instead.
    [("\u0901", "\u0d82")],
    "SLP1": [
        (JIHVAMULIYA, "Z"),
        (UPADHMANIYA, "V"),
        # Lha
        ("ळ्ह", "|"),
        # Svarita
        ("\u0951", "^"),
        # Anudatta
        ("\u0952", "\\"),
    ],
    "SOYOMBO": [
        (JIHVAMULIYA, "\U00011a84"),
        (UPADHMANIYA, "\U00011a85"),
        (DANDA, "\U00011a9b"),
        (DOUBLE_DANDA, "\U00011a9c"),
    ],
    "TAMIL_SUPERSCRIPTED": [
        # Aytam
        ("\u0b83", "\u0b83"),
    ],
    "TIBETAN": [
        (JIHVAMULIYA, "\u0f88"),
        (UPADHMANIYA, "\u0f89"),
    ],
    "VELTHUIS": [
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
        ("\u0915\u093c", "q"),
        ("\u0916\u093c", ".kh"),
        ("\u0957\u093c", ".g"),
        ("\u091c\u093c", "z"),
        ("\u0921\u093c", "R"),
        ("\u0922\u093c", "Rh"),
        ("\u092b\u093c", "f"),
    ],
}


def _sanitize(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


def _to_deva_nfd(s: str) -> str:
    overrides = {
        "\u0958": "\u0915\u093c",  # ka
        "\u0959": "\u0916\u093c",  # kha
        "\u095a": "\u0917\u093c",  # ga
        "\u095b": "\u091c\u093c",  # ja
        "\u095c": "\u0921\u093c",  # Da
        "\u095d": "\u0922\u093c",  # Dha
        "\u095e": "\u092b\u093c",  # pha
        "\u095f": "\u092f\u093c",  # ya
    }
    return overrides.get(s, s)


def to_unique(xs: list) -> list:
    seen = set()
    ret = []
    for x in xs:
        if x not in seen:
            ret.append(x)
            seen.add(x)
    return ret


def _maybe_override(name: str, deva: str, raw: str) -> str | None:
    return OVERRIDES.get(name, {}).get(deva, raw)


def _reorder_short_vowels(items: list) -> list:
    # Rank show vowel items lower than long vowel items so that we prefer long vowels when reversing.
    short_vowel_codes = {"\u0946", "\u094a", "\u090e", "\u0912"}
    no_short_e_o = [x for x in items if x[0] not in short_vowel_codes]
    short_e_o = [x for x in items if x[0] in short_vowel_codes]
    return no_short_e_o + short_e_o


def _reorder_kharoshthi_n(items: list) -> list:
    nga = {"\u0919"}
    no_nga = [x for x in items if x[0] not in nga]
    nga = [x for x in items if x[0] in nga]
    return no_nga + nga


def _ol_chiki_consonants(items: list) -> list:
    new = []
    for x, y in items:
        if len(y) > 1 and y.endswith("ᱚ"):
            new.append((x, y[:-1]))
        else:
            new.append((x, y))

    return new


def create_key_consts() -> str:
    buf = []
    for deva, name in KEY_NAMES.items():
        buf.append(f'const {name}: &str = "{deva}";')
    return "\n".join(buf)


def create_scheme_entry(name: str, items: list[tuple[str, str]]) -> str:
    buf = []
    seen = set()

    buf.append(f"pub const {name}: &[(&str, &str)] = &[")
    for deva, raw in items:
        deva = unicodedata.normalize("NFC", _sanitize(deva))
        raw = unicodedata.normalize("NFC", _sanitize(raw))

        if (deva, raw) in seen:
            continue
        seen.add((deva, raw))

        if deva in KEY_NAMES:
            key_name = KEY_NAMES[deva]
            buf.append(f'    ({key_name}, "{raw}"),')
        else:
            buf.append(f'    ("{deva}", "{raw}"),')
    buf.append("];\n")

    return "\n".join(buf)


def main():
    # We're waiting on some changes to be pushed to indic-transliteration, so
    # use a fork for now.
    repo = "https://github.com/akprasad/common_maps.git"
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
        create_key_consts(),
        "",
    ]

    BRAHMIC_WITH_DEVA_ACCENTS = {
        "ASSAMESE",
        "BENGALI",
        "GUJARATI",
        "KANNADA",
        "MALAYALAM",
        "ORIYA",
        "TELUGU",
        "SHARADA",
    }

    for path in sorted(glob("/Users/arun/temp/common_maps/**/*.toml")):
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

            if category == "accents":
                if scheme_name in BRAHMIC_WITH_DEVA_ACCENTS or scheme_name == "GRANTHA":
                    continue

            if category.endswith("alternates"):
                for raw_main, alts in data[category].items():
                    deva = raw_to_deva.get(raw_main)
                    if deva is None:
                        continue

                    deva = unicodedata.normalize("NFC", _sanitize(deva))
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

        scheme_items = [(_to_deva_nfd(x), _to_deva_nfd(y)) for (x, y) in scheme_items]
        scheme_items = to_unique(scheme_items)

        # Add svarita and anudatta for Brahmic scripts that use Devanagari accent marks.
        if scheme_name in BRAHMIC_WITH_DEVA_ACCENTS:
            scheme_items.extend(
                [
                    # Svarita
                    ("\u0951", "\u0951"),
                    # Anudatta
                    ("\u0952", "\u0952"),
                    # Dirgha svarita
                    ("\u1cda", "\u1cda"),
                ]
            )
        elif scheme_name == "GRANTHA":
            scheme_items.extend(
                [
                    # Svarita (use chandra symbol)
                    ("\u0951", "\u1cf4"),
                    # Dirgha svarita (use Devanagari svarita)
                    ("\u1cda", "\u0951"),
                    # Anudatta (use Devanagari)
                    ("\u0952", "\u0952"),
                ]
            )

        scheme_items.extend(EXTENSIONS.get(scheme_name, []))

        renames = {
            "GONDI_GUNJALA": "GUNJALA_GONDI",
            "GONDI_MASARAM": "MASARAM_GONDI",
            "ISO": "ISO_15919",
            "MANIPURI": "MEETEI_MAYEK",
            "ZANBAZAR_SQUARE": "ZANABAZAR_SQUARE",
            "TAMIL_SUPERSCRIPTED": "TAMIL",
            "TIRHUTA_MAITHILI": "TIRHUTA",
        }

        scheme_name = renames.get(scheme_name, scheme_name)

        scheme_items = _reorder_short_vowels(scheme_items)
        if scheme_name == "OL_CHIKI":
            scheme_items = _ol_chiki_consonants(scheme_items)
        elif scheme_name == "KHAROSHTHI":
            scheme_items = _reorder_kharoshthi_n(scheme_items)
        buf.append(create_scheme_entry(scheme_name, scheme_items))

    with open(CRATE_DIR / "src/autogen_schemes.rs", "w") as f:
        f.write("\n".join(buf))

    print("Cleaning up ...")
    shutil.rmtree(common_maps)

    print("Done.")


if __name__ == "__main__":
    main()
