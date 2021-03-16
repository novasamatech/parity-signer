# Parity Signer - Turn your smartphone into a hardware wallet

Parity Signer is a mobile crypto wallet. You can create Substrate and
Ethereum accounts, sign messages/transactions, and transfer funds to
and from these accounts.

Parity Signer is [GPL 3.0 licensed](LICENSE).

### Requirements

- `node.js` ( `>=10`)
- `yarn` (tested on `1.6.0`)

#### iOS
- `cocoapods` (`$ sudo gem install cocoapods`)
- `Xcode` (only for iOS, tested on `Version 11.3.1 (9F2000)`)

#### Android
- `Android Studio` (only for Android, tested on `Version 3.3`)
- `$JAVA_HOME` envarionment variable set to java home directory (eg. `/Library/Java/JavaVirtualMachines/jdk1.8.0_60.jdk/Contents/Home`)
- `$ANDROID_HOME` environment variable set to Android SDK directory (eg. `/home/your_username/Android/Sdk`)*.

\* It's recommended to install **Android Studio** and use that to install the necessary build tools and SDKs for the Android version you want to test on. It's also the best way to test in the emulator.

### Setup

#### iOS
- Install Dependencies

    ```
    yarn install:ios
    ```
#### Android
- Install Dependencies

    ```
    yarn install
    ```

#### Any system
- Generate an Android Debug Key if it is first time.

    ```
    ./scripts/gen_key.sh
    ```

### Develop

- Start React Native server

    ```
    yarn start
    ```

Then:

- iOS

    ```
    yarn ios
    ```

- Android

    ```
    yarn android
    ```

---

# Building and Publishing

In this page we will list the steps to publish the app on iOS App Store / Android Play Store.

## iOS App Store

##### Build production version

1. Switch to the `master` branch and install the dependencies with `yarn install:ios`
3. Build the JS bundle with `yarn build-prod:ios`
4. Start the react-native server with `yarn start`
5. Build the project in Xcode, with `Generic iOS Device` selected on the top left

##### Upload to iOS Store

1. On Xcode, Choose `Product -> Archieve`
2. In the following menu, click `Validate App`.
3. Click 'Distribute App' for uploading to Test Flight.
4. Then it should be seen on Test Flight on [Apple Connect](https://appstoreconnect.apple.com/), after it is approved, it could be used with the internal test team.

## Android Play Store

1. `yarn install` to install all the related dependencies.
3. `cd android && ./gradlew assembleRelease` for generating unsigned apk.
4. Sign the apk with keystore file
5. In Google Play Console -> Choose the App -> Release Management on Left Panel -> App Release on Left Panel -> Production track -> Manage -> Create Release -> Add the signed apk file here.


# Changelog

## Breaking Changes on Building

### Changes from 4.4.0

We extract the Rust part of Parity Signer to an independent npm library called [react-native-substrate-sign](https://github.com/paritytech/react-native-substrate-sign). The npm library includes prebuild ios static library (*.a) and android dynamic library (*.so), so that Parity Signer does not need to manually build Rust code anymore. Related to [#649](https://github.com/paritytech/parity-signer/issues/649)

### Changes from 4.3.1

From [4.3.1](https://github.com/paritytech/parity-signer/commit/ea5786c85661d9b176795b9386af640b3e73aff3) we use the latest prebuild NDK (r21) toolchains for building rust libraries for android, so that we do not need to build the standalone NDK toolchains manually. If you have built or develop Parity Signer before 4.3.1, please download the NDK r19 or newer[here](https://developer.android.com/ndk/downloads) and point the `NKD_HOME` environment variable to it with e.g. `export NDK_HOME=/path/to/latest/ndk`


# Troubleshooting

## "No dimension set for key window" on Android < 5.0

This error should be accompanied with `error: closed` in terminal when deploying the debug version of the signer on a device that runs Android older than 5.0. It happens because the Android API does not support the reverse proxy that would allow the phone to communicate with the debug server on your computer.

A suitable workaround is to run both devices on the same WiFi and use your local WiFi IP address. Check your WiFi settings for your local IP address (eg. `192.168.1.42`), then, while having the app open on the phone (either on error page or blank screen) run a command in terminal:

```
adb shell input keyevent 82
```

(You can find `adb` binary in your local Android SDK folder under `platform-tools`, eg. `/home/your_username/Android/Sdk/platform-tools`)

This should open a menu on the device. In that menu go to `Dev Settings` > `Debug server host & port for device`, and enter your local IP address with port 8081 (eg. `192.168.1.42:8081`). Restart the app, the error should disappear.

## Can't build the rust part or start `./scripts/init.sh`
If shell has this error:
```shelll
error: component 'rust-std' for target 'armv7-apple-ios' is unavailable for download for channel stable
```
Please switch rustup toolchains to version `1.41.1` with `rustup default 1.41.1` since from 1.42.0 rust [dropped 32-bit apple target support](https://blog.rust-lang.org/2020/01/03/reducing-support-for-32-bit-apple-targets.html)), the latest version for building ios target is 1.41.1.

## Can't find NDK with `yarn android`

## Changes from 4.3.1

From [4.3.1](https://github.com/paritytech/parity-signer/commit/ea5786c85661d9b176795b9386af640b3e73aff3) we use the latest prebuild NDK (r21) toolchains for building rust libraries for android, so that we do not need to build the standalone NDK toolchains manually. If you have built or develop Parity Signer before 4.3.1, please download the NDK r19 or newer [here](https://developer.android.com/ndk/downloads) and point the `NKD_HOME` environment variable to it with e.g. `export NDK_HOME=/path/to/latest/ndk`

## Cannot run after upgrade to latest codebase

1. `yarn clean`
2. `yarn install`
3. `cd ios && pod install && cd ..`
4. delete app on device
5. `yarn start --reset-cache`

#### build on iOS
6. in Xcode (be sure to open with `./ios/NativeSigner.xcodeworkspace` file), clean build with `shift + command + K`
7. `yarn run ios`

#### build on Android
6. clean build with `cd android && ./gradlew clean && cd ..`
7. `yarn run android`

## E2E Tests fail or timeout unexpectedly
1. erase simulator data on iOS Simulator with `Device` -> `Erase all content and settings`
2. `yarn clean && yarn`
3. `yarn start --reset-cache`
4. build & run as above


# Update Network Tutorial

Parity Signer support adding a new Substrate based network or update the existing network via QR code.

This tutorial will walk through how to add a new Rococo Network with Polkadot.js App.

## 1. Get the network metadata as QR Code

Switch to the network you want to play with on Polkadot.js app. Click `Settings` -> `MetaData`

![Network Metadata QR Code](images/Network-Metadata-QR.png)

Here we can see the chain specifications like `Network Name`, `Address Prefix`, and `Genesis Hash` etc. They are all the metaData of the network which is required by Parity Signer. The only item we could change is network color, it is used on Parity Signer to distinguish other networks.

On the right side is the QR Code we need.

## 2. Scan metadata QR code with Parity Signer

Now on the Parity Signer app, click the QR scanner Button anywhere on the app, and scan this QR code, you will have the new Network added to Parity Signer. You can now create accounts under it and sign extrinsic with this network.

![Network Metadata Added on Parity Signer](images/Network-Metadata-Added.png)

Notice since the metadata is generally very big data, and currently, it is hard to sync with Parity Signer, so when signing the transactions on added networks, we cannot interpreter the extrinsic details at the moment. Please check on this [issue](https://github.com/paritytech/parity-signer/issues/457) for the update.
