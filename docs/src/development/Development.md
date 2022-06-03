# Development

### Requirements

- Rust

#### iOS
- `Xcode` (only for iOS, tested on `Version 11.3.1 (9F2000)`)

#### Android
- `Android Studio` (only for Android, tested on `Version 2020.3 (build number 203.7717.56.2031.7621141)`)

\* It's recommended to install **Android Studio** and use that to install the necessary build tools and SDKs for the Android version you want to test on. It's also the best way to test in the emulator.

### Setup

#### Android
- Generate an Android Debug Key if it is first time.

    ```
    ./scripts/gen_key.sh
    ```
- Export `$NDK_HOME` environment variable (usually somewhere under android-sdk directory; please follow AndroidStudio documentation)

### Develop

1. Build Rust libraries
    ```
    ./scripts/build.sh
    ```
2. Build platform-specific code
