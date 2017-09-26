# native-signer

### Requirements

- `node.js` (tested on `v7.4.0`)
- `npm` (tested on `5.0.0`) **does not work with npm 5.1+, see [#75](https://github.com/paritytech/parity-signer/issues/75)**
- `rustup` (tested on `rustup 1.0.0 (17b6d21 2016-12-15)`)
- `rustc` (tested on `rustc 1.19.0 (0ade33941 2017-07-17)`)
- `cargo` (tested on `cargo 0.20.0 (a60d185c8 2017-07-13)`)
- `android_ndk` (tested on `r13b`)
- `Xcode` (only, for iOS, tested on `Version 8.1 (8B62)`)
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
