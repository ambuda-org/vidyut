Vidyut
======

> मा भूदेवं क्षणमपि च ते विद्युता विप्रयोगः ॥ 

**Vidyut** is an experimental Sanskrit parser that aims to be accurate,
lightweight, and lightning fast. But right now, Vidyut is none of these things.

Our current code is a proof of concept that parses most Sanskrit text with
middling accuracy at a rate of about 4 shlokas a second. Now that we have a
basic scaffold in place, we can iterate on speed, efficiency, and quality.


Usage
-----

```
$ make install
$ cargo run "Darmakzetre kurukzetre samavetA yuyutsavaH" --cache-file cache.bin
```
