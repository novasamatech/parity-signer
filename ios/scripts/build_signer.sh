#!/bin/bash
set -e
source $HOME/.cargo/env

IOS_ARCHS=(aarch64-apple-ios x86_64-apple-ios)
LIB_NAME=signer

printf "Building iOS targets...";

# XCode tries to be helpful and overwrites the PATH. Reset that.
PATH="$(bash -l -c 'echo $PATH')"


cd "$(dirname "${0}")/../../rust/signer"

# Loop over each iOS architecture and build the library
LIB_PATHS=()
for i in "${IOS_ARCHS[@]}";
do
  rustup target add "$i";
  env -i PATH="${PATH}" \
  "${HOME}"/.cargo/bin/cargo build --locked --target "$i" --release --no-default-features
  LIB_PATHS+=("../target/${i}/release/lib${LIB_NAME}.a")
done

# Delete the existing XCFramework
printf "Deleting existing XCFramework...\n"
rm -rf "../../ios/Packages/signerFFI/${LIB_NAME}.xcframework"

# Create the universal XCFramework
printf "Creating universal XCFramework...";

mkdir -p "../target/universal"

xcodebuild -create-xcframework \
  -library "../target/aarch64-apple-ios/release/lib${LIB_NAME}.a" \
  -library "../target/x86_64-apple-ios/release/lib${LIB_NAME}.a" \
  -output "../../ios/Packages/signerFFI/$${LIB_NAME}.xcframework"

printf "Build completed successfully!\n";
