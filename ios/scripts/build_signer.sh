#!/bin/bash

set -e

# Function to inject or revert the type attribute in Package.swift
# Currently SPM does not allow to properly build XCFramework with xcodebuild
# But if we add .dynamic to Package by default, our project tests won't be compiling
inject_type_attribute() {
  local inject=$1
  local package_swift_path=$2

  if [ "$inject" = true ]; then
     echo "Injecting 'type: .dynamic' into Package.swift..."
     # Check if 'type: .dynamic' is already present
     if ! grep -q "type:\s*.dynamic" "$package_swift_path"; then
       perl -i -p0e 's/(library[^,]*,)/$1 \n\t\t\ttype: .dynamic,/g' "$package_swift_path"
     else
       echo "'type: .dynamic' already present in Package.swift"
     fi
   else
     echo "Reverting 'type: .dynamic' in Package.swift..."
     # Revert the addition of 'type: .dynamic' if present
     perl -i -p0e 's/\n\t\t\ttype: .dynamic,//g' "$package_swift_path"
   fi
}

# Source Rust envs first
source $HOME/.cargo/env

PROJECT_NAME="signer"
MAIN_PROJECT_PATH="./../PolkadotVault.xcodeproj"
PROJECT_DIR="./../Packages/${PROJECT_NAME}"
BUILD_FOLDER="./../../PolkadotVault/.build"
OUTPUT_DIR="./../../PolkadotVault/Frameworks"
SIMULATOR_ARCHIVE_PATH="${OUTPUT_DIR}/${PROJECT_NAME}-iphonesimulator.xcarchive"
DEVICE_ARCHIVE_PATH="${OUTPUT_DIR}/${PROJECT_NAME}-iphoneos.xcarchive"
PACKAGE_SWIFT_PATH="${PROJECT_DIR}/Package.swift"

IOS_ARCHS=(aarch64-apple-ios x86_64-apple-ios)
LIB_NAME=signer




# Inject 'type: .dynamic' into Package.swift
inject_type_attribute true "$PACKAGE_SWIFT_PATH"

# One iteration for device arch and another for simulator arch
for PLATFORM in "iOS" "iOS Simulator"; do
    case $PLATFORM in
      "iOS")
        ARCHIVE_PATH=$DEVICE_ARCHIVE_PATH
        SDK=iphoneos
        ;;
      "iOS Simulator")
        ARCHIVE_PATH=$SIMULATOR_ARCHIVE_PATH
        SDK=iphonesimulator
        ;;
    esac

    echo "Building archive for ${PLATFORM}..."

    # Build archive for given platform
    xcodebuild archive \
      -quiet \
      -scheme $PROJECT_NAME \
      -project $MAIN_PROJECT_PATH \
      -destination="generic/platform=${PLATFORM}" \
      -archivePath $ARCHIVE_PATH \
      -sdk $SDK \
      -derivedDataPath $BUILD_FOLDER \
      SKIP_INSTALL=NO \
      BUILD_LIBRARY_FOR_DISTRIBUTION=YES

    echo "Built archive for ${PLATFORM}."

    FRAMEWORK_PATH="${ARCHIVE_PATH}/Products/Library/Frameworks/${PROJECT_NAME}.framework"
    MODULES_PATH="$FRAMEWORK_PATH/Modules"
    mkdir -p $MODULES_PATH

    echo "FRAMEWORK_PATH: $FRAMEWORK_PATH"
    echo "MODULES_PATH: $MODULES_PATH"

    BUILD_PRODUCTS_PATH="${BUILD_FOLDER}/Build/Intermediates.noindex/ArchiveIntermediates/${PROJECT_NAME}/BuildProductsPath/Release-${SDK}"
    RESOURCES_BUNDLE_PATH="${BUILD_PRODUCTS_PATH}/${PROJECT_NAME}_${PROJECT_NAME}.bundle"

    echo "Copying Swift modules..."

    echo "BUILD_PRODUCTS_PATH: $BUILD_PRODUCTS_PATH"
    echo "RESOURCES_BUNDLE_PATH: $RESOURCES_BUNDLE_PATH"

    # Copy Swift modules
    if [ -d $BUILD_PRODUCTS_PATH ]
      then
        find $BUILD_PRODUCTS_PATH -name "*.swiftmodule" -type d -exec cp -r {} $MODULES_PATH \;
        echo "Copied Swift modules."
      else
        echo "Could not find Swift modules within: $BUILD_PRODUCTS_PATH"
    fi

    echo "Copying resources bundle..."

    # Copy resources bundle, if it exists
    if [ -e $RESOURCES_BUNDLE_PATH ]
    then
      cp -r $RESOURCES_BUNDLE_PATH $FRAMEWORK_PATH
      echo "Copied resources bundle."
    else
      echo "Could not find resources at: $RESOURCES_BUNDLE_PATH"
    fi
done

echo "Creating XCFramework..."

# Create XCFramework
xcodebuild -create-xcframework \
 -framework "${DEVICE_ARCHIVE_PATH}/Products/Library/Frameworks/${PROJECT_NAME}.framework" \
 -framework "${SIMULATOR_ARCHIVE_PATH}/Products/Library/Frameworks/${PROJECT_NAME}.framework" \
 -output "${OUTPUT_DIR}/${PROJECT_NAME}.xcframework"

echo "Created XCFramework."

echo "Cleaning up build artifacts..."

inject_type_attribute false "$PACKAGE_SWIFT_PATH"

# Clean up build artifacts
rm -rf "$BUILD_FOLDER"
rm -rf "$DEVICE_ARCHIVE_PATH"
rm -rf "$SIMULATOR_ARCHIVE_PATH"

echo "Finished build process."

















echo "Starting build process..."

# Clean up any previous build artifacts
echo "Cleaning up previous build artifacts..."
rm -rf "$OUTPUT_DIR"
rm -rf "$BUILD_FOLDER"
mkdir -p "$OUTPUT_DIR"

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
rm -rf "../../ios/Frameworks/${LIB_NAME}.xcframework"

# Create the universal XCFramework
printf "Creating universal XCFramework...";

mkdir -p "../target/universal"

xcodebuild -create-xcframework \
  -library "../target/aarch64-apple-ios/release/lib${LIB_NAME}.a" \
  -library "../target/x86_64-apple-ios/release/lib${LIB_NAME}.a" \
  -output "../../ios/Frameworks/${LIB_NAME}.xcframework"

printf "Build completed successfully!\n";
