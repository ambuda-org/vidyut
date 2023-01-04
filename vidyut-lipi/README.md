*vidyut-lipi* is a work-in-progress transliterator. It is not ready for public use.


Usage
-----

```rust
use vidyut_lipi::{Scheme, transliterate};

let result = transliterate("devau", Scheme::Iast, Scheme::Slp1);
assert_eq!(result, "devO");
```

```shell
# Run transliteration
$ cargo run --bin transliterate -- --text rāmau 
```
