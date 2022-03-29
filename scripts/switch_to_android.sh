#!/bin/bash

cd "$(dirname "${0}")/../rust/signer"
cp ../os-specific-lock/android/Cargo.lock ../
cp android-hack/Cargo.toml .


