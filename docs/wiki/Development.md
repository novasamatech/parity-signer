# Development


### Requirements

- `node.js` ( `>=10`)
- `yarn` (tested on `1.6.0`)
- rustup [https://rustup.rs/](https://rustup.rs/)
- watchman [install instructions](https://facebook.github.io/watchman/docs/install.html) also you may need to increase the number of inotify watches with 
```bash
echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf;
sudo sysctl -p;
```

#### iOS
- `cocoapods` (`$ sudo gem install cocoapods`)
- `Xcode` (tested on `Version 11.3.1 (9F2000)`)

#### Android
- `Android Studio` (only for Android, tested on `Version 4.1`)
- In Android studio, install 
- Set environment variable for`$JAVA_HOME` set to java home directory, `$ANDROID_HOME` set to Android SDK directory and `$NDK_HOME` to point to the ndk directory and version installed by Android Studio.

example of `~/.bashrc` or `~/.zhrc`:
```
# React native Android development
 export ANDROID_HOME=$HOME/Android/Sdk
 export JAVA_HOME=$(readlink -f /usr/bin/javac | sed "s:/bin/javac::")
 export NDK_HOME=$ANDROID_HOME/ndk/22.0.7026061

 export PATH=$PATH:$ANDROID_HOME/emulator
 export PATH=$PATH:$ANDROID_HOME/tools
 export PATH=$PATH:$ANDROID_HOME/tools/bin
 export PATH=$PATH:$ANDROID_HOME/platform-tools
 export PATH=$PATH:$JAVA_HOME/bin
```
### Setup

- run `./scripts/init.sh` to install rust dependancies

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
	
	Magic command to clean, reinstall deps and launch the RN server with the cache cleaned:  
  `yarn clean:android && yarn && yarn start --reset-cache` 
