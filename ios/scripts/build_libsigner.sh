#!/bin/bash
set -e
source $HOME/.cargo/env

IOS_ARCHS=(aarch64-apple-ios x86_64-apple-ios)
LIB_NAME=signer

printf "Building iOS targets...";

# XCode tries to be helpful and overwrites the PATH. Reset that.
PATH="$(bash -l -c 'echo $PATH')"

cd "$(dirname "${0}")/../../rust/signer"

for i in "${IOS_ARCHS[@]}";
do
  rustup target add "$i";
  env -i PATH="${PATH}" \
  "${HOME}"/.cargo/bin/cargo build --locked --target "$i" --release --no-default-features
done

lipo -create -output "../../ios/NativeSigner/lib${LIB_NAME}.a" ../target/x86_64-apple-ios/release/libsigner.a ../target/aarch64-apple-ios/release/libsigner.a
lipo -create -output "lib${LIB_NAME}.a" ../target/x86_64-apple-ios/release/libsigner.a ../target/aarch64-apple-ios/release/libsigner.a
