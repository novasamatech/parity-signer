#!/bin/bash

# Change this name to the rust library name
LIB_NAME=signer
API_LEVEL=29

IOS_ARCHS=(aarch64-apple-ios x86_64-apple-ios) # unsupported: armv7-apple-ios armv7s-apple-ios)
OS_ARCH=$(uname | tr '[:upper:]' '[:lower:]')

