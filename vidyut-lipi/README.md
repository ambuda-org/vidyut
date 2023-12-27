<div align="center">
<h1><code>vidyut-lipi</code></h1>
<p><i>A fast Sanskrit transliterator</i></p>
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

As of 2023-12-26, this code prints the following:

```text
Balinese        ᬲᬂᬲ᭄ᬓᬺᬢᬫ᭄
Bengali         সংস্কৃতম্
Brahmi          𑀲𑀁𑀲𑁆𑀓𑀾𑀢𑀫𑁆
Burmese         သံသ်ကၖတမ်
Devanagari      संस्कृतम्
Grantha         𑌸𑌂𑌸𑍍𑌕𑍃𑌤𑌮𑍍
Gujarati        સંસ્કૃતમ્
Gurmukhi        ਸਂਸ੍ਕਤਮ੍
HarvardKyoto    saMskRtam
Iast            saṃskṛtam
Itrans          saMskRRitam
Javanese        ꦱꦁꦱ꧀ꦏꦽꦠꦩ꧀
Kannada         ಸಂಸ್ಕೃತಮ್                                                                                                                                                            Malayalam       സംസ്കൃതമ്
Odia            ସଂସ୍କୃତମ୍                                                                                                                                                             Sharada         𑆱𑆁𑆱𑇀𑆑𑆸𑆠𑆩𑇀
Sinhala         සංස්කෘතම්
Slp1            saMskftam
Tamil           ஸம்ஸ்க்ரு'தம்
Telugu          సంస్కృతమ్
Velthuis        sa.msk.rtam
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
