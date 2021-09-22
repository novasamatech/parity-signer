#!/bin/bash
set -e

. "$(dirname "${0}")/variables.sh"

cd "$(dirname "${0}")/../rust/signer"

if [ "$1" == "android" ]
  then

    # Build android

    if [ -z ${NDK_HOME+x} ];
      then
        printf 'Please install android-ndk or export NDK_HOME\n\n'
        printf 'from https://developer.android.com/ndk/downloads or with sdkmanager'
        exit 1
      else
        printf "Building Andriod targets...";
    fi

    printf "Building ARM64 Andriod targets...";
    cp android-hack/Cargo.toml .
    CC_aarch64_linux_android="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang" \
    CXX_aarch64_linux_android="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang++" \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang" \
    AR_aarch64_linux_android="${ANDROID_PREBUILD_BIN}/aarch64-linux-android-ar" \
      cargo build --target=aarch64-linux-android --release

    printf "Building ARMv7 Andriod targets...";
    CC_armv7_linux_androideabi="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang" \
    CXX_armv7_linux_androideabi="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang++" \
    CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang" \
    AR_armv7_linux_androideabi="${ANDROID_PREBUILD_BIN}/arm-linux-androideabi-ar" \
      cargo build --target=armv7-linux-androideabi --release

    printf "Building 32-bit x86  Andriod targets...";
    CC_i686_linux_android="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang" \
    CXX_i686_linux_android="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang++" \
    CARGO_TARGET_I686_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang" \
    AR_i686_linux_android="${ANDROID_PREBUILD_BIN}/i686-linux-android-ar" \
      cargo  build --target=i686-linux-android --release
    
    printf "Building 64-bit x86  Andriod targets...";
    CC_x86_64_linux_android="${ANDROID_PREBUILD_BIN}/x86_64-linux-android${API_LEVEL}-clang" \
    CXX_x86_64_linux_android="${ANDROID_PREBUILD_BIN}/x86_64-linux-android${API_LEVEL}-clang++" \
    CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/x86_64-linux-android${API_LEVEL}-clang" \
    AR_x86_64_linux_android="${ANDROID_PREBUILD_BIN}/x86_64-linux-android-ar" \
      cargo  build --target=x86_64-linux-android --release

    for i in "${!ANDROID_ARCHS[@]}";
      do
        mkdir -p -v "../../android/app/src/main/jniLibs/${ANDROID_FOLDER[$i]}"
        cp "./target/${ANDROID_ARCHS[$i]}/release/lib${LIB_NAME}.so" "../../android/app/src/main/jniLibs/${ANDROID_FOLDER[$i]}/lib${LIB_NAME}.so"
    done

fi

if [ "$1" == "ios" ]
  then

    # Build iOS

    printf "Building iOS targets...";
    
    cp ios-hack/Cargo.toml .

    for i in "${IOS_ARCHS[@]}";
      do
        rustup target add "$i";
        cargo build --target "$i" --release --no-default-features
    done

    lipo -create -output "../../ios/NativeSigner/lib${LIB_NAME}.a" target/x86_64-apple-ios/release/libsigner.a target/aarch64-apple-ios/release/libsigner.a
    lipo -create -output "lib${LIB_NAME}.a" target/x86_64-apple-ios/release/libsigner.a target/aarch64-apple-ios/release/libsigner.a
    #unsupported: target/armv7-apple-ios/release/libsigner.a target/armv7s-apple-ios/release/libsigner.a

fi

#echo "hello tom" > read.txt
#cat read.txt
