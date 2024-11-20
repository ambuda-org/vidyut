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

# Scripts to use from `common_maps.git`
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


# Human-readable names for Unicode combos
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
    "\u1cda": "DIRGHA_SVARITA",
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
    "\ua8f4": "DOUBLE_CANDRABINDU_VIRAMA",
    "\u0b83": "TAMIL_AYTHAM",
    "\u200c": "ZERO_WIDTH_NON_JOINER",
    "\u200d": "ZERO_WIDTH_JOINER",
}

class AttributeDict(dict):
    def __init__(self, d):
        dict.__init__(self, **d)
        for k, v in d.items():
            setattr(self, k, v)

# Mapping from names to Unicode sequences
C = AttributeDict({v: k for k, v in KEY_NAMES.items()})


VOWEL_TO_MARK = {
    "आ": C.SIGN_AA,
    "इ": C.SIGN_I,
    "ई": C.SIGN_II,
    "उ": C.SIGN_U,
    "ऊ": C.SIGN_UU,
    "ऋ": C.SIGN_R,
    "ॠ": C.SIGN_RR,
    "ऌ": C.SIGN_L,
    "ॡ": C.SIGN_LL,
    "ऎ": C.SIGN_E,
    "ए": C.SIGN_EE,
    "ऐ": C.SIGN_AI,
    "ऒ": C.SIGN_O,
    "ओ": C.SIGN_OO,
    "औ": C.SIGN_AU,
}


