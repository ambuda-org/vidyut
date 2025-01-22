Data
====

We have designed `vidyut-prakriya` so that it can run without any side data.
But in practice, it is useful to have side data available to generate words
and show the text of specific rules.

Most of the data files here were sourced from from [ashtadhyayi.com][a-com],
and the author of ashtadhyayi.com has graciously agreed to share thse files
with us under an MIT license.

[a-com]: https://ashtadhyayi.com


`dhatupatha.tsv`
----------------

`dhatupatha.tsv` is a comprehensive Dhatupatha that we also use in our
exhaustive test suite.

This dhatupatha is a superset of traditional dhatupathas from five different
sources:

- the *Siddhāntakaumudī*
- the *Bṛhaddhātukusumākaraḥ*
- the *Mādhavīyadhātuvṛttiḥ*
- the *Kṣīrataraṅgiṇī*
- the *Dhātupradīpaḥ*

The source data is [available on Github][dhatupatha-json] as a JSON file. To
create `dhatupatha.tsv`, we downloaded this file and made the following changes:

1. First, we converted the original JSON file to a TSV file with three columns.
   For our current needs, these fields are sufficient.

2. Next, we added svaras to the dhatus in this list. Many of the dhatus here
   overlap with the dhatus used in [SanskritVerb][sanskrit-verb], a PHP-based
   Paninian generator that heavily inspired `vidyut-prakriya`. Where we could
   find an overlap, we used the svaras provided by SanskritVerb. Otherwise, we
   added svaras through a simple mechanical procedure: *aniṭ* roots have
   *anudātta* on the root vowel, *ātmanepadī* roots without *ṅit* are
   *anudāttet*, and *ubhayapadī* roots without *ñit* are *svaritet*.

3. Finally, we made a few corrections to the dhatus in this list. The original
   JSON file will likely be updated with these corrections, so we have omitted
   the details for now.

[dhatupatha-json]: https://github.com/ashtadhyayi-com/data/blob/master/dhatu/data.txt
[sanskrit-verb]: https://github.com/drdhaval2785/SanskritVerb


Sutra files
-----------

Our sutras are split into the following 5 files:

- `dhatupatha-ganasutras.tsv` contains gana-sutras from the Dhatupatha.
- `linganushasanam.tsv` contains sutras from the Linganushasanam.
- `phit-sutras.tsv` contains sutras from the Phit Sutras.
- `sutrapatha.tsv` contains the Ashtadhyayi.
- `unadipatha.tsv` contains the Unadipatha.

All sutra files are 2-column TSV files with the following columns:

- `code`, a short identifier for this sutra.
- `text`, the sutra text encoded in SLP1.
