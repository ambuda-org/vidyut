# UVTS Implementation Notes

## What's Implemented

The UVTS (Unified Vedic Transliteration Scheme) has been added to vidyut-lipi with comprehensive support for:

### 1. Basic Character Mappings
- **Vowels**: a, A, i, I, u, U, R (ṛ), q (ṝ), L (ḷ), Q (ḹ), e, E (ai), o, O (au)
- **Consonants**: 
  - k, K, g, G, f (ṅ)
  - c, C, j, J, F (ñ)
  - w (ṭ), W (ṭh), x (ḍ), X (ḍh), N (ṇ)
  - t, T, d, D, n
  - p, P, b, B, m
  - y, r, l, v
  - S (ś), z (ṣ), s, h
- **Special characters**: M (ṃ), H (ḥ), ~ (candrabindu)
- **Punctuation**: | (daṇḍa), || (double daṇḍa)
- **Numerals**: 0-9

### 2. Vedic Accent Support with Syllabification
- **Post-syllable accent marking** using vidyut-chandas for proper syllable detection
- **Udātta**: \ (after syllable)
- **Anudātta**: / (after syllable)
- **Svarita**: = (after syllable)
- **Function**: `devanagari_to_uvts_with_vedic_accents()` - converts Devanagari with Unicode accent marks to UVTS with proper syllable-based accent placement
- **Jihvāmūlīya**: { (ᳵ)
- **Upadhmānīya**: } (ᳶ)

### 3. Musical Notations (Sāmaveda)
- **Elongation markers**: +, ++, +++ (1-3 mātrā extensions)
- **Pause markers**: ^, ^^ (short and long pauses)
- **Glides**: ~ (up), ` (down)
- **Special markers**: % (stobha), @ (pluta)
- **Function**: `parse_musical_notations()` - parses UVTS text to identify musical notations
- **Function**: `add_musical_notation()` - adds musical notation at specified positions

### 4. Regional Variants and Manuscript Markers
- **Regional variants**: [andhra], [dravida], [maharastra], [gujarat], [kashmir], [kerala], [karnataka], [bengal]
- **Function**: `add_regional_variant()` - adds regional variant markers
- **Function**: `add_manuscript_marker()` - adds manuscript tradition markers like [ms:poona]

### 5. Advanced UVTS Module (`src/uvts.rs`)
The module provides:
- Syllable-aware accent placement
- Musical notation parsing and generation
- Regional and manuscript markers
- Integration with vidyut-chandas for accurate syllabification

## What's Still Limited

### 1. Context-Aware Handling
- **Conflict resolution**: Some symbols have multiple meanings (e.g., `~` for candrabindu vs. glide, `|` for daṇḍa vs. kampana)
- Currently these are handled as separate features, but context-aware parsing would improve accuracy

### 2. Śākhā-Specific Extensions
Not yet implemented:
- `•` (pada break)
- `·` (compound break)
- `_` (Taittirīya gap)
- `&` (udgītha mark)
- `{stotra}...{/stotra}`, `{saman}...{/saman}` (section markers)

### 3. Implicit Udātta Detection
- The system can mark explicit accents from Unicode, but detecting implicit udātta (unmarked high tone) would require:
  - Knowledge of Vedic accent patterns
  - Śākhā-specific rules
  - Contextual analysis

## Usage Examples

### Basic Transliteration
```rust
use vidyut_lipi::{Lipika, Scheme};

let mut lipika = Lipika::new();
let devanagari = "संस्कृतम्";
let uvts = lipika.transliterate(devanagari, Scheme::Devanagari, Scheme::Uvts);
println!("{}", uvts); // Output: "saMskRtam"
```

### With Vedic Accents
```rust
use vidyut_lipi::uvts::devanagari_to_uvts_with_vedic_accents;

// Devanagari with Unicode accent marks
let devanagari_with_accents = "अग्नि॒मीळे॑"; // anudātta on gni, udātta on ḷe
let uvts = devanagari_to_uvts_with_vedic_accents(devanagari_with_accents);
println!("{}", uvts); // Output: "agni/mILe\"

### Musical Notations (Sāmaveda)
```rust
use vidyut_lipi::uvts::{parse_musical_notations, add_musical_notation, MusicalNotation};

// Parse existing musical notations
let samaveda_text = "agni+m I++Le puro^hita";
let parsed = parse_musical_notations(samaveda_text);

// Add musical notation
let text = "agnim";
let with_elongation = add_musical_notation(text, 3, MusicalNotation::Elongation1);
println!("{}", with_elongation); // Output: "agni+m"
```

### Regional Variants
```rust
use vidyut_lipi::uvts::{add_regional_variant, RegionalVariant};

let uvts_text = "namaste";
let with_region = add_regional_variant(uvts_text, RegionalVariant::Kerala);
println!("{}", with_region); // Output: "namaste [kerala]"
```

## Testing

The implementation includes comprehensive tests:
- Basic character mappings in `tests/basic.rs`
- Advanced features (accents, musical notations) in `src/uvts.rs`
- Integration with vidyut-chandas for syllabification

## Future Enhancements

1. **Context-aware parsing** to handle symbol conflicts
2. **Śākhā-specific modules** for specialized notation
3. **Implicit accent detection** based on Vedic rules
4. **Export/import utilities** for UVTS file format
5. **Web interface** for interactive UVTS encoding/decoding