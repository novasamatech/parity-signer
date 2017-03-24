# native-signer

### Requirements

- `node.js` (tested on `v7.4.0`)
- `npm` (tested on `4.0.5`)
- `multirust` (tested on `0.8.0 (8654d1c07 2016-03-08 23:02:12 +0000)`)
- `rustc` (tested on `1.15.1 (021bd294c 2017-02-08)`)
- `cargo` (tested on `0.16.0-nightly (6e0c18c 2017-01-27)`)
- `android_ndk` (tested on `r13b`)
- `Xcode` (only, for iOS, tested on `Version 8.1 (8B62)`)
- `$NDK_HOME` envarionment variable set to ndk home directory (eg. `/usr/local/opt/android-ndk`)

### setup

- macOS

    ```
    ./setup_macos.sh
    ```

- linux

    ```
    ./setup_linux.sh
    ```

### usage

- iOS

    ```
    make -C rust/signer ios
    react-native run-ios
    ```

- Android

    ```
    make -C rust/signer android
    react-native run-android
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
	"rlp": "f85f800182520894095e7baea6a6c7c4c2dfeb977efac326af552d870a801ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804",
	"from": "006E27B6A72E1f34C626762F3C4761547Aff1421"
}
```

[tx_qr]: ./docs/tx_qr.png
