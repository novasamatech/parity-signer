#!/bin/bash

echo $APK_KEYSTORE | base64 -d >> android/app/parity.keystore

curl -O https://bootstrap.pypa.io/get-pip.py && python get-pip.py && pip install awscli

ech "signing apk"
jarsigner -verbose -storepass $(echo $KEYSTORE_PASSWORD | base64 -d) -sigalg SHA1withRSA -digestalg SHA1 -keystore parity.keystore ./android/app/build/outputs/apk/release/app-release-unsigned.apk  parity-signer-key
mkdir release
zipalign -p 4 ./android/app/build/outputs/apk/release/app-release-unsigned.apk release/signer-app-release.apk

echo "uploading build ${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}} s3"
aws configure set aws_access_key_id $S3_KEY
aws configure set aws_secret_access_key $S3_SECRET

export AWS_ACCESS_KEY_ID="$S3_KEY"

case "${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}}" in
  (beta|stable|nightly|master)
    export S3_BUCKET=releases.parity.io/signer;
    ;;
  (*)
    export S3_BUCKET=signer-builds;
    ;;
esac

aws s3 sync release s3://$S3_BUCKET/${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}}

