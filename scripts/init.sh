#!/bin/bash
source ./scripts/variables.sh

cargo install cargo-lipo

case "$(uname | tr '[:upper:]' '[:lower:]')" in
   darwin)
      echo 'Add rust toolchains for android and ios'
      for i in "${IOS_ARCHS[@]}";
        do rustup target add "$i";
      done
      for i in "${ANDROID_ARCHS[@]}";
       do rustup target add "$i" ;
      done
      ;;
   linux)
      echo 'Add rust toolchains for android'
      for i in "${ANDROID_ARCHS[@]}";
       do rustup target add "$i" ;
      done
     ;;
   *)
     echo 'Please use a Linux or Mac to build'
     ;;
esac

FILE=./android/app/debug.keystore
if [ -f "$FILE" ]; then
    echo "$FILE exist, skip.."
    else
    echo "generating andriod debug keystore file.."
    cd ./android/app && keytool -genkey -v -keystore debug.keystore -storepass android -alias androiddebugkey -keypass android -keyalg RSA -keysize 2048 -validity 10000

fi

#cargo install cbindgen

#ln -s ./litentry_rust/rust/target/aarch64-linux-android/release/libexample.so ./litentry_app/android/src/main/arm64-v8a/
#ln -s ./litentry_rust/rust/target/armv7-linux-androideabi/release/libexample.so ./litentry_app/android/src/main/armeabi-v7a/
#ln -s ./litentry_rust/rust/target/i686-linux-android/release/libexample.so ./litentry_app/android/src/main/x86/
#
#ln -s ./litentry_rust/rust/target/universal/release/libexample.a ./litentry_app/ios/




