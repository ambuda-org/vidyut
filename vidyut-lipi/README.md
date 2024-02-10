<div align="center">
<h1><code>vidyut-lipi</code></h1>
<p><i>An Indic transliterator</i></p>
</div>

`vidyut-lipi` is a transliteration library for Sanskrit and Pali that also
supports many of the scripts used within the Indosphere. Our goal is to provide
a standard transliterator that is easy to bind to other programming languages.

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

Communities around the world write Sanskrit and other Indic languages with
different scripts in different contexts. For example, a user might type
Sanskrit in ITRANS, read it in Kannada, and publish it in Devanagari. Such
communities often rely on a *transliterator*, which converts text from one
encoding scheme to another.

While various transliterators exist, none are both high-quality and widely
available in different programming languages. The result is that maintenance
and feature work is diluted across several different implementations.

`vidyut-lipi` aims to provide a standard transliterator for the Sanskrit
ecosystem. Our priorities are:

- quality, including a comprehensive test suite.
- test coverage across all of the schemes in common use.
- a precise and ergonomic API.
- availability in multiple languages, including Python and WebAssembly.
- high performance across various metrics, including runtime, startup time, and
  file size.

We encourage you to [file an issue][issue] if `vidyut-lipi` does not support
your use case. We are especially excited about supporting new scripts and new
programming languages.

[issue]: https://github.com/ambuda-org/vidyut/issues


Alternatives to `vidyut-lipi`
-----------------------------

There are two main alternatives to `vidyut-lipi`, both of which have been
influential on our overall design:

- [Aksharamukha][am] offers high quality and supports more than a hundred
  different scripts. Aksharamukha offers best-in-class transliteration, but it
  is available only in Python.

- [indic-transliteration][it] implements the same basic transliterator in
  multiple programming languages. indic-transliteration supports a large
  software ecosystem, but its different implementations each have their own
  quirks and limitations.

[am]: https://github.com/virtualvinodh/aksharamukha/
[it]: https://github.com/indic-transliteration

Our long-term goal is to combine the quality of Aksharamukha with the
availability of indic-transliteration. Until then, `vidyut-lipi` provides the
following short-term benefits:

- High-quality transliteration for Rust and WebAssembly.
- Smooth support for other programming languages through projects like
  [pyo3][pyo3] (Python), [magnus][magnus] (Ruby), [cxx][cxx] (C++), etc.

[pyo3]: https://pyo3.rs/v0.20.2/
[magnus]: https://github.com/matsadler/magnus
[cxx]: https://cxx.rs/


Usage
-----

We recommend using `vidyut-lipi` through our `Lipika` API:

```rust
use vidyut_lipi::{Lipika, Scheme};

// `Lipika` must be `mut` since its method calls mutate its internal cache.
let mut lipika = Lipika::new();

let result = lipika.transliterate("saMskRtam", Scheme::HarvardKyoto, Scheme::Devanagari);
assert_eq!(result, "संस्कृतम्");
```

You can also use `detect` to detect which `Scheme` a piece of text might be using:

```rust
use vidyut_lipi::{Lipika, Scheme, detect};

let some_text = "संस्कृतम्";
let detected = detect(&some_text).unwrap_or(Scheme::HarvardKyoto);

let mut lipika = Lipika::new();
let result = lipika.transliterate(some_text, detected, Scheme::HarvardKyoto);
assert_eq!(result, "saMskRtam");
```

For a list of all available `Scheme`s, you can use `Scheme::iter()`:

```rust
use vidyut_lipi::{Lipika, Scheme, detect};

let mut lipika = Lipika::new();
for scheme in Scheme::iter() {
  let result = lipika.transliterate("saMskRtam", Scheme::HarvardKyoto, *scheme);
  println!("{:15} {result}", format!("{:?}", scheme));
}
```

As of 2024-01-27, this code prints the following:

```text
Balinese        ᬲᬂᬲ᭄ᬓᬺᬢᬫ᭄
BarahaSouth     saMskRutam
Bengali         সংস্কৃতম্
Brahmi          𑀲𑀁𑀲𑁆𑀓𑀾𑀢𑀫𑁆
Burmese         သံသ်ကၖတမ်
Devanagari      संस्कृतम्
Grantha         𑌸𑌂𑌸𑍍𑌕𑍃𑌤𑌮𑍍
Gujarati        સંસ્કૃતમ્
Gurmukhi        ਸਂਸ੍ਕਤਮ੍
HarvardKyoto    saMskRtam
Iast            saṃskṛtam
Iso15919        saṁskr̥tam
Itrans          saMskRRitam
Javanese        ꦱꦁꦱ꧀ꦏꦽꦠꦩ꧀
Kannada         ಸಂಸ್ಕೃತಮ್
Khmer           សំស្ក្ឫតម៑
Malayalam       സംസ്കൃതമ്
Modi            𑘭𑘽𑘭𑘿𑘎𑘵𑘝𑘦𑘿
Newa            𑐳𑑄𑐳𑑂𑐎𑐺𑐟𑐩𑑂
Odia            ସଂସ୍କୃତମ୍
Saurashtra      ꢱꢀꢱ꣄ꢒꢺꢡꢪ꣄
Sharada         𑆱𑆁𑆱𑇀𑆑𑆸𑆠𑆩𑇀
Siddham         𑖭𑖽𑖭𑖿𑖎𑖴𑖝𑖦𑖿
Sinhala         සංස්කෘතම්
Slp1            saMskftam
Tamil           ஸம்ʼஸ்க்ருʼதம்
Telugu          సంస్కృతమ్
Thai            สํสฺกฺฤตมฺ
Tibetan         སཾསྐྲྀཏམ
Tirhuta         𑒮𑓀𑒮𑓂𑒏𑒵𑒞𑒧𑓂
Velthuis        sa.msk.rtam
Wx              saMskqwam
```

`Lipika` is a thin wrapper over the `transliterate` function. We recommend
`Lipika` because it handles some bookkeeping and caching on your behalf, but if
you want more precise control, you can use `transliterate` directly like so:

```rust
use vidyut_lipi::{transliterate, Mapping, Scheme};

let mapping = Mapping::new(Scheme::HarvardKyoto, Scheme::Devanagari);
let result = transliterate("saMskRtam", &mapping);
assert_eq!(result, "संस्कृतम्");
```
