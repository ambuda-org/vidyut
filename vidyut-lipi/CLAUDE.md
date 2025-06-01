# Claude Development Notes

This file tracks ongoing development work and PRs that need attention.

## Active Pull Requests

### Vedic Transliteration Extension System
- **PR**: https://github.com/ambuda-org/vidyut/pull/204
- **Branch**: `feat/schema/sakha-specific-vedic-transliteration-schemes`
- **Status**: Under Review
- **Created**: 2024-12-31

**Summary**: Adds extensible Vedic transliteration system with sakha-specific support for preserving accent marks across all scripts.

**Key Features**:
- Extension trait system for customizing transliteration
- Support for all four Vedas (Rigveda, Yajurveda, Samaveda, Atharvaveda)
- Perfect round-trip preservation of accents
- Works with 40+ scripts

**Action Items**:
- [ ] Monitor PR for review comments
- [ ] Address any requested changes
- [ ] Update tests if needed
- [ ] Resolve any merge conflicts
- [ ] Update PR demonstration comment after each commit

**PR Demonstration Comments**:
- **Purpose**: Track sakha-specific Vedic transliteration with real examples over time
- **Data Source**: Authentic Vedic verses with udātta/anudātta marks from udapaana corpus
- **Verses Used**:
  - RV 1.1.1 (RigvedaShakala extension)
  - AV 2.1.1 (AtharvavedaSaunaka extension) 
  - SV 1.1.1 (SamavedaKauthuma extension)
  - TS 1.1.1.1 (TaittiriyaYajurveda extension)
- **Scripts Demonstrated**: Devanagari, Telugu, ISO 15919, WX (sakha-specific)
- **Update Script**: `create_pr_comment_with_accents.py` 
- **Update Command**: `gh pr comment https://github.com/ambuda-org/vidyut/pull/204 --body "$(python3 create_pr_comment_with_accents.py)"`
- **Update Strategy**: Add new comment after each significant commit to track progress
- **Comment History**:
  - 2925895271: Initial sakha-specific demonstration (2025-01-31)
  - 2926311811: Requirements-based system demonstration (2025-01-31)

**Files Changed**:
- Core: `src/lib.rs`, `src/lipika.rs`, `src/mapping.rs`, `src/transliterate.rs`
- Extensions: `src/extensions/` (new module)
- Tests: `tests/extensions.rs`, `tests/vedic_all_scripts.rs`, `tests/vedic_verses.rs`
- Examples: `examples/vedic_extension.rs`, `examples/vedic_all_scripts.rs`
- Docs: `README_VEDIC_EXTENSIONS.md`

## Development Guidelines

When addressing PR comments:
1. Pull latest changes from upstream main
2. Address each comment thoroughly
3. Add tests for any edge cases identified
4. Update documentation as needed
5. Re-run all tests before pushing

## Commands

```bash
# Run Vedic extension tests
cargo test --test extensions
cargo test --test vedic_all_scripts
cargo test --test vedic_verses

# Run linting
cargo clippy

# Check formatting
cargo fmt --check
```