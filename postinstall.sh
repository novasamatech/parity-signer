#!/bin/bash

echo $APK_KEYSTORE | base64 -d >> android/app/parity.keystore

curl -O https://bootstrap.pypa.io/get-pip.py && python get-pip.py && pip install awscli

echo "signing apk"
jarsigner -verbose -storepass $(echo $KEYSTORE_PASSWORD | base64 -d) -sigalg SHA1withRSA -digestalg SHA1 -keystore android/app/parity.keystore ./android/app/build/outputs/apk/release/app-release-unsigned.apk  parity-signer-key
mkdir release
zipalign -p 4 ./android/app/build/outputs/apk/release/app-release-unsigned.apk release/signer-app-release.apk

echo "uploading build ${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}} s3"

case "${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}}" in
  (beta|stable|nightly|master)
    export S3_BUCKET=releases.parity.io/signer;
    ;;
  (*)
    export S3_BUCKET=signer-builds;
    ;;
esac

aws s3 sync release s3://$S3_BUCKET/${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}}

