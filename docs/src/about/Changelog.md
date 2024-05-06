# Changelog

## 5.0.1

Android version release, minor fixes

## New in version 5.0.0

### Architecture

No more typescript or react native. Backend is completely in Rust, frontend is in native.

### Building

#### Dependencies

Number of dependencies was greatly reduced; no npm/yarn/nodejs/cocoapods, etc. All dependencies are handled by:
 - Cargo (rust packages)
 - Xcode (only default iOS frameworks are used)
 - Gradle

#### Rust backend

Rust libraries were moved back into the repository. Crypto functions are imported from Substrate. All logic and most of storage is written in Rust. An important hack here is that `rust/signer` crate has 2 versions of Cargo.toml for android and iOS architectures, as target library features could not be adjusted by normal means.

#### Native frontend

Frontend for both iOS and Android re-written in native frameworks. Thus, standard out-of-the-box build scripts could be used for building once Rust libraries are built and linked

### Features

#### Secure seed storage

Secrets are stored in devices' encrypted storage and some effort is made to prevent them leaking in system memory. Thus, all is as safe as the phone is - the same credentials used for unlocking the phone are used to unlock seeds. User is responsible to keep them adequate.

#### Transaction preview

Transactions content is shown before signing; no hash signing is allowed, but signing messages is possible.

#### History feature

The Vault now logs all operations it performs. It is important to remember that this is not log of account operations, but log of device history. This history could be cleared if needed, but not modified by other means. Detected presence of network connection is also logged.

#### N+1 derivation

Much requested feature that makes Vault automatically increment numbered seeds on creation.

#### Network and metadata updates

All network data updates now could be performed through scanning QR codes. Whenever some update is needed, most probably you should just scan some QR video. Don't worry about skipped frames, it's fountain code so you only need enough frames.

All updates could be signed, and signing key will be trusted on first use, so Vault device should be linked to single source of authority on correct metadata.

#### Key re-use in different networks

Keys could be used only in one network. Need to re-use key in another network? Just create key with the same derivation path in that network to allow re-use and it will work.

