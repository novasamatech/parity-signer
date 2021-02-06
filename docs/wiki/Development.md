# Development


### Requirements

- `node.js` ( `>=10`)
- `yarn` (tested on `1.6.0`)
- rustup [https://rustup.rs/](https://rustup.rs/)
- watchman [install instructions](https://facebook.github.io/watchman/docs/install.html) also you may need to increase the number of inogtify watches with 
```bash
echo 999999 | sudo tee -a /proc/sys/fs/inotify/max_user_watches &&\
echo 999999 | sudo tee -a /proc/sys/fs/inotify/max_queued_events &&\
echo 999999 | sudo tee -a /proc/sys/fs/inotify/max_user_instances &&\
watchman shutdown-server
```

#### iOS
- `cocoapods` (`$ sudo gem install cocoapods`)
- `Xcode` (only for iOS, tested on `Version 11.3.1 (9F2000)`)

#### Android
- `Android Studio` (only for Android, tested on `Version 3.3`)
- `$JAVA_HOME` envarionment variable set to java home directory (eg. `/Library/Java/JavaVirtualMachines/jdk1.8.0_60.jdk/Contents/Home`)
- `$ANDROID_HOME` environment variable set to Android SDK directory (eg. `/home/your_username/Android/Sdk`)*.

\* It's recommended to install **Android Studio** and use that to install the necessary build tools and SDKs for the Android version you want to test on. It's also the best way to test in the emulator.

example of `~/.bashrc` or `~/.zhrc`:
```
# React native Android development
 export ANDROID_HOME=$HOME/Android/Sdk
 export JAVA_HOME=$(readlink -f /usr/bin/javac | sed "s:/bin/javac::")
 export NDK_HOME=$ANDROID_HOME/ndk-bundle

 export PATH=$PATH:$ANDROID_HOME/emulator
 export PATH=$PATH:$ANDROID_HOME/tools
 export PATH=$PATH:$ANDROID_HOME/tools/bin
 export PATH=$PATH:$ANDROID_HOME/platform-tools
 export PATH=$PATH:$JAVA_HOME/bin
```
- run `./scripts/init.sh` to install rust dependancies
- it might be needed to run `mkdir ~/Android/Sdk/ndk-bundle && mv ~/Android/Sdk/ndk/<ndk-version>`

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
