<div align="center">
<h1><code>bindings-python</code></h1>
<p><i>Vidyut's Python bindings</i></p>
</div>

This directory provides Python bindings for Vidyut. Our work here focuses
specifically on Rust-Python conversion and *not* on the underlying logic.
This means we focus on:

- improving comments, documentation, and type hints
- adding better error messages
- making our API more exhaustive
- making our API more Pythonic
- adding extensive tests for our Rust-Python conversions

## Features

Vidyut's Python bindings provide comprehensive transliteration capabilities:

- **40+ transliteration schemes** including modern Indic scripts, historical scripts, and Roman schemes
- **Vedic extensions** for accurate accent preservation (udātta, anudātta, svarita) across all scripts
- **Sakha-specific support** for Rigveda, Yajurveda, Samaveda, and Atharvaveda traditions
- **Round-trip transliteration** ensuring perfect preservation of linguistic information
- **High performance** with Rust-powered transliteration engine


Links
-----

For users:

- Documentation: https://vidyut.readthedocs.io
- Releases: https://pypi.org/project/vidyut/
- Changes: https://vidyut.readthedocs.io/en/latest/changes.html
- Code: https://github.com/ambuda-org/vidyut
- Issues: https://github.com/ambuda-org/vidyut/issues
- Chat: https://discord.gg/7rGdTyWY7Z

For contributors:

- [CONTRIBUTING.md](https://github.com/ambuda-org/vidyut/blob/main/CONTRIBUTING.md)
