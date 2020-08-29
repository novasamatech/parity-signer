# Changelog

## Breaking Changes on Building

### Changes from 4.4.0

We extract the Rust part of Parity Signer to an independent npm library called [react-native-substrate-sign](https://github.com/paritytech/react-native-substrate-sign). The npm library includes prebuild ios static library (*.a) and android dynamic library (*.so), so that Parity Signer does not need to manually build Rust code anymore. Related to [#649](https://github.com/paritytech/parity-signer/issues/649)

### Changes from 4.3.1

From [4.3.1](https://github.com/paritytech/parity-signer/commit/ea5786c85661d9b176795b9386af640b3e73aff3) we use the latest prebuild NDK (r21) toolchains for building rust libraries for android, so that we do not need to build the standalone NDK toolchains manually. If you have built or develop Parity Signer before 4.3.1, please download the NDK r19 or newer[here](https://developer.android.com/ndk/downloads) and point the `NKD_HOME` environment variable to it with e.g. `export NDK_HOME=/path/to/latest/ndk`
