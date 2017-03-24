#! /bin/bash

set -e

# ios
multirust add-target stable i386-apple-ios
multirust add-target stable x86_64-apple-ios
multirust add-target stable armv7-apple-ios
multirust add-target stable armv7s-apple-ios
multirust add-target stable aarch64-apple-ios

# android
multirust add-target stable aarch64-linux-android
multirust add-target stable armv7-linux-androideabi
multirust add-target stable i686-linux-android

./create-ndk-standalone.sh

npm i
