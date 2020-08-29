# Building and Publishing

In this page we will list the steps to publish the app on iOS App Store / Android Play Store.

## iOS App Store

##### Build production version

1. Switch to the `master` branch and run `yarn build`
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
