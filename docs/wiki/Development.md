# Development


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

- Install Dependencies

    ```
    yarn build
    ```   

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
