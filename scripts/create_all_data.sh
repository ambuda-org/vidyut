#!/usr/bin/env bash

# Create all of the linguistic data necessary for general usage.


# Exit if any step in this install script fails.
set -e

# Clean up temporary files, if they exist.
rm -rf data-git
rm -rf dcs-data

# Create necessary directories.
mkdir -p "data/build/${1}"


echo -e "
=========================
| DCS corpus data       |
=========================
"

if [[ -e "data/raw/dcs" ]]; then
    echo "Training data already exists -- skipping fetch."
else
    echo "Training data does not exist -- fetching."

    mkdir -p "data/raw/dcs"
    git clone --depth 1 https://github.com/OliverHellwig/sanskrit.git dcs-data

    mv dcs-data/dcs/data/conllu data/raw/dcs/conllu
    rm -rf dcs-data
fi


echo -e "
=========================
| Linguistic data fetch |
=========================
"

if [[ -e "data/raw/lex" ]]; then
    echo "Lexical data already exists -- skipping fetch."
else
    echo "Lexical data does not exist -- fetching."

    mkdir -p "data/raw/lex"
    git clone --depth=1 https://github.com/sanskrit/data.git data-git

    python3 data-git/bin/make_data.py --make_prefixed_verbals
    mv data-git/all-data/* data/raw/lex

    rm -rf data-git
fi


echo -e "
=========================
| Vidyut build          |
=========================
"

if [[ "$1" == "" ]]; then
    make_cmd="make  -j`nproc`"
else
    make_cmd=$1
fi

$make_cmd create_kosha
$make_cmd test_kosha
$make_cmd create_sandhi_rules
$make_cmd train_cheda
$make_cmd eval_cheda


echo -e "\nComplete."
