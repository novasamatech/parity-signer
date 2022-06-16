<div align="center">
	
![Logo Black](docs/src/res/logo-black.svg#gh-light-mode-only)	
![Logo White](docs/src/res/logo-white.svg#gh-dark-mode-only)
	
</div>

<div align="center">
    <br><br>
    Air-gapped cold storage for your crypto keys
    <br><br>
</div>

<div align="center">
    <a href="https://github.com/paritytech/parity-signer/releases"><img src="docs/src/res/github-badge.png" width="150"></a> <a href="https://play.google.com/store/apps/details?id=io.parity.signer"><img src="docs/src/res/google-play-badge.png" width="150"></a> <a href="https://itunes.apple.com/us/app/parity-signer/id1218174838"><img src="docs/src/res/app-store-badge.png" width="150"></a><br<br>
</div>

# Introduction

Parity Signer is a mobile application that allows any smartphone to act as an air-gapped crypto wallet. This is also known as "cold storage".

You can create accounts in Substrate-based networks, sign messages/transactions, and transfer funds to and from these accounts without any sort of connectivity enabled on the device.

You must turn off or even physically remove the smartphone's Wifi, Mobile Network, and Bluetooth to ensure that the mobile phone containing these accounts will not be exposed to any online threat. Switching to airplane mode suffices in many cases.

☝️ **Disabling the mobile phone's networking abilities is a requirement for the app to be used as intended, check our [wiki](https://paritytech.github.io/parity-signer/about/Security-And-Privacy.html) for more details.**

Any data transfer from or to the app happens using QR code. By doing so, the most sensitive piece of information, the private keys, will never leave the phone. The Parity Signer mobile app can be used to store any Substrate account, this includes Polkadot (DOT) and Kusama (KSM) networks.

**Available for both iOS and Android.**

![](docs/src/res/screens-for-readme.png)

# Links

- [Official Website](https://parity.io/signer)
- [Docs](https://paritytech.github.io/parity-signer/index.html) — auto-generated documentation for end users
- [App Store](https://itunes.apple.com/us/app/parity-signer/id1218174838), [Google Play](https://play.google.com/store/apps/details?id=io.parity.signer), [Github Releases](https://github.com/paritytech/parity-signer/releases) — public builds
- [Signer Companion](https://parity.link/signer-companion) — web extension to inject keys from Signer and sign transactions through the browser
- [Metadata Portal](https://metadata.parity.io) — QR codes with the latest metadata
- [BananaSplit](https://bs.parity.io) — split your seed phrase for maximum security
- [Legacy: last public release with React Native](https://github.com/paritytech/parity-signer/tree/legacy-4.5.3)

# Features

- Generate and store multiple private keys
- Parse and sign transactions
- Use derived keys to have multiple addresses with a single seed phrase
- Backup and restore your accounts
- View activity log to detect unauthorized access
- Update [metadata](https://metadata.parity.io) without going online
- Add new networks

# How to use

Please read our documentation before using Signer for the first time or before upgrading. It covers the main use-cases such as installing on a new phone, creating keys, upgrading and adding new networks:

👉 https://paritytech.github.io/parity-signer/index.html

To contribute into the documentation use [docs](docs) folder

# Project Structure

Signer is a native app for iOS and Android. Native UI's are written on Swift and Kotlin and built on top of a universal Rust core library, which implements all the logic. Here's a rough folder structure of the project.

- `android` - Android project. Builds by Android Studio automatically
- `docker` - files for CI on gitlab
- `docs` - official documentation. Built and published on each commit
- `ios` - iOS project folder. Read how to build it in the "Build Process" section
- `rust` - backend Rust code. Internals are listed below
- `scripts` - mostly releasing scripts and `./build.sh` required for building iOS library

Since most of the application logic is concentrated in the `rust` folder, it makes sense to review it separately.

There are 3 actual endpoints in `rust` folder: `signer`, which is source of library used for Signer itself; `generate_message`, which is used to update Signer repo with new built-in network information and to generate over-the-airgap updates; and `qr_reader_pc` which is a minimalistic app to parse qr codes that we had to write since there was no reasonably working alternative.

Sub-folders of the `rust` folder:

- `constants` — constant values defined for the whole workspace.
- 🔥 `db_handling` — all database-related operations for Signer and generate_message tool. Most of the business logic is contained here.
- `defaults` — built-in and test data for database
- `definitions` — objects used across the workspace are defined here
- `files` — contains test files and is used for build and update generation processes. Most contents are gitignored.
- `generate_message` — tool to generate over-the-airgap updates and maintain network info database on hot side
- 🔥 `navigator` — navigation for Signer app; it is realized in rust to unify app behavior across the platforms
- `parser` - parses signable transactions. This is internal logic for transaction_parsing that is used when signable transaction is identified, but it could be used as a standalone lib for the same purpose.
- `printing_balance` — small lib to render tokens with proper units
- `qr_reader_pc` — small standalone PC app to parse QR codes in Signer ecosystem. Also is capable of parsing multiframe payloads (theoretically, in practice it is not feasible due to PC webcam low performance)
- `qr_reader_phone` — logic to parse QR payloads in Signer
- `qrcode_rtx` — multiframe erasure-encoded payload generator for signer update QR animation.
- `qrcode_static` — generation of static qr codes used all over the qorkspace
- 🔥 `signer` — FFI interface crate to generate bindings that bridge native code and rust backend
- `transaction_parsing` — high-level parser for all QR payloads sent into Signer
- `transaction_signing` — all operations that could be performed when user accepts payload parsed with transaction_parsing

> 🔥 — this emoji means an important folder for the application logic

# Build Process

**1.** First and foremost, make sure you have the latest [Rust](https://www.rust-lang.org/tools/install) installed in your system. Nothing will work without Rust.

If you get errors like `cargo: feature X is required`, it most likely means you have an old version of Rust. Update it by running `rustup update stable`.

**2.** Install `uniffi-bindgen`. Version has to match the version of `uniffi` crates specified
   in the project (currently it is `0.18.0`):

   ```bash
   cargo install uniffi_bindgen --version 0.18.0 
   ```

## iOS

**3.** You probably already have [Xcode](https://developer.apple.com/xcode/) installed if you are reading this. If not, go get it. 

**4.** Compile the core Rust library first:

```
cd scripts && ./build.sh ios
```

**5.** Open the `NativeSigner.xcodeproj` project from the `ios` folder in your Xcode and click Run (Cmd+R).

**6.** The first time you start the app, you will need to put your device into Airplane Mode. In the iOS simulator, you can do this by turning off WiFi on your Mac (yes, this is an official apple-recommended way).

However, we strongly recommend that you use a real device for development, as some important parts (e.g. camera) may not work in the simulator.

## Android

> ⚠️ Android build has only been tested on Linux. If you manage by some miracle to run this on a Mac, please add the steps to this Readme

**3.** Install necessary rust targets (this set may vary depending on the target architecture
   you are building for be it android studio emulators or hardware devices):

   ```bash
    rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
   ```

**4.** Download [Android Studio](https://developer.android.com/studio).

**5.** Open the project from the `android` directory.

**6.** Install NDK. Go to `File -> Project Structure -> SDK Location`. Next to the "Android NDK location" section, click "Download Android NDK" button.

⚠️  We hightly recommend you to update all existing plugins and SDK's for Kotlin, Gradle,
etc even if you just downloaded a fresh Android Studio. It's always a good idea to restart
Android Studio after that. This can save you many hours on Stackoverflow trying to fix
random errors like "NDK not found".

**7.** Connect your device or create a virtual one. Open `Tools -> Device Manager` and create a new phone simulator with the latest Android.

**8.** Run the project (Ctrl+R). It should build the Rust core library automatically.

# Tests

Core Rust code is fully covered by tests and they are run in CI on each commit. To run tests on your machine:

```
cd rust && cargo test --locked
```

We don't have test for UIs for now (other then navigation which is handled on rust side), which means Swift and Kotlin are not covered. We plan to do it in the future.


# Bugs and Feedback

If you found a bug or want to propose an improvement, please open [an issue](https://github.com/paritytech/parity-signer/issues).

Try to create bug reports that are:

- _Reproducible._ Include steps to reproduce the problem.
- _Specific._ Include as much detail as possible: which version, what phone, OS, etc.
- _Unique._ Do not duplicate existing opened issues.
- _Scoped to a Single Bug._ One bug per report.

Official team email for direct inquiries: signer@parity.io

# Contributing

Our contribution guidelines are still in development. Until then, you're welcome to participate in discussions and send PRs with small bugfixes, we'd love it. Each PR must be reviewed by at least two project maintainers.

# License

Parity-Signer is [GPL 3.0 licensed](LICENSE).
