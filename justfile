# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

clean:
    #!/usr/bin/env bash
    rm -rf ios/build
    rm -rf android/app/src/main/assets/Database
    rm -rf ios/NativeSigner/Database
    cd rust
    cargo clean

switch_to_android:
    #!/usr/bin/env bash
    cp rust/os-specific-lock/android/Cargo.lock rust/
    cp rust/signer/android-hack/Cargo.toml rust/signer/

switch_to_ios:
    #!/usr/bin/env bash
    cp rust/os-specific-lock/ios/Cargo.lock rust/
    cp rust/signer/ios-hack/Cargo.toml rust/signer/

build_android:
    #!/usr/bin/env bash
    cd scripts
    ./build.sh android
    cd ..

build_ios:
    #!/usr/bin/env bash
    cd scripts
    ./build.sh ios
    cd ..

# build all - this is probably impossible
#build: build_ios build_android

#do all automatable tests for ios
test_rust_ios: switch_to_ios
    #!/usr/bin/env bash
    cd rust
    cargo test

#do all automatable tests 
test_rust_android: switch_to_android
    #!/usr/bin/env bash
    cd rust
    cargo test


test_rust: test_rust_ios test_rust_android

# Test including ignored tests
test:
    #!/usr/bin/env bash
    cd rust
    cargo test

# Generate documentation
doc:
    #!/usr/bin/env bash
    cd rust
    cargo doc -p --workspace -p signer --all-features --no-deps

bump:
    #!/usr/bin/env bash
    cd ios
    agvtool next-version -all

# zip artifacts
zip_ios:
    #!/usr/bin/env bash
    BASE_PATH=./ios/build/NativeSigner/Build/Products/Release-iphoneos
    pushd $BASE_PATH
    APP=$(ls -d *.app)
    ARCHIVE=$(ls -d *.xcarchive)
    zip -r $APP.zip $APP
    zip -r $ARCHIVE.zip $ARCHIVE
    shasum -a 256 $APP.zip > $APP.zip.sha256
    shasum -a 256 $ARCHIVE.zip> $ARCHIVE.zip.sha256
    du -hd0 *.{app,xcarchive,zip,sha256}
    popd
    # ls -al *.xcarchive
    # ls -al *.zip

# Verify the checksums
checksum_ios:
    #!/usr/bin/env bash
    BASE_PATH=./ios/build/NativeSigner/Build/Products/Release-iphoneos
    pushd $BASE_PATH
    for checksum in *.sha256; do
        shasum -c $checksum
    done
    popd

# Open artifacts folder
open_ios:
    #!/usr/bin/env bash
    BASE_PATH=./ios/build/NativeSigner/Build/Products/Release-iphoneos
    open $BASE_PATH

# Full programm excluding the bump
release_ios: clean build_ios zip_ios checksum_ios open_ios
