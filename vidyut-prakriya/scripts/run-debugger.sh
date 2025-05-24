#!/usr/bin/env sh
if [ ! "$(command -v wasm-pack)" ]
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
if [ -z "$DEBUG" ]; then
  echo wasm-pack build --target web --release -- --features serde
  wasm-pack build --target web --release -- --features serde
  export HTTP_PORT=8001
  WWW_DIR=www-release
  mkdir $WWW_DIR; cp www/* $WWW_DIR/
else
  echo wasm-pack build --target web --debug -- --features serde
  wasm-pack build --target web --debug -- --features serde
  export HTTP_PORT=8000
  WWW_DIR=www
fi;
mkdir -p $WWW_DIR/static/wasm && cp pkg/* $WWW_DIR/static/wasm
mkdir -p $WWW_DIR/static/data && cp data/* $WWW_DIR/static/data
cd $WWW_DIR && python3 -m http.server $HTTP_PORT
