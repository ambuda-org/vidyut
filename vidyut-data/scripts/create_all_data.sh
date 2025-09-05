#!/usr/bin/env bash

# Create all of the linguistic data necessary for general usage.

# Clean up temporary files, if they exist.
rm -Rf data-git 2&> /dev/null
rm -Rf dcs-data 2&> /dev/null

# Exit if any step in this install script fails.
set -e

# Create necessary directories.
OUTPUT_DIR="data/build/vidyut-latest"

echo "========================="
echo "Data fetch"
echo "========================="
echo
if [ -e "data/raw/dcs" ]; then
    echo "Training data already exists -- skipping fetch."
else
    echo "Training data does not exist -- fetching."
    mkdir -p "data/raw/dcs"
    # Use the archive zip style with the full "fixed commit hash"
    # to avoid cloning with a lot of depth.
    # NOTE: This hash needs to be updated regularly (at least annually perhaps)
    FIXED_COMMIT_HASH=1bc281ec4abc41b8d0953b4ac92dc85077c4e610
    curl -o dcs-data.zip -L "https://github.com/OliverHellwig/sanskrit/archive/${FIXED_COMMIT_HASH}.zip"
    if [ "$(uname -o)" = "Darwin" ];
    then
      ## Use ditto on the mac
      ditto -V -x -k --sequesterRsrc dcs-data.zip .
    else
      unzip dcs-data.zip
    fi
    mv "sanskrit-${FIXED_COMMIT_HASH}" dcs-data;
    mv dcs-data/dcs/data/conllu data/raw/dcs/conllu
    rm -Rf dcs-data
fi
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
echo "vidyut-chandas"
echo "========================="
mkdir -p "${OUTPUT_DIR}/chandas"
cp -r ../vidyut-chandas/data/* "${OUTPUT_DIR}/chandas"
echo "Copied files to output dir."
echo
echo "========================="
echo "vidyut-kosha"
echo "========================="
make create_kosha
make test_kosha
echo
echo "========================="
echo "vidyut-lipi"
echo "========================="
echo "(no data files needed)"
echo
echo "========================="
echo "vidyut-prakriya"
echo "========================="
mkdir -p "${OUTPUT_DIR}/prakriya"
cp -r "../vidyut-prakriya/data/" "${OUTPUT_DIR}/prakriya"
echo "Copied files to output dir."
echo
echo "========================="
echo "vidyut-sandhi"
echo "========================="
make create_sandhi_rules
echo
echo "========================="
echo "vidyut-cheda"
echo "========================="
make train_chedaka
make eval_chedaka
echo
echo "Complete."
