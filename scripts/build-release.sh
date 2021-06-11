#!/usr/bin/env bash

keystore="$(readlink -f "$1")"
keyalias=$2
keypass_arg="${3:+-storepass $3}"

set -e
echo "[+] Building rust components"
"$(dirname "${0}")"/build.sh android
echo "[+] Moving to android directory to build app..."
yarn install
pushd "$(dirname "${0}")"/../android
  echo "[+] Running Gradle"
  ./gradlew assembleRelease
  echo "[+] Build complete! Signing bundle"
  jarsigner -verbose -sigalg SHA1withRSA -digestalg SHA1 -keystore "$keystore" $keypass_arg app/build/outputs/apk/release/app-release-unsigned.apk "$keyalias"
  zipalign -p 4 app/build/outputs/apk/release/app-release-unsigned.apk ../signer-app-release-signed.apk
popd
