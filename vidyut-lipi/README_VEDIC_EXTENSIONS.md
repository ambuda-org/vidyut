# Vedic Extensions for vidyut-lipi

This document describes the Vedic transliteration extension system implemented in vidyut-lipi.

## Overview

The Vedic extension system allows sakha-specific (branch-specific) transliteration of Vedic texts with proper accent preservation across all Indic and Roman scripts supported by vidyut-lipi.

## Key Features

1. **Accent Preservation**: Vedic accent marks (udatta, anudatta, svarita) are preserved exactly through round-trip transliteration
2. **Multi-Script Support**: Works across 40+ scripts including:
   - Modern Indic scripts (Devanagari, Bengali, Tamil, etc.)
   - Historical scripts (Brahmi, Grantha, Sharada, etc.)
   - Roman transliteration schemes (IAST, Harvard-Kyoto, etc.)
3. **Sakha-Specific Rules**: Each Vedic branch can define its own accent notation system
4. **Extensible Design**: New sakhas can be added without modifying core code

## Architecture

### Extension Trait

```rust
pub trait TransliterationExtension: Send + Sync {
    fn name(&self) -> &str;
    fn extend_mapping(&self, mapping: &mut Mapping);
    fn pre_process(&self, text: &str) -> String;
    fn post_process(&self, text: &str) -> String;
}
```

### Vedic Sakha Trait

```rust
pub trait VedicSakha: Send + Sync {
    fn name(&self) -> &str;
    fn accent_mappings(&self) -> Vec<(&'static str, &'static str, VedicAccent)>;
    fn phonetic_mappings(&self) -> Vec<(&'static str, &'static str)>;
}
```

## Usage

```rust
use vidyut_lipi::{Lipika, Scheme};
use vidyut_lipi::extensions::vedic::{RigvedaShakala, VedicExtension};

// Create a transliterator with Rigveda extension
let mut lipika = Lipika::new()
    .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));

// Transliterate with accent preservation
let verse = "agni'mI_le puro'hitam"; // ' = udatta, _ = anudatta
let devanagari = lipika.transliterate(verse, Scheme::HarvardKyoto, Scheme::Devanagari);
// Result: अग्नि॑मी॒ले पुरो॑हितम्
```

## Implemented Sakhas

### 1. Rigveda Shakala
- Udatta: `'` → `॑` (U+0951)
- Anudatta: `_` → `॒` (U+0952)
- Svarita: `=` → `॒॑` (combined)

### 2. Taittiriya Yajurveda
- Udatta: `#` → `॑` (U+0951)
- Anudatta: `q` → `॒` (U+0952)
- Svarita: `=` → `॓` (U+0953)
- Special phonetic markers: `(gm)`, `(gg)`, `~M`

### 3. Samaveda Kauthuma
- Uses numeric notation for musical tones
- `1`, `2`, `3` for different pitch levels

### 4. Atharvaveda Saunaka
- Simplified notation similar to Rigveda
- Udatta: `'` → `॑`
- Anudatta: `_` → `॒`

## Round-Trip Preservation

The system ensures perfect round-trip transliteration:

```
HarvardKyoto → Devanagari → HarvardKyoto
agni'mI_le → अग्नि॑मी॒ले → agni'mI_le ✓
```

This works across any combination of supported scripts:

```
HK → Devanagari → Tamil → Kannada → Bengali → IAST → HK
```

## Technical Details

### Bidirectional Mapping

The extension system automatically creates bidirectional mappings based on the transliteration direction:

- **Alphabet → Abugida**: ASCII marks (`'`, `_`) → Unicode combining marks (`॑`, `॒`)
- **Abugida → Alphabet**: Unicode marks → ASCII marks
- **Abugida → Abugida**: Unicode marks preserved as-is

### Script Coverage

The system has been tested and verified to work with:

- **All modern Indic scripts**: Devanagari, Bengali, Gujarati, Gurmukhi, Kannada, Malayalam, Odia, Tamil, Telugu
- **Historical scripts**: Brahmi, Grantha, Sharada, Siddham, Modi, Tirhuta
- **Roman schemes**: IAST, ISO 15919, Harvard-Kyoto, Velthuis, SLP1, WX, ITRANS

## Testing

Comprehensive tests ensure:
1. Accent preservation across all scripts
2. Round-trip accuracy
3. Multi-script chain preservation
4. Sakha-specific notation consistency

Run tests with:
```bash
cargo test --test vedic_all_scripts
cargo test --test extensions
cargo test --test vedic_verses
```

## Examples

See the examples directory for demonstrations:
- `examples/vedic_extension.rs` - Basic usage
- `examples/vedic_all_scripts.rs` - Comprehensive multi-script demo

## Future Extensions

The system is designed to easily accommodate:
- Additional sakhas (Maitrayaniya, Kathaka, etc.)
- More complex accent systems (e.g., Samaveda's 7-tone system)
- Manuscript-specific conventions
- Regional variations in accent notation