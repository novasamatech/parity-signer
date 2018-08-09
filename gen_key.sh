#!/bin/bash

keytool -genkey -noprompt \
 -alias testing \
 -keysize 2048 \
 -keyalg RSA \
 -dname "CN=mytestingapk.com, OU=ID, O=UNKNOWN, L=UNKNOWN, S=UNKNOWN, C=GB" \
 -keystore android/app/apk.keystore \
 -storepass testing \
 -keypass testing 
