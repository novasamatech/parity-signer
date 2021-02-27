# Testing

### Test Signing

For a super quick test and to avoid the hurdle of creating an account, sending funds to it and finally create a transaction as described in the [tutorial using MyCrypto](./docs/tutorials/MyCrypto-tutorial.md), you can use a pre-funded account on Kovan Network and the following workflow. To get access to this account, you need to:

- Recover an account
- Select `Kovan` network and choose a name
- Use the recovery phrase: `this is sparta` you'll get the account address: `006E27B6A72E1f34C626762F3C4761547Aff1421`
- Validate and accept the warning message
- Chose a pin code
- Scan this QR code to sign a transaction sending some Kovan Eth to the same account.

![Sample QR Code](https://raw.githubusercontent.com/paritytech/parity-signer/master/docs/tx_qr.png)

Corresponding data:

```json
{
    "action": "signTransaction",
    "data": {
        "account": "006e27b6a72e1f34c626762f3c4761547aff1421",
        "rlp": "ea1584ee6b280082520894006e27b6a72e1f34c626762f3c4761547aff1421872386f26fc10000802a8080"
    }
}
```

### Unit Test

If dependencies are not installed please first run `yarn install:ios` or `yarn install` for android.

Run `yarn unit` for all the units test.

If debugging is needed:

1. Insert `debugger;` in the code where you think it fails.
2. Run `yarn unit:debug`
3. Open a new tab in Chrome and go to `chrome://inspect`
4. Click the `inspect` button of target under `Remote Target`
5. Back to the terminal, choose one of the node watch commands to run the tests again.

### Integration Test

Stylo is integrated with [Detox](https://github.com/wix/Detox) E2E testing. Detox has very detailed [documentation](https://github.com/wix/Detox/blob/master/docs/README.md).

First make sure `detox-cli` is installed as global dependency with

```
yarn global add detox-cli
```

#### Complete Test

If dependencies are not installed please first run `yarn install:ios` or `yarn install` for android.

1. run react native server with `yarn start`

2. run `yarn e2e:ios` or `yarn e2e:android`.

##### Develop and Test
Details please refer to Detox official guide [here](https://github.com/wix/Detox/blob/master/docs/Guide.DevelopingWhileWritingTests.md)

Once you have run `yarn ios` you do not need to build it, just run:
```shell
yarn test-e2e:ios
```
This command will open another simulator with the pre-defined configurations.

Re-run tests without re-installing the app
```
yarn test-e2e:ios --reuse
```

If you want to use another specific emulator/simulator than those defined in the configuration, add `--device-name` flag (on Android API version is needed), for example:
```
yarn test-e2e:ios --device-name iPhone X
yarn test-e2e:android --device-name Pixel_2_API_28
```

On Android please replace `ios` with `android`, currently Detox's Android 0.60.x support is in progress, if there is an error, try to build it again with `yarn build-e2e:android`
