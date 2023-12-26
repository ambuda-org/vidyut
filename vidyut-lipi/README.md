<div align="center">
<h1><code>vidyut-lipi</code></h1>
<p><i>A fast Indic transliterator</i></p>
</div>

`vidyut-lipi` is an experimental Sanskrit transliteration library that also
supports many of the scripts used within the Indosphere. Our goal is to provide
a standard transliterator for the Sanskrit ecosystem that is easy to bind to
other programming languages.

This [crate][crate] is under active development as part of the [Ambuda][ambuda]
project. If you enjoy our work and wish to contribute to it, we encourage you
to [join our Discord server][discord], where you can meet other Sanskrit
programmers and enthusiasts.

An online demo is available [here][demo].

[crate]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[ambuda]: https://ambuda.org
[discord]: https://discord.gg/7rGdTyWY7Z
[demo]: https://ambuda-org.github.io/vidyut-lipi/

- [Overview](#overview)
- [Usage](#usage)
- [Design](#design)


Overview
--------

Communities around the world write Sanskrit and other Indian languages in
different scripts in different contexts. For example, a user might type
Sanskrit in ITRANS, read it in Kannada, and publish it in Devanagari. Such
communities often rely on a *transliterator*, which converts text from one
scheme to another.

While various transliterators exist, none are both high-quality and widely
available in different programming languages. The result is that maintenance
and feature work is diluted across several different implementations.

`vidyut-lipi` aims to provide a standard transliterator for the Sanskrit
ecosystem. Our priorities are:

- quality, including a comprehensive test suite.
- coverage across all of the schemes in common use.
- ease of use (and reuse) for developers.
- high performance across various metrics, including runtime, startup time, and
  file size.

We recommend `vidyut-lipi` if you need a simple and high-quality
transliteration library, and we encourage you to [file an issue][issue] if
`vidyut-lipi` does not support your use case. We are especially excited about
supporting new scripts and new programming languages.

[issue]: https://github.com/ambuda-org/vidyut/issues

If `vidyut-lipi` is not right for your needs, we also strongly recommend
the [Aksharamukha][aksharamukha] the [indic-transliteration][indic-trans]
projects, which have each been highly influential in our work on `vidyut-lipi`.

[aksharamukha]: https://github.com/virtualvinodh/aksharamukha/
[indic-trans]: https://github.com/indic-transliteration


Usage
-----

For simple use cases that aren't very performance-sensitive, we recommend using
`vidyut-lipi` like so:

```rust
use vidyut_lipi::{Scheme, transliterate};

let result = transliterate("devO", Scheme::Slp1, Scheme::Iast);
assert_eq!(result, "devau");
```

We are still stabilizing our API and will share more examples here soon.
