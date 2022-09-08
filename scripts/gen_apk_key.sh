#!/bin/bash

rm -rf android/apk.keystore

keytool -genkey -noprompt \
 -alias testing \
 -keysize 2048 \
 -keyalg RSA \
 -dname "CN=mytestingapk.com, OU=ID, O=UNKNOWN, L=UNKNOWN, S=UNKNOWN, C=GB" \
 -keystore android/apk.keystore \
 -storepass testing \
 -keypass testing
