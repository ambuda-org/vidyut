#!/usr/bin/env sh
if [[ ! $(command -v wasm-pack) ]]
then
    echo "Our debugger requires wasm-pack. Please install wasm-pack:"
    echo "https://rustwasm.github.io/wasm-pack/installer/"
    echo
    exit 1
fi

# `cargo` uses the debug build by default, but `wasm-pack` uses the release
# build by default instead. Creating this release build is slow, so instead
# explicitly use the debug build by passing the `--debug` flag.
wasm-pack build --target web --debug
mkdir -p www/static/wasm && cp pkg/* www/static/wasm
mkdir -p www/static/data && cp data/* www/static/data
cd www \
    && python3 -m venv env \
    && . env/bin/activate \
    && pip3 install -r requirements.txt \
    && python app.py

