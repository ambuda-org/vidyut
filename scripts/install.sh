#!/usr/bin/env sh

# Exit if any step in this install script fails.
set -e

git clone --depth=1 git@github.com:sanskrit/data.git data-git
python data-git/bin/make_data.py --make_prefixed_verbals
mv data-git/all-data data
rm -rf data-git

make train
RUST_LOG=info cargo run -- "saMskftam"
echo "Vidyut is ready for use."
