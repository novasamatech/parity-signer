# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

clean:
    #!/usr/bin/env bash
    rm -rf ios/build
    cd rust
    cargo clean

_build_ios:
    #!/usr/bin/env bash
    cd scripts
    ./build.sh ios

# build all
build: _build_ios

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
zip:
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
checksum:
    #!/usr/bin/env bash
    BASE_PATH=./ios/build/NativeSigner/Build/Products/Release-iphoneos
    pushd $BASE_PATH
    for checksum in *.sha256; do
        shasum -c $checksum
    done
    popd

# Open artifacts folder
open:
    #!/usr/bin/env bash
    BASE_PATH=./ios/build/NativeSigner/Build/Products/Release-iphoneos
    open $BASE_PATH

# Full programm excluding the bump
release: clean build zip checksum open
