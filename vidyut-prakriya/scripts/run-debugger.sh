#!/usr/bin/env sh
if ! command -v wasm-pack > /dev/null 2<&1
then
    echo "Our debugger requires wasm-pack. Please install wasm-pack:"
    echo "https://rustwasm.github.io/wasm-pack/installer/"
    echo
    exit 1
fi

wasm-pack build --target web
mkdir -p www/static/wasm && cp pkg/* www/static/wasm
mkdir -p www/static/data && cp data/* www/static/data
cd www \
    && python3 -m venv env \
    && . env/bin/activate \
    && pip3 install -r requirements.txt \
    && python app.py

