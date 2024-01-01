Data
====

We have designed `vidyut-chandas` so that it can run without any side data. But
in practice, it is useful to have a list of meters available already. For this
reason, the `vidyut-chandas` crate includes `meters.tsv`, which you can use to
get started.


Creating the data file
----------------------

`meters.tsv` was sourced from the [Sanskrit metres][s-m] project by Shreevatsa
Rajagopalan, which itself sources its data from a transcription of the
*Vṛttaratnākara* prepared by Dr. Dhaval Patel. Our version of this data
contains only *vṛtta* meters and does not currently support *jāti* meters.

We extracted this data using `extract_meter_data.py`, which you can find in the
`scripts` directory of this crate.

[s-m]: https://github.com/shreevatsa/sanskrit
