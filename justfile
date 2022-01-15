VERSION := `toml get rust/signer/Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"
export TAG:=`toml get cli/Cargo.toml "package.version" | jq -r .`

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
