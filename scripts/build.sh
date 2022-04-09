#!/bin/bash
set -e

. "$(dirname "${0}")/variables.sh"

# Build Signer backend


if [ "$1" != "android" ] && [ "$1" != "ios" ]
then
	printf 'Please select target OS\n'
	printf 'build.sh android\n'
	printf 'or\n'
	printf 'build.sh ios\n'
	exit 1
fi

if [ "$1" == "android" ]
  then
    . "$(dirname "${0}")/switch_to_android.sh"
fi

if [ "$1" == "ios" ]
  then

    # Build iOS

    printf "Building iOS targets...";
    
    cd "$(dirname "${0}")/../rust/signer"

    for i in "${IOS_ARCHS[@]}";
      do
        rustup target add "$i";
        cargo build --locked --target "$i" --release --no-default-features
    done

    lipo -create -output "../../ios/NativeSigner/lib${LIB_NAME}.a" ../target/x86_64-apple-ios/release/libsigner.a ../target/aarch64-apple-ios/release/libsigner.a
    lipo -create -output "lib${LIB_NAME}.a" ../target/x86_64-apple-ios/release/libsigner.a ../target/aarch64-apple-ios/release/libsigner.a
    #unsupported: target/armv7-apple-ios/release/libsigner.a target/armv7s-apple-ios/release/libsigner.a

    # Generate cold release database with built-in metadata

    cd "$(dirname "${0}")/../generate_message"
    cargo run --locked make_cold_release

    # Move database to assets

    rm -rf ../../ios/NativeSigner/Database
    mkdir ../../ios/NativeSigner/Database/
    mkdir ../../ios/NativeSigner/Database/Database/
    cp -R ../database/database_cold_release/ ../../ios/NativeSigner/Database/Database
fi

#echo "hello tom" > read.txt
#cat read.txt
