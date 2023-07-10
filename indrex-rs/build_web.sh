#!/usr/local/bin/bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/"

CRATE_NAME="index-rs-wasm"

OPEN=false
FAST=false

while test $# -gt 0; do
  case "$1" in
    -h|--help)
      echo "build_demo_web.sh [--fast] [--open]"
      echo "  --fast: skip optimization step"
      echo "  --open: open the result in a browser"
      exit 0
      ;;
    --fast)
      shift
      FAST=true
      ;;
    --open)
      shift
      OPEN=true
      ;;
    *)
      break
      ;;
  esac
done

# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/static/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

# Clear output from old stuff:
rm -f static/${CRATE_NAME}_bg.wasm

echo "Building rust…"
BUILD=release

cargo build \
  -p ${CRATE_NAME} \
  --release \
  --lib \
  --target wasm32-unknown-unknown

echo "Generating JS bindings for wasm…"
TARGET_NAME="index_rs_wasm.wasm"
wasm-bindgen "target/wasm32-unknown-unknown/$BUILD/$TARGET_NAME" \
  --out-dir static --no-modules --no-typescript
# wasm-pack build example_eframe --out-dir static

# to get wasm-strip:  apt/brew/dnf install wabt
# wasm-strip static/${CRATE_NAME}_bg.wasm

echo "Finished static/${CRATE_NAME}_bg.wasm"

if [ "${OPEN}" = true ]; then
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux, ex: Fedora
    xdg-open http://fbi.com:8000/public
  elif [[ "$OSTYPE" == "msys" ]]; then
    # Windows
    start http://fbi.com:8000/public
  else
    # Darwin/MacOS, or something else
    open http://fbi.com:8000/public
  fi
fi
