#!/usr/bin/env sh

rm -R data/

# Exit if any step in this install script fails.
set -e

git clone --depth=1 git@github.com:sanskrit/data.git data-git
python data-git/bin/make_data.py --make_prefixed_verbals
mv data-git/all-data data
rm -rf data-git

mkdir -p "data/vidyut-0.1.0"
cp data/sandhi-rules.csv "data/vidyut-0.1.0"

make train
make create_lexicon
RUST_LOG=info cargo run --release --bin vidyut -- "saMskftam"
echo "Vidyut is ready for use."
