#!/bin/bash
set -e
chain="$1"
spec_version="$2"

rust_dir="$(dirname "${0}")/../rust/generate_message"
pushd "$rust_dir" || exit 1
  cargo run restore_defaults
  cargo run load_metadata -a
  cargo run show -database
  echo "[+] Generating UNSIGNED metadata APNG for $chain spec_version $spec_version..."
  cargo run make -qr -crypto none -msgtype load_metadata -payload "sign_me_load_metadata_${chain}V${spec_version}"
popd || exit 1
