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

From [4.3.1](https://github.com/paritytech/parity-signer/commit/ea5786c85661d9b176795b9386af640b3e73aff3) we use the latest prebuild NDK (r21) toolchains for building rust libraries for android, so that we do not need to build the standalone NDK toolchains manually. If you have built or develop Stylo before 4.3.1, please download the NDK r19 or newer [here](https://developer.android.com/ndk/downloads) and point the `NKD_HOME` environment variable to it with e.g. `export NDK_HOME=/path/to/latest/ndk`

## Cannot run after upgrade to latest codebase

1. `yarn clean`
2. `yarn install`
3. `cd ios && pod install && cd ..`
4. delete app on device
5. `yarn start --reset-cache`

#### build on iOS
6. in Xcode (be sure to open with `./ios/stylo-app.xcodeworkspace` file), clean build with `shift + command + K`
7. `yarn run ios`

#### build on Android
6. clean build with `cd android && ./gradlew clean && cd ..`
7. `yarn run android`

## E2E Tests fail or timeout unexpectedly
1. erase simulator data on iOS Simulator with `Device` -> `Erase all content and settings`
2. `yarn clean && yarn`
3. `yarn start --reset-cache`
4. build & run as above
