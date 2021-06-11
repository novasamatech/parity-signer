#!/usr/bin/env bash

keystore="$(readlink -f "$1")"
keyalias=$2
set -e
"$(dirname "${0}")"/build.sh android
pushd "$(dirname "${0}")"/../android
  ./gradlew assembleRelease
  jarsigner -verbose -sigalg SHA1withRSA -digestalg SHA1 -keystore "$keystore" app/build/outputs/apk/release/app-release-unsigned.apk "$keyalias"
  zipalign -p 4 app/build/outputs/apk/release/app-release-unsigned.apk ../signer-app-release-signed.apk
popd