# Tweaks to the defaults in common_map
OVERRIDES = {
    "BARAHA":
    # Existing accent marks seem to be mostly wrong -- delete so that we
    # can redefine them elsewhere.
    {
        "\u1ce1": None,
        C.COMBINING_DIGIT_1: None,
        C.COMBINING_DIGIT_2: None,
        C.COMBINING_DIGIT_3: None,
    },
    "GRANTHA": {
        # vowel sign AU
        C.SIGN_AU: "\U0001134c",
    },
    "GONDI_GUNJALA": {
        # No avagraha defined -- for now, use Devanagari avagraha as placeholder.
        C.AVAGRAHA: "\u093d",
        C.VIRAMA: "\U00011d97",
    },
    "GONDI_MASARAM": {
        # Virama
        C.VIRAMA: "\U00011d45",
        # No avagraha defined -- for now, use Devanagari avagraha as placeholder.
        "\u093d": "\u093d",
        # Conjuncts
        "क्ष": "𑴮",
        "ज्ञ": "𑴯",
    },
    "GURMUKHI": {
        C.R: "ਰੁ",
        C.RR: "ਰੂ",
        C.L: "ਲੁ",
        C.LL: "ਲੂ",
        C.SIGN_R: "\u0a4dਰੁ",
        C.SIGN_RR: "\u0a4dਰੂ",
        C.SIGN_L: "\u0a4dਲੁ",
        C.SIGN_LL: "\u0a4dਲੂ",
        C.E: "ਏ",  # letter short e
        C.O: "ਓ",  # letter short o
        C.SIGN_E: "\u0a47",  # sign short e
        C.SIGN_O: "\u0a4b",  # sign short o
        C.RHA: "\u0a22\u0a3c",
    },
    "HK": {
        C.DANDA: ".",
        C.DOUBLE_DANDA: "..",
    },
    "ISO": {
        C.DANDA: ".",
        C.DOUBLE_DANDA: "..",
        C.KHHA: "k͟h",
        # Delete -- common_maps maps this to "ḳ", which we need for aytam.
        # We'll add a valid mapping for क़: further below.
        C.QA: None,
        # Delete -- mistake, corrected below.
        "ḫ": None,
    },
    "IAST": {
        "ळ": "ḻ",
        "ऴ": None,
        C.DANDA: ".",
        C.DOUBLE_DANDA: "..",
        C.CANDRABINDU: "m̐",
    },
    "JAVANESE": {
        C.DANDA: "\ua9c8",
        C.DOUBLE_DANDA: "\ua9c9",
    },
    "KAITHI": {
        C.QA: "𑂍\U000110ba",
        C.KHHA: "𑂎\U000110ba",
        C.GHHA: "𑂏\U000110ba",
        C.ZA: "𑂔\U000110ba",
        C.DDDHA: "𑂙\U000110ba",
        C.RHA: "𑂛\U000110ba",
        C.FA: "𑂤\U000110ba",
        C.YYA: "𑂨\U000110ba",
    },
    "KHAROSHTHI": {
        C.DANDA: "\U00010a56",
        C.DOUBLE_DANDA: "\U00010a57",
    },
    "KHMER": {
        C.DANDA: "។",
        C.DOUBLE_DANDA: "៕",
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
        C.DANDA: "꯫",
        C.DOUBLE_DANDA: "꯫꯫",
    },
    "MALAYALAM": {
        # sign short o
        "\u094a": "\u0d4a",
        # sign au
        C.SIGN_AU: "\u0d57",
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
        C.DANDA: "\U00011641",
        C.DOUBLE_DANDA: "\U00011642",
    },
    "MON": {
        C.DANDA: "\u104a",
        C.DOUBLE_DANDA: "\u104b",
    },
    "NEWA": {
        C.DANDA: "\U0001144b",
        C.DOUBLE_DANDA: "\U0001144c",
    },
    "SOYOMBO": {
        # Errors in `common_maps`
        "\U00011a9b": None,
        "\U00011a9c": None,
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
    "TELUGU": {
        C.QA: None,
        C.KHHA: None,
        C.GHHA: None,
        C.ZA: None,
        C.DDDHA: None,
        C.RHA: None,
        C.FA: None,
        C.YYA: None,
    },
    "TIBETAN": {
        # Virama
        C.VIRAMA: "\u0f84",
        # Use distinct "va" character instead of "ba".
        "व": "\u0f5d",
    },
    "TIRHUTA_MAITHILI": {
        C.RRA: "𑒩\U000114C3",
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
        C.SIGN_R: "\U00011A34\U00011A2B\U00011A09",  # sign vocalic r
        C.SIGN_RR: "\U00011A34\U00011A2B\U00011A09\U00011A0A",  # sign vocalic rr
        C.SIGN_L: "\U00011A34\U00011A2C\U00011A09",  # sign vocalic l
        C.SIGN_LL: "\U00011A34\U00011A2C\U00011A09\U00011A0A",  # sign vocalic ll
    },
}


# Additional characters not present in common_map (or deleted in OVERRIDES)
EXTENSIONS = {
    "ASSAMESE": [
        (C.CANDRABINDU_VIRAMA, "\u09fc"),
        (C.ABBREVIATION_SIGN, "\u09fd"),
    ],
    "BARAHA": [
        ("\u0914", "ou"),
        (C.SIGN_AU, "ou"),
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
    "BENGALI": [
        (C.CANDRABINDU_VIRAMA, "\u09fc"),
        (C.ABBREVIATION_SIGN, "\u09fd"),
    ],
    "BRAHMI": [
        (C.JIHVAMULIYA, "\U00011003"),
        (C.UPADHMANIYA, "\U00011004"),
    ],
    "DEVANAGARI": [
        # DEVANAGARI VOWEL SIGN PRISHTHAMATRA E (U+094E)
        # See comments on U+094E for details.
        ("\u0948", "\u0947\u094e"),
        ("\u094b", "\u093e\u094e"),
        (C.SIGN_AU, "\u094b\u094e"),

        (C.PRENKHA, C.PRENKHA),
        (C.DIRGHA_SVARITA, C.DIRGHA_SVARITA),
        (C.VEDIC_DOT_BELOW, C.VEDIC_DOT_BELOW),
        (C.COMBINING_DIGIT_0, C.COMBINING_DIGIT_0),
        (C.DOUBLE_CANDRABINDU_VIRAMA, C.DOUBLE_CANDRABINDU_VIRAMA),
        # Punctuation
        ("\u0970", "\u0970"),
        ("\u0971", "\u0971"),
    ],
    "DOGRA": [
        (C.ABBREVIATION_SIGN, "\U0001183b"),
    ],
    "GONDI_MASARAM": [
        ("त्र", "𑴰"),
    ],
    "GRANTHA": [
        # OO (EE + AA length mark)
        ("\u094b", "\U00011347\U0001133e"),
        # AU length mark
        (C.SIGN_AU, "\U00011357"),
        # AU (AA + AU length mark)
        (C.SIGN_AU, "\U00011347\U00011357"),

        (C.COMBINING_DIGIT_0, "\U00011366"),
        (C.COMBINING_DIGIT_1, "\U00011367"),
        (C.COMBINING_DIGIT_2, "\U00011368"),
        (C.COMBINING_DIGIT_3, "\U00011369"),
        (C.COMBINING_DIGIT_4, "\U0001136a"),
        (C.COMBINING_DIGIT_5, "\U0001136b"),
        (C.COMBINING_DIGIT_6, "\U0001136c"),
        (C.NUKTA, "\U0001133c"),
        (C.CANDRABINDU_VIRAMA, "\U0001135e"),
        (C.DOUBLE_CANDRABINDU_VIRAMA, "\U0001135f"),

        # -- 3 reserved chars --
        (C.COMBINING_A, "\U00011370"),
        (C.COMBINING_KA, "\U00011371"),
        (C.COMBINING_NA, "\U00011372"),
        (C.COMBINING_PA, "\U00011374"),
        (C.COMBINING_VI, "\U00011373"),
    ],
    "GUJARATI": [
        (C.ABBREVIATION_SIGN, "\u0af0"),
    ],
    "GURMUKHI": [
        (C.ABBREVIATION_SIGN, "\u0a76"),
    ],
    "IAST": [
        (C.CANDRABINDU, "\u0303"),
    ],
    "ITRANS": [
        # Vedic anusvara (just render as candrabindu)
        ("\u0901", "{\\m+}"),
        ("\u1cda", "\\\""),
    ],
    "ISO": [
        (C.TAMIL_AYTHAM, "ḳ"),
        (C.QA, "q"),
        (C.JIHVAMULIYA, "ẖ"),
        (C.UPADHMANIYA, "ḫ"),
        (C.KA + C.VIRAMA + C.HA, "k:h"),
        (C.GA + C.VIRAMA + C.HA, "g:h"),
        (C.CA + C.VIRAMA + C.HA, "c:h"),
        (C.JA + C.VIRAMA + C.HA, "j:h"),
        (C.TTA + C.VIRAMA + C.HA, "ṭ:h"),
        (C.DDA + C.VIRAMA + C.HA, "ḍ:h"),
        (C.TA + C.VIRAMA + C.HA, "t:h"),
        (C.DA + C.VIRAMA + C.HA, "d:h"),
        (C.PA + C.VIRAMA + C.HA, "p:h"),
        (C.BA + C.VIRAMA + C.HA, "b:h"),
    ],
    "KANNADA": [
        (C.JIHVAMULIYA, "\u0cf1"),
        (C.UPADHMANIYA, "\u0cf2"),
    ],
    "KAITHI": [
        (C.NUKTA, "\U000110ba"),
        (C.RRA, "𑂩\U000110ba"),
        (C.ABBREVIATION_SIGN, "\U000110bb"),
    ],
    "KHUDAWADI": [
        (C.NUKTA, "\U000112e9"),
        (C.DDDHA, "\U000112ca"),
        (C.RRA, "\U000112d9\U000112e9"),
    ],
    "MALAYALAM": [
        # AU archaic mark
        (C.SIGN_AU, "\u0d4c"),
        (C.CANDRABINDU_VIRAMA, "\u0d04"),
    ],
    "MODI": [
        (C.ABBREVIATION_SIGN, "\U00011643"),
    ],
    "NEWA": [
        (C.JIHVAMULIYA, "\U00011460"),
        (C.UPADHMANIYA, "\U00011461"),
        (C.NUKTA, "\U00011446"),
        (C.RRA, "\U0001142c\U00011446"),
        (C.CANDRABINDU_VIRAMA, "\U0001145f"),
        (C.ABBREVIATION_SIGN, "\U0001144f"),
    ],
    "SHARADA": [
        (C.JIHVAMULIYA, "\U000111c2"),
        (C.UPADHMANIYA, "\U000111c3"),
        (C.ABBREVIATION_SIGN, "\U000111c7"),
        # Discouraged "SHARADA OM," but support anyway.
        (C.OM, "\U000111c4")
    ],
    "SINHALA": 
    # Sinhala chandrabindu is not supported in the fonts I tried, so
    # use anusvara instead.
    [("\u0901", "\u0d82")],
    "SLP1": [
        (C.JIHVAMULIYA, "Z"),
        (C.UPADHMANIYA, "V"),
        ("ळ्ह", "|"),
        (C.SVARITA, "^"),
        (C.ANUDATTA, "\\"),
    ],
    "SOYOMBO": [
        (C.JIHVAMULIYA, "\U00011a84"),
        (C.UPADHMANIYA, "\U00011a85"),
        (C.DANDA, "\U00011a9b"),
        (C.DOUBLE_DANDA, "\U00011a9c"),
    ],
    "TAKRI": [
        (C.ABBREVIATION_SIGN, "\U000116b9"),
    ],
    "TAMIL_SUPERSCRIPTED": [
        (C.TAMIL_AYTHAM, "\u0b83"),
    ],
    "TELUGU": [
        (C.NUKTA, "\u0c3c"),
    ],
    "TIBETAN": [
        (C.JIHVAMULIYA, "\u0f88"),
        (C.UPADHMANIYA, "\u0f89"),
    ],
    "TIRHUTA_MAITHILI": [
        (C.ABBREVIATION_SIGN, "\U000114c6"),
    ],
    "VELTHUIS": [
        (C.VIRAMA, "&"),
        # Chandrabindu variant
        (C.CANDRABINDU, "/"),
        (C.SIGN_CANDRA_E, "~a"),
        (C.SIGN_CANDRA_O, "~o"),
        # Punctuation
        ("\u0970", "@"),
        ("\u0971", "#"),
        # Consonants with nuktas
        (C.RRA, "^r"),
        (C.QA, "q"),
        (C.KHHA, ".kh"),
        (C.GHHA, ".g"),
        (C.ZA, "z"),
        (C.DDDHA, "R"),
        (C.RHA, "Rh"),
        (C.FA, "f"),
        (C.YYA, ".y")
    ],
}


def _sanitize(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


def to_unique(xs: list) -> list:
    """Remove duplicates from `xs`."""
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
    # Rank short vowels lower than long vowels so that our transliterator
    # chooses long vowels when reversing.
    short_vowel_codes = {C.E, C.O, C.SIGN_E, C.SIGN_O}
    no_short_e_o = [x for x in items if x[0] not in short_vowel_codes]
    short_e_o = [x for x in items if x[0] in short_vowel_codes]
    return no_short_e_o + short_e_o


def _reorder_kharoshthi_n(items: list) -> list:
    nga = {C.NGA}
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
        # subprocess.run(f"git clone --depth 1 {repo}", shell=True)

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
                    deva = unicodedata.normalize("NFC", _sanitize(deva))
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

        # Add svarita and anudatta for Brahmic scripts that use Devanagari accent marks.
        if scheme_name in BRAHMIC_WITH_DEVA_ACCENTS:
            scheme_items.extend(
                [
                    (C.SVARITA, "\u0951"),
                    (C.ANUDATTA, "\u0952"),
                    (C.DIRGHA_SVARITA, C.DIRGHA_SVARITA),
                ]
            )
        elif scheme_name == "GRANTHA":
            scheme_items.extend(
                [
                    # Svarita (use chandra symbol)
                    (C.SVARITA, "\u1cf4"),
                    # Dirgha svarita (use Devanagari svarita)
                    (C.DIRGHA_SVARITA, "\u0951"),
                    # Anudatta (use Devanagari)
                    (C.ANUDATTA, "\u0952"),
                ]
            )

        scheme_items.extend(EXTENSIONS.get(scheme_name, []))

        scheme_items = _reorder_short_vowels(scheme_items)
        if scheme_name == "OL_CHIKI":
            scheme_items = _ol_chiki_consonants(scheme_items)
        elif scheme_name == "KHAROSHTHI":
            scheme_items = _reorder_kharoshthi_n(scheme_items)

        # Finally, rename the scheme per our conventions.
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

        buf.append(create_scheme_entry(scheme_name, scheme_items))

    with open(CRATE_DIR / "src/autogen_schemes.rs", "w") as f:
        f.write("\n".join(buf))

    print("Cleaning up ...")
    # shutil.rmtree(common_maps)

    print("Done.")


if __name__ == "__main__":
    main()
