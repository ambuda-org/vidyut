Vidyut
======

> मा भूदेवं क्षणमपि च ते विद्युता विप्रयोगः ॥ 

Vidyut is a lightning-fast toolkit for processing Sanskrit text. Vidyut aims to
provide standard components that are fast, memory-efficient, and competitive
with the state of the art.

Vidyut compiles to native code and can be bound to your language of choice. As
part of our work on [Ambuda][ambuda], we provide first-class support for Python
bindings through [vidyut-py][vidyut-py].

Vidyut is currently **experimental** code, and its API is not stable. If you
wish to use Vidyut for your production use case, please [file an issue][issues]
first.

[![Build status](https://github.com/ambuda-org/vidyut/workflows/ci/badge.svg)](https://github.com/ambuda-org/vidyut/actions)

[ambuda]: https://ambuda.org
[vidyut-py]: https://github.com/ambuda-org/vidyut-py
[issues]: https://github.com/ambuda-org/vidyut/issues


Components
----------

Vidyut currently contains two major components.


### Lexicon

`Lexicon` maps Sanskrit words to their semantics with high performance and
minimal memory usage. In one recent test, we were able to store 29.5 million
inflected Sanskrit words in 31 megabytes of disk space for a total cost of
around 1 byte per word, and we were able to retrieve these words at around
820 ns/word, as compared to 530 ns/word for a standard in-memory hash map.

`Lexicon`'s underlying data structure is a [finite-state transducer][fst-wiki],
as implemented in the [fst][fst-crate] crate. The one downside to an FST is
that we must construct it ahead of time and cannot add new keys to it once it
has been created. But since the Sanskrit word list is largely stable, this is a
minor concern.

[fst-wiki]: https://en.wikipedia.org/wiki/Finite-state_transducer
[fst-crate]: https://docs.rs/fst/latest/fst/index.html


### Segmenter

`Segmenter` performs a *padaccheda* on a Sanskrit phrase and annotates each
*pada* with its basic morphological information.

`Segmenter` is not yet competitive with other options, but we are optimistic
that we can improve it over time. What is quite special, however, is its sheer
speed: `Segmenter` can process a shloka in under 10 milliseconds, and we expect
it to become even faster in the future.


Usage
-----

As mentioned above, Vidyut is currently **experimental** code, and its API is
not stable. If you wish to use Vidyut for your production use case, please
[file an issue][issues] first.

In addition, we encourage you to join the `#nlp` channel on [Ambuda's Discord
server][discord], where you can chat directly with the development team and get
fast answers to your questions.

Occasional discussion related to Vidyut might also appear on
[ambuda-discuss][ambuda-discuss] or on standard mailing lists like
[sanskrit-programmers][sanskrit-programmers].

[discord]: https://discord.gg/7rGdTyWY7Z
[ambuda-discuss]: https://groups.google.com/g/ambuda-discuss
[sanskrit-programmers]: https://groups.google.com/g/sanskrit-programmers


Development
-----------

Build the code and fetch our linguistic data:

    make install

Run a simple evaluation script:

    make eval

Run unit tests:

    make test

Profile overall runtime and memory usage:

    make profile-general

Profile runtime per function:

    make target=time profile-target-osx

Profile memory allocations:

    make target=alloc profile-target-osx
