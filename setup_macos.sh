#! /bin/bash

set -e

# ios
rustup target add i386-apple-ios
rustup target add x86_64-apple-ios
rustup target add armv7-apple-ios
rustup target add armv7s-apple-ios
rustup target add aarch64-apple-ios

# android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android

./create-ndk-standalone.sh

yarn
