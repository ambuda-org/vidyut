#!/usr/bin/env sh
if [[ ! $(command -v wasm-pack) ]]
then
    echo "Our debugger requires wasm-pack. Please install wasm-pack:"
    echo "https://rustwasm.github.io/wasm-pack/installer/"
    echo
    exit 1
fi

# `cargo` uses the debug build by default, but `wasm-pack` uses the release
# build by default instead. Creating this release build is slow, but the debug
# build seems to have issues with enum parsing. So, stick with the release
# build.
wasm-pack build --target web --release
mkdir -p www/static/data && cp data/*.tsv www/static/data
mkdir -p www/static/wasm && cp pkg/* www/static/wasm
cd www && python3 -m http.server
