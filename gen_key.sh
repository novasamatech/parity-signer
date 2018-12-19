#!/bin/bash

echo $APK_KEYSTORE | base64 -d > android/app/parity.keystore

