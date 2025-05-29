# Transliteration Scheme Extensions

## Overview

All major transliteration schemes in vidyut-lipi have been extended to support the full range of Vedic characters that UVTS handles, ensuring comprehensive coverage across all schemes.

## Extended Schemes

### Roman/Latin-based Schemes

#### 1. **Harvard-Kyoto (HK)**
**Added:**
- `JIHVAMULIYA` → `jh` 
- `UPADHMANIYA` → `ph`

**Already had:**
- `SVARITA` → `̭` (combining mark)
- `ANUDATTA` → `॒` (combining mark)

#### 2. **IAST**
**Added:**
- `SVARITA` → `॑` (combining mark)
- `ANUDATTA` → `॒` (combining mark)
- `JIHVAMULIYA` → `ḵ` (standard IAST notation)
- `UPADHMANIYA` → `ṗ` (standard IAST notation)

#### 3. **ISO 15919**
**Added:**
- `SVARITA` → `॑` (combining mark)
- `ANUDATTA` → `॒` (combining mark)
- `JIHVAMULIYA` → `ḵ` (same as IAST)
- `UPADHMANIYA` → `ṗ` (same as IAST)

#### 4. **Velthuis**
**Added:**
- `SVARITA` → `\\accent` (Velthuis-style)
- `ANUDATTA` → `/accent` (Velthuis-style)
- `JIHVAMULIYA` → `.jh` (dot notation)
- `UPADHMANIYA` → `.ph` (dot notation)

#### 5. **WX**
**Added:**
- `SVARITA` → `=` (ASCII-safe)
- `ANUDATTA` → `/` (ASCII-safe)
- `JIHVAMULIYA` → `{` (brace notation)
- `UPADHMANIYA` → `}` (brace notation)

#### 6. **SLP1** 
**Already complete** - had all Vedic characters:
- `SVARITA` → `^`
- `ANUDATTA` → `\\`
- `JIHVAMULIYA` → `Z`
- `UPADHMANIYA` → `V`

### Indic Script-based Schemes

#### 1. **Scripts with Native Vedic Support**
**Already complete** - these scripts have dedicated Unicode characters:
- **Devanagari**: `ᳵ` (jihvāmūlīya), `ᳶ` (upadhmānīya)
- **Kannada**: `ೱ` (jihvāmūlīya), `ೲ` (upadhmānīya)
- **Sharada**: `𑇂` (jihvāmūlīya), `𑇃` (upadhmānīya)
- **Brahmi**: `𑀃` (jihvāmūlīya), `𑀄` (upadhmānīya)
- **Newa**: `𑑠` (jihvāmūlīya), `𑑡` (upadhmānīya)
- **Soyombo**: `𑪄` (jihvāmūlīya), `𑪅` (upadhmānīya)
- **Tibetan**: `ྈ` (jihvāmūlīya), `ྉ` (upadhmānīya)

#### 2. **Scripts Extended with Approximations**
**Added** using consonant + virama + ZWNJ combinations:
- **Tamil**: `க்‌` (ka+virama+ZWNJ), `ப்‌` (pa+virama+ZWNJ)
- **Telugu**: `క్‌` (ka+virama+ZWNJ), `ప్‌` (pa+virama+ZWNJ)
- **Malayalam**: `ക്‌` (ka+virama+ZWNJ), `പ്‌` (pa+virama+ZWNJ)  
- **Bengali**: `ক্‌` (ka+virama+ZWNJ), `প্‌` (pa+virama+ZWNJ)
- **Gujarati**: `ક્‌` (ka+virama+ZWNJ), `પ્‌` (pa+virama+ZWNJ)

*Note: These scripts already had SVARITA and ANUDATTA using the shared Unicode combining marks ॑ and ॒*

## Character Coverage Comparison

| Character | Devanagari | UVTS | SLP1 | HK | IAST | ISO15919 | Velthuis | WX |
|-----------|------------|------|------|-----|------|----------|----------|-----|
| Svarita | ॑ | = | ^ | ̭ | ॑ | ॑ | \\accent | = |
| Anudātta | ॒ | / | \\ | ॒ | ॒ | ॒ | /accent | / |
| Jihvāmūlīya | ᳵ | { | Z | jh | ḵ | ḵ | .jh | { |
| Upadhmānīya | ᳶ | } | V | ph | ṗ | ṗ | .ph | } |

## Benefits

1. **Universal Coverage**: All schemes can now handle Vedic texts with full accent notation
2. **Bidirectional Conversion**: Can convert between any scheme preserving Vedic characters
3. **Academic Standards**: Each scheme uses its conventional notation for these characters
4. **ASCII Compatibility**: ASCII schemes (UVTS, SLP1, WX) maintain ASCII-only encoding

## Usage Examples

### Vedic Text with Accents
```rust
// Input Devanagari with Vedic accents
let devanagari = "अग्नि॑म् ईळे॒";

// Can now convert to any scheme:
let uvts = lipika.transliterate(devanagari, Devanagari, Uvts);      // "agni=m ILe/"
let slp1 = lipika.transliterate(devanagari, Devanagari, Slp1);      // "agni^m ILe\\"  
let iast = lipika.transliterate(devanagari, Devanagari, Iast);      // "agni॑m īḷe॒"
let hk = lipika.transliterate(devanagari, Devanagari, HarvardKyoto); // "agni̭m ILe॒"
```

### Jihvāmūlīya and Upadhmānīya
```rust
// Input with special consonants
let devanagari = "तत्ᳵकारः तत्ᳶपूर्वम्";

// Convert to various schemes:
let uvts = lipika.transliterate(devanagari, Devanagari, Uvts);      // "tat{kAraH tat}pUrvam"
let slp1 = lipika.transliterate(devanagari, Devanagari, Slp1);      // "tatZkAraH tatVpUrvam"
let iast = lipika.transliterate(devanagari, Devanagari, Iast);      // "tatḵkāraḥ tatṗpūrvam"
```

## Testing

The extensions include comprehensive tests to verify:
- Bidirectional conversion between all schemes
- Preservation of Vedic characters during transliteration
- Correct mapping of each Vedic character in each scheme
- Round-trip consistency

## Future Enhancements

1. **Additional Vedic Characters**: Support for more specialized Vedic Unicode characters
2. **Context-Aware Notation**: Better handling of accent placement rules
3. **Śākhā-Specific Variants**: Different conventions for different Vedic traditions
4. **Validation Tools**: Functions to validate Vedic text encoding