#! /bin/bash

set -e

# android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android

./create-ndk-standalone.sh

npm i
