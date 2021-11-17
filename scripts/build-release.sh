#!/usr/bin/env bash

keystore="$(readlink -f "$1")"
keypass="$2"

cat "$keypass" | wc

# Get latest android-sdk-linux version for apksigner path
ANDROID_BUILD_TOOLS_PATH=$(find /opt/android-sdk-linux/build-tools/ -maxdepth 1 -type d | sort -V | tail -n 1)

set -e

# Build rust lib
pushd "$(dirname "${0}")"/..
  ./scripts/build.sh android
popd

pushd "$(dirname "${0}")"/../android
  echo "[+] Running Gradle"
  ./gradlew assembleRelease
  echo "[+] Build complete! Signing bundle"
  "$ANDROID_BUILD_TOOLS_PATH/apksigner" sign --ks "$keystore" --ks-pass "pass:$keypass" app/build/outputs/apk/release/app-release-unsigned.apk
  cp app/build/outputs/apk/release/app-release-unsigned.apk ../signer-ci-build.apk
popd
