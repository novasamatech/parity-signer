#!/bin/bash
set -e
source $HOME/.cargo/env

LIB_NAME=signer
IOS_VERSION=15.8.1

# Validate the input argument
if [ "$1" == "device" ]; then
    ARM_ARCH=aarch64-apple-ios
    echo "Building for iOS Device (x86_64-apple-ios and aarch64-apple-ios)"
elif [ "$1" == "simulator" ]; then
    ARM_ARCH=aarch64-apple-ios-sim
    echo "Building for iOS Simulator (x86_64-apple-ios and aarch64-apple-ios-sim)"
else
    echo "Usage: $0 [device|simulator]"
    exit 1
fi

IOS_ARCHS=(x86_64-apple-ios ${ARM_ARCH})

# XCode tries to be helpful and overwrites the PATH. Reset that.
PATH="$(bash -l -c 'echo $PATH')"

cd "$(dirname "${0}")/../../rust/signer"

for i in "${IOS_ARCHS[@]}";
do
  rustup target add "$i";
  env -i PATH="${PATH}" IPHONEOS_DEPLOYMENT_TARGET="${IOS_VERSION}" \
  RUSTFLAGS="-C link-arg=-mios-version-min=${IOS_VERSION}"
  "${HOME}"/.cargo/bin/cargo build --locked --target "$i" --release --no-default-features
done

lipo -create -output "../../ios/PolkadotVault/lib${LIB_NAME}.a" ../target/x86_64-apple-ios/release/libsigner.a ../target/${ARM_ARCH}/release/libsigner.a
lipo -create -output "lib${LIB_NAME}.a" ../target/x86_64-apple-ios/release/libsigner.a ../target/${ARM_ARCH}/release/libsigner.a