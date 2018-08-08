# Parity Signer

[![App Store][app-store-badge]][app-store-url]
[![Google Play][google-play-badge]][google-play-url]

[app-store-badge]: ./res/app-store-badge.png
[app-store-url]: https://itunes.apple.com/us/app/parity-signer/id1218174838
[google-play-badge]: ./res/google-play-badge.png
[google-play-url]: https://play.google.com/store/apps/details?id=com.nativesigner

### Requirements

- `node.js` (tested on `v8.4.0`)
- `yarn` (tested on `1.6.0`)
- `rustup` (tested on `rustup 1.5.0 (92d0d1e9e 2017-06-24)`)
- `rustc` (tested on `rustc 1.27.0 (3eda71b00 2018-06-19)`)
- `cargo` (tested on `cargo 1.27.0 (1e95190e5 2018-05-27)`)
- `android_ndk` (tested on `r13b`)
- `Xcode` (only, for iOS, tested on `Version 9.4.1 (9F2000)`)
- `$NDK_HOME` envarionment variable set to ndk home directory (eg. `/usr/local/opt/android-ndk`)
- `$JAVA_HOME` envarionment variable set to java home directory (eg. `/Library/Java/JavaVirtualMachines/jdk1.8.0_60.jdk/Contents/Home`)

### setup

- macOS

    ```
    ./setup_macos.sh

    echo "ndk.dir=$NDK_HOME" > android/local.properties
    echo "sdk.dir=$ANDROID_HOME" >> android/local.properties
    ```

- linux

    ```
    ./setup_linux.sh

    echo "ndk.dir=$NDK_HOME" > android/local.properties
    echo "sdk.dir=$ANDROID_HOME" >> android/local.properties
    ```

### usage

- iOS

    ```
    npm run ios
    ```

- Android

    ```
    npm run android
    ```

### Example

#### Create new account

seed: `this is sparta`

address: `006E27B6A72E1f34C626762F3C4761547Aff1421`

#### Scan qr code


qr:

[![qr][tx_qr]]

data:

```json
{
  "action":"signTransaction",
  "data":
  {
    "rlp":"f85f800182520894095e7baea6a6c7c4c2dfeb977efac326af552d870a801ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804",
    "account":"006E27B6A72E1f34C626762F3C4761547Aff1421"
  }
}
```

[tx_qr]: ./docs/tx_qr.png
