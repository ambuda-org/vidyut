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