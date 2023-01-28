<div align="center">
<h1><code>vidyut-cheda</code></h1>
<p><i>A fast Sanskrit segmenter</i></p>
</div>

`vidyut-cheda` segments Sanskrit expressions into words then annotates those
words with their morphological data. Our segmenter is optimized for real-time
and interactive usage: it is fast, low-memory, and capably handles pathological
input.

For your convenience, `vidyut-cheda` contains helper scripts that will
automatically train an interesting model. See the [Usage](#usage) section
below for details.

This [crate][crate] is under active development as part of the [Ambuda][ambuda]
project. If you enjoy our work and wish to contribute to it, we encourage you
to [join our Discord server][discord], where you can meet other Sanskrit
programmers and enthusiasts.

- [Overview](#overview)
- [Usage](#usage)
- [Design](#design)
- [Data](#data)

[crate]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[ambuda]: https://ambuda.org
[discord]: https://discord.gg/7rGdTyWY7Z


Overview
--------

In many Sanskrit texts, words are often written continuously with minimal use
of spaces or punctuation. In addition, Sanskrit's regular sound changes, called
*sandhi*, can alter the sounds at the beginning and end of individual words.
As a result, both readers and software can have trouble understanding where one
word ends and another begins.

`vidyut-cheda` is a fast segmenter that also performs basic part of speech
tagging. It has three distinguishing qualities:

1. *Precision*. Our segmenter cares about getting words *exactly* right and will
   reject words that might look acceptable at first glance. This precision is a
   good fit for the problems we care about, but it might not be right for all
   applications.

2. *Speed*. On my laptop (a 2.4GHz 8-core CPU with 64 GB of DDR4 RAM), this
   crate can process roughly 100 shlokas a second. We have not yet done deep
   optimization work on this crate, and we expect that it can be even faster in
   the future.

3. *Portability*. Our segmenter currently uses around 50 MB of memory, less
   than a tenth of the memory used by comparable solutions. `vidyut-cheda` has
   a fast cold start and is thus well suited for use in web servers or embedded
   applications.


Usage
-----

```rust,no_run
use vidyut_cheda::{Config, Chedaka, Error};

let data_dir = "path/to/your/vidyut-data";
let config = Config::new(&data_dir);
let chedaka = Chedaka::new(config)?;

let words = chedaka.run("Darmakzetre kurukzetre samavetA yuyutsavaH .")?;
for word in words {
    println!("{}:  {}, {:?}", word.text(), word.lemma(), word.info());
}

# Ok::<(), Error>(())
```


Design
------

We wish to find a sequence of words that maximizes a word sequence `w` that
maximizes the following probability:

```text
P(w[0], w[1], w[2], ..., w[n])
```

To simplify our calculations, we make a Markov assumption. That is, we assume
that the probability of a given word is solely dependent on the word that came
immediately before it:

```text
P(w[0], ..., w[k]) ~= P(w[0], ..., w[k-1]) * P(w[k] | w[k-1])
```

Since there are billions of possible Sanskrit words and little training data,
it is difficult to calculate  the conditional probability above. Therefore, we
estimate the probability of a word using its part of speech tag (`t`) and the
frequency of its underlying lemma (`L`):

```text
P(w[k] | w[k-1]) ~= P(t[k] | t[k-1]) * P(L[k])
```

At runtime, we use a sandhi splitter to exhaustively split the input into two
parts: (1) a candidate word and (2) the remainder of the input. If (1) is a
valid Sanskrit word, we calculate the probability of our overall solution using
the approach described above and push it onto a priority queue, with the best
solutions ranked highest. Once we see a solution with no remaining, we simply
return it.

This approach is very simple, but it works surprisingly well in practice. Once
we have investigated other low-hanging fruit here, we will consider using more
powerful models.

For details, see the comments on `Chedaka`.


Data
----

`vidyut-cheda` depends on two kinds of data: linguistic data and training data.

Our *linguistic data* is an extensive list of Sanskrit words and stems. We use
this list to check whether a given string is a valid Sanskrit word and, if so,
what properties that word has. As of now, our linguistic data comes from the
[sanskrit/data][s-data] repo. But over time, we will make increasing use of
`vidyut-prakriya` to generate the data we need.

Our *training data* is a large corpus of Sanskrit texts that has already been
segmented and tagged. As of now, all of our training data comes from a data
dump of the Digital Corpus of Sanskrit, available [here][dcs-data]. Over time,
however, we will make increasing use of the output we produce on Ambuda with
the help of `vidyut-cheda`.

[s-data]: https://github.com/sanskrit/data
[dcs-data]: https://github.com/OliverHellwig/sanskrit.git
