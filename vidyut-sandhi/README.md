<div align="center">
<h1><code>vidyut-sandhi</code></h1>
<p><i>Useful sandhi utilities</i></p>
</div>

`vidyut-sandhi` contains various utilities for working with sandhi changes
between words. It is fast, simple, and appropriate for most use cases.

For your convenience, `vidyut-sandhi` contains helper scripts that will
generate an interesting and comprehensive list of sandhi rules. For details,
see the [Usage](#usage) section below.

This [crate][crate] is under active development as part of the [Ambuda][ambuda]
project. If you enjoy our work and wish to contribute to it, we encourage you
to [join our Discord server][discord], where you can meet other Sanskrit
programmers and enthusiasts.

- [Overview](#overview)
- [Usage](#usage)

[crate]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[ambuda]: https://ambuda.org
[discord]: https://discord.gg/7rGdTyWY7Z


Overview
--------

**Sandhi** is the name for the various sound change rules that occur in
Sanskrit. For example, the two terms *ca* and *iti* usually combine into a
single *ceti*.

Such changes depend on both a term's sounds and a term's morphology. For
example, the two terms *te* and *eva* could combine as either *ta eva* or *te
eva* depending on the grammatical number of the word *te*.

To describe sandhi fully, therefore, we must formalize a variety of phonetic
and morphological rules. But for most applications, this level of rigor is not
necessary.

`vidyut-sandhi`, is fast, simple, and appropriate for most use cases. It models
sandhi rules as a simple triple of `(first, second, result)` where `result` is
the combination of `first + second`. We also have a few ad-hoc rules for words
like `sa` and `eza`.

Since `vidyut-sandhi` uses a simple model internally, it will likely
overgenerate and return invalid splits. We provide a few heuristic functions to
ignore such splits, but a truly rigorous solution must have more morphological
awareness than this crate will provide. If you require more rigor, we suggest
using `vidyut-cheda`, which combines `vidyut-sandhi` with a dictionary and
ranker.


Usage
-----

First, create a list of sandhi rules:

```shell
cargo run generate_rules > sandhi-rules.csv
```

Then, you can use our `Splitter` like so:

```rust,no_run
# use vidyut_sandhi::Error;
use vidyut_sandhi::Splitter;

let s: Splitter = Splitter::from_csv("sandhi-rules.csv")?;

let input = "ceti";
for split in s.split_at(input, 1) {
  println!("{} -> {} {}", input, split.first(), split.second());
}
# Ok::<(), Error>(())
```

For extra flexibility, you can also create a list of rules manually:

```rust
use vidyut_sandhi::{Splitter, SplitsMap};

let mut map: SplitsMap = SplitsMap::new();
map.insert("e".to_string(), ("a".to_string(), "i".to_string()));
map.insert("e".to_string(), ("A".to_string(), "I".to_string()));

let s: Splitter = Splitter::from_map(map);

let input = "ceti";
for split in s.split_at(input, 1) {
  println!("{} -> {} {}", input, split.first(), split.second());
}
```
