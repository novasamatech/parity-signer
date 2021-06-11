#!/bin/bash

source ./variables.sh

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
