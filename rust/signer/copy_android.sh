#! /bin/bash

mkdir -p ../../android/app/src/main/jniLibs
mkdir -p ../../android/app/src/main/jniLibs/x86
mkdir -p ../../android/app/src/main/jniLibs/arm64-v8a
mkdir -p ../../android/app/src/main/jniLibs/armeabi-v7a

cp ./target/i686-linux-android/release/libsigner.so ../../android/app/src/main/jniLibs/x86/libsigner.so
cp ./target/aarch64-linux-android/release/libsigner.so ../../android/app/src/main/jniLibs/arm64-v8a/libsigner.so
cp ./target/armv7-linux-androideabi/release/libsigner.so ../../android/app/src/main/jniLibs/armeabi-v7a/libsigner.so
