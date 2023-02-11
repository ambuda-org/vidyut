#!/usr/bin/env sh

# Create all of the linguistic data necessary for general usage.

# Clean up temporary files, if they exist.
rm -Rf data-git 2&> /dev/null
rm -Rf dcs-data 2&> /dev/null

# Exit if any step in this install script fails.
set -e

# Create necessary directories.
mkdir -p "data/build/${1}"

echo "========================="
echo "| DCS corpus data       |"
echo "========================="
echo
if [ -e "data/raw/dcs" ]; then
    echo "Training data already exists -- skipping fetch."
else
    echo "Training data does not exist -- fetching."
    mkdir -p "data/raw/dcs"
    git clone --depth 1 https://github.com/OliverHellwig/sanskrit.git dcs-data
    mv dcs-data/dcs/data/conllu data/raw/dcs/conllu
    rm -Rf dcs-data
fi
echo
echo "========================="
echo "| Linguistic data fetch |"
echo "========================="
echo
if [ -e "data/raw/lex" ]; then
    echo "Lexical data already exists -- skipping fetch."
else
    echo "Lexical data does not exist -- fetching."
    mkdir -p "data/raw/lex"
    git clone --depth=1 https://github.com/sanskrit/data.git data-git
    python3 data-git/bin/make_data.py --make_prefixed_verbals
    mv data-git/all-data/* data/raw/lex
    rm -rf data-git
fi
echo
echo "========================="
echo "| Vidyut build          |"
echo "========================="
echo
make create_kosha
make test_kosha
make create_sandhi_rules
make train_cheda
make eval_cheda
echo
echo "Complete."
