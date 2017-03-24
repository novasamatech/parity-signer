#! /bin/bash

set -e

# android
multirust add-target stable aarch64-linux-android
multirust add-target stable armv7-linux-androideabi
multirust add-target stable i686-linux-android

./create-ndk-standalone.sh

npm i
