# Build

First and foremost, make sure you have the latest [Rust](https://www.rust-lang.org/tools/install) installed in your system. Nothing will work without Rust.

If you get errors like `cargo: feature X is required`, it most likely means you have an old version of Rust. Update it by running `rustup update stable`.

## iOS

**1.** You probably already have [Xcode](https://developer.apple.com/xcode/) installed if you are reading this. If not, go get it. 

**2.** Compile the core Rust library first:

```
cd scripts && ./build.sh ios
```

**3.** Open the `NativeSigner.xcodeproj` project from the `ios` folder in your Xcode and click Run (Cmd+R).

**4.** The first time you start the app, you will need to put your device into Airplane Mode. In the iOS simulator, you can do this by turning off WiFi on your Mac (yes, this is an official apple-recommended way).

However, we strongly recommend that you use a real device for development, as some important parts (e.g. camera) may not work in the simulator.

## Android

> ⚠️ Android build has only been tested on Linux. If you manage by some miracle to run this on a Mac, please add the steps to this Readme

**1.** Download [Android Studio](https://developer.android.com/studio).

**2.** Open the project from the `android` directory.

**3.** Install NDK. Go to `File -> Project Structure -> SDK Location`. Next to the "Android NDK location" section, click "Download Android NDK" button.

We hightly recommend you to update all existing plugins and SDK's for Kotlin, Gradle, etc even if you just downloaded a fresh Android Studio. It's always a good idea to restart Android Studio after that. This can save you many hours on Stackoverflow trying to fix random errors like "NDK not found".

**4.** Connect your device or create a virtual one. Open `Tools -> Device Manager` and create a new phone simulator with the latest Android.

**5.** Run the project (Ctrl+R). It should build the Rust core library automatically.

