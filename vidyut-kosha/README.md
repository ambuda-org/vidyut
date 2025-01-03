<div align="center">
<h1><code>vidyut-kosha</code></h1>
<p><i>A compact Sanskrit lexicon</i></p>
</div>

`vidyut-kosha` defines a key-value store that can compactly map tens of
millions of Sanskrit words to their inflectional data. Depending on the
application, storage costs can be as low as 1 byte per word. This storage
efficiency comes at the cost of increased lookup time, but in practice, we have
found that this increase is negligible and well worth the efficiency gains
elsewhere.

For your convenience, `vidyut-kosha` contains helper scripts that will
automatically build an interesting and comprehensive dictionary. For details,
see the [Usage](#usage) section below.

This [crate][crate] is under active development as part of the [Ambuda][ambuda]
project. If you enjoy our work and wish to contribute to it, we encourage you
to [join our Discord server][discord], where you can meet other Sanskrit
programmers and enthusiasts.

- [Overview](#overview)
- [Usage](#usage)
- [Design](#design)

[crate]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[ambuda]: https://ambuda.org
[discord]: https://discord.gg/7rGdTyWY7Z


Overview
--------

Sanskrit is a highly inflected language, and any reasonable Sanskrit word list
will have tens of millions of words at minimum. In practice, Sanskrit programs
make various compromises on this word list so that performance doesn't suffer.

One common compromise is to store irregular words as-is and store regular words
just in their stem form. At query time, we guess at the query's underlying stem
with the help of prefix and suffix tables that we curate manually.

This approach is workable but has two main weaknesses:

- *Speed.* To illustrate, the query `ubhe` could yield the candidates `ubha`,
  `ubhA`, `ubhi`, `ubh`, and `ubhe`, all of which we must check against the
  store.

- *Overgeneration*. By using prefix and suffix tables, we will potentially
  accept invalid words like \*`sundarA` (for `sundarI`) or \*`putrakA` (for
  `putrikA`). This can be useful for some applications, but often it's a
  decision made by need, not by choice.

`vidyut-kosha` avoids these weaknesses. It stores words exactly, which avoids
overgeneration. And although single-key lookup is slightly slower, it softens the
multiple lookup problem, which makes it faster for many applications.

In exchange, we must pay the following price:

- We must construct the dictionary ahead of time. If we wish to change the
  dictionary, we must construct it again. Construction takes several minutes to
  complete.

- We must be able to convert our values to 64-bit integers. This sounds
  daunting but is straightforward in practice. (If you plan to use
  `vidyut-kosha` as an inflectional dictionary, our library seamlessly manages
  this conversion for you already.)


Usage
-----

Detailed notes are coming soon. For now, try running the following command:

```shell
$ make create_kosha
```


Design
------

Although `vidyut-kosha` can support any encoding type for keys, we strongly
recommend using [SLP1][slp1] or [WX][wx] for better space efficiency and query
performance.

Our underlying data structure is a [finite-state transducer][fst-wiki] (FST) as
implemented in the [`fst`][fst-crate] crate. For details on how FSTs work, we
recommend [this blog post][fst-blog] by the creator of the `fst` crate,
particularly the section titled "[Finite state machines as data
structures][fst-blog-section]."

Briefly, an FST generalizes the [trie][trie] data structure by storing both
shared prefixes and shared suffixes. The construction algorithm for FSTs will
find these prefixes and suffixes automatically, and it will try reusing saved
prefixes and suffixes wherever possible.

FSTs have several important constraints, two of which are most revelant here:

1. The values in the FST must satisfy a specific algebra. Rust's `fst` crate
   satisfies this algebra by requiring that values are 64-bit integers. So to
   put it simply, we must be able to convert whatever information we need to
   store into a 64-bit integer.

2. The keys in the FST must all be unique. But Sanskrit words are not
   necessarily unique, and we must to be able to store duplicate keys.

[slp1]: https://en.wikipedia.org/wiki/SLP1
[wx]: https://en.wikipedia.org/wiki/WX_notation
[trie]: https://en.wikipedia.org/wiki/Trie
[fst-wiki]: https://en.wikipedia.org/wiki/Finite-state_transducer
[fst-crate]: https://docs.rs/fst/latest/fst/index.html
[fst-blog]: https://blog.burntsushi.net/transducers/
[fst-blog-section]: https://blog.burntsushi.net/transducers/#finite-state-machines-as-data-structures


### Solving the encoding constraint

For the use case of an inflectional dictionary, we want to store two kinds of
information: *enumerated data* and *text data*.

**Enumerated data** includes categories like person, number, gender, case, and
so on. Each category can take one of several possible values that we know ahead
of time. We can trivially convert such data to an integer value. For example,
we represent the *vacana* (number) of a word with one of four possible values:
`eka`, `dvi`, `bahu`, or `none` if the *vacana* is unknown for some reason.
Thus we can map `eka` to 1, `dvi` to 2, and so on.

**Text data** includes the stem or root of the underlying form, and we cannot
feasibly enumerate it ahead of time. We encode this data through a lookup table
approach: if we append all strings to a list, then the index of that string in
the list is its integer representation. By using this approach, we pay the
price of storing these strings the old-fashioned way. But a list of lemmas is
*much* smaller than a list of words, so the space requirements are trivial.

Once we have mapped all of our information to integer values, we can treat our
64-bit integer as a bitfield and add values together with the appropriate
shifts. For example, here's an early version of our scheme for *tinantas*,
which uses only 32 bits:

```text
OOLLLLppVVaadddddddddddddddddddd

O: part of speech (2 bits = 4 possible values)
L: lakara (4 bits = 16 possible values)
p: purusha (2 bits = 4 possible values)
V: vacana (2 bits = 4 possible values)
a: pada + prayoga (2 bits = 4 possible values)
d: dhatu ID (20 bits = ~1 million possible values)
```

One real consequence of the encoding constraint is that we can't associate
words with completely arbitrary data. But if our data is structured carefully,
we have plenty of room to associate each word with interesting information.


### Solving the uniqueness constraint

A Sanskrit dictionary is practical only if it support duplicate keys. To solve
the uniqueness constraint, we use the following workaround, which
(surprisingly) works well in practice.

The `fst` crate treats keys as a vector of unsigned 8-bit integers, which means
that each "letter" in the key can take a value in the range [0, 255]. Since the
ASCII range starts at 65, we can encode extra information in the range [0, 64],
which lets us mark duplicates.

For example, we might store the various forms of `gacchati` as follows;

- `gacchati`,
- `gacchati + 0`
- `gacchati + 1`

And so on up to `gacchati + 64` if necessary. This simple approach lets us
store up to 65 duplicates of a given key.

Since even 65 duplicates is not enough in practice, we simply extended the
range of bytes we append to the key:

- `gacchati`,
- `gacchati + 0 0`
- `gacchati + 0 1`,
- `gacchati + 0 2`

And so on up to `gacchati + 64 64` if necessary. This approach lets us store
more than 4000 duplicates per key, which is more than enough for Sanskrit.

At lookup time, we would first check for the query word -- `gacchati`, let's
say -- then sequentially chehck `gacchati + 0 0` and so on until we no longer
find any results. This lookup might seem extremely expensive, but due to how
FSTs are structured, it is surprisingly cheap.
