#!/bin/bash

source ./scripts/variables.sh

# Build iOS
cd ./rust/signer

cargo lipo --release

cp "./target/universal/release/lib${LIB_NAME}.a" "../../ios/lib${LIB_NAME}.a"

# Build android

if [ -z ${NDK_HOME+x} ];
  then
    printf 'Please install android-ndk\n\n'
    printf 'from https://developer.android.com/ndk/downloads or with sdkmanager'
    exit 1
  else
    printf "Building Andriod targets...";
fi

CC_aarch64_linux_android="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang" \
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang" \
  cargo build --target aarch64-linux-android --release

CC_armv7_linux_androideabi="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang" \
CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang" \
  cargo build --target armv7-linux-androideabi --release

CC_i686_linux_android="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang" \
CARGO_TARGET_I686_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang" \
  cargo  build --target i686-linux-android --release

CC_x86_64_linux_android="${ANDROID_PREBUILD_BIN}/x86_64-linux-android${API_LEVEL}-clang" \
  CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/x86_64-linux-android${API_LEVEL}-clang" \
      cargo  build --target x86_64-linux-android --release

for i in "${!ANDROID_ARCHS[@]}";
  do
    mkdir -p "../android/src/main/${ANDROID_FOLDER[$i]}"
    cp "./target/${ANDROID_ARCHS[$i]}/release/lib${LIB_NAME}.so" "../android/src/main/jniLibs/${ANDROID_FOLDER[$i]}/lib${LIB_NAME}.so"
done
