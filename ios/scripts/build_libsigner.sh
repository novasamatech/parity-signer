#!/bin/bash
set -e
source $HOME/.cargo/env

IOS_ARCHS=(aarch64-apple-ios x86_64-apple-ios)
LIB_NAME=signer
IOS_VERSION=18.2

printf "Building iOS targets...";

# XCode tries to be helpful and overwrites the PATH. Reset that.
PATH="$(bash -l -c 'echo $PATH')"

cd "$(dirname "${0}")/../../rust/signer"

for i in "${IOS_ARCHS[@]}";
do
  rustup target add "$i";
  env -i PATH="${PATH}" IPHONEOS_DEPLOYMENT_TARGET="${IOS_VERSION}" \
  "${HOME}"/.cargo/bin/cargo build --locked --target "$i" --release --no-default-features \
  -Z build-std \
  -- -C link-arg=-mios-version-min="${IOS_VERSION}"
done

lipo -create -output "../../ios/PolkadotVault/lib${LIB_NAME}.a" ../target/x86_64-apple-ios/release/libsigner.a ../target/aarch64-apple-ios/release/libsigner.a
lipo -create -output "lib${LIB_NAME}.a" ../target/x86_64-apple-ios/release/libsigner.a ../target/aarch64-apple-ios/release/libsigner.a
