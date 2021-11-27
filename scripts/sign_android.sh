#!/usr/bin/env bash
set -e

keystore="$(readlink -f "$1")"
keypass="$2"

# Get latest android-sdk-linux version for apksigner path
ANDROID_BUILD_TOOLS_PATH=$(find /opt/android-sdk-linux/build-tools/ -maxdepth 1 -type d | sort -V | tail -n 1)

pushd "$(dirname "${0}")"/../android
  echo "[+] Signing bundle"
  "$ANDROID_BUILD_TOOLS_PATH/apksigner" sign --ks "$keystore" --ks-pass "pass:$keypass" app/build/outputs/apk/release/app-release-unsigned.apk
  cp app/build/outputs/apk/release/app-release-unsigned.apk ../signer-ci-build.apk
popd
