<div align="center">
<h1><code>vidyut-chandas</code></h1>
<p><i>A Sanskrit metrical classifier</i></p>
</div>

`vidyut-chandas` identifies the meter in some piece of Sanskrit text.

This [crate][crate] is under active development as part of the [Ambuda][ambuda]
project. If you enjoy our work and wish to contribute to it, we encourage you
to [join our Discord server][discord], where you can meet other Sanskrit
programmers and enthusiasts.

An online demo is available [here][demo].

`vidyut-chandas` is not a state-of-the-art solution, and you might consider
exploring and using these other projects instead:

- [Skrutable](https://github.com/tylergneill/skrutable)
- [sanskritmetres](https://github.com/shreevatsa/sanskrit)

[crate]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[ambuda]: https://ambuda.org
[discord]: https://discord.gg/7rGdTyWY7Z
[demo]: https://ambuda-org.github.io/vidyut-lipi/

- [Overview](#overview)
- [Usage](#usage)


Overview
--------

Sanskrit poetry uses a variety of *meters*, which specify the syllable patterns
that a verse must follow. `vidyut-chandas` aims to provide a simple API for
identifying the meter that a given verse uses.

`vidyut-chandas` is experimental code and follows in the footsteps of great
projects like [sanskritmetres][sm] and [Skrutable][skrutable].

[sm]: https://github.com/shreevatsa/sanskrit
[skrutable]: https://github.com/tylergneill/skrutable


Usage
-----

*(This API is unstable.)*

We recommend using `vidyut-chandas` through our `Chandas` API:

```rust,no_run
use vidyut_chandas::{Chandas, MatchType, Vrtta};

let chandas = Chandas::from_file("/path/to/meters.tsv").unwrap();
let result = chandas.classify("mAtaH samastajagatAM maDukEwaBAreH");
assert_eq!(result.padya().as_ref().unwrap().name(), "vasantatilakA");
assert_eq!(result.match_type(), MatchType::Pada);
```
