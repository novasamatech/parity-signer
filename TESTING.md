# Testing

### Test Signing

For a super quick test and to avoid the hurdle of creating an account, sending funds to it and finally create a transaction as described in the [tutorial using Parity Fether](https://wiki.parity.io/Parity-Signer-Mobile-App-Fether-tutorial) or the [tutorial using MyCrypto](https://wiki.parity.io/Parity-Signer-Mobile-App-MyCrypto-tutorial), you can use a pre-funded account on Kovan Network and the following workflow. To get access to this account, you need to:

- Recover an account
- Select `Kovan` network and choose a name
- Use the key phrase: `this is sparta` you'll get the account address: `006E27B6A72E1f34C626762F3C4761547Aff1421`
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

Layer Wallet is integrated with [Detox](https://github.com/wix/Detox) E2E testing. Detox has very detailed [documentation](https://github.com/wix/Detox/blob/master/docs/README.md).

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

In order to clear the detox cache run:
```
detox clean-framework-cache && detox build-framework-cache
```

In order to clear the ios simulator run:
```
xcrun simctl erase all
```

If you want to use another specific emulator/simulator than those defined in the configuration, add `--device-name` flag (on Android API version is needed), for example:
```
yarn test-e2e:ios --device-name iPhone X
yarn test-e2e:android --device-name Pixel_2_API_28
```

On Android please replace `ios` with `android`, currently Detox's Android 0.60.x support is in progress, if there is an error, try to build it again with `yarn build-e2e:android`

### Additional Tests
Basically any method in `src/util` should be unit tested via Jest.

### E2E Testing (Ethereum Legacy)

1. Recover an account on Kovan with the phrase "this is sparta"
2. Go to MyCrypto and generate a transaction
3. Scan transaction QR
4. Scan the signature back to MyCrypto
5. Make sure the transaction actually succeeds
6. Go to Sign a Message tab in MyCrypto now
7. Type in a message, any message
8. Repeat steps 3, 4
9. Expect the message is readable just as you typed it in
10. Scan the signature back to MyCrypto
11. Verify the message

### E2E Testing (UOS)
There is currently no testnet so we just check that the signature is formatted correctly.

1. Clone  https://github.com/polkadot-js/apps
2. Disable the balance validation in apps so you can construct an extrinsic with 0 balance
3. Choose Kusama network in Settings Tab
4. Do any extrinsic of your choice
5. Scan QR
5. Go to Settings, and change to Substrate (either the hosted Flaming Fir or a custom chain)
6. Run the same transaction
7. Expect the message: "Signer does not currently support a chain with the genesis hash: 0x....."
8. Go to `constants.js`
9. Add the appropriate genesis hash
10. Uncomment the checks for `isSubstrateDev` in `PayloadDetailsCard.js`
11. Repeat steps 4, 5
12. Expect the method decoding to be raw bytes.

# QA Checking List

## Identity Manipulation

* Identity could be generated with 12 words key phrase
* Identity could be generated with 24 words key phrase
* Identity could be successfully recovered
* Substrate account could be generated and derivated
* Ethereum account could be generated within Identity
* An Root account could be derived by the custom (Add Network Account -> Create Custom Path -> Use empty string as path)
* An Passworded account could be derived by the custom (Add Network Account -> Create Custom Path -> Add additional password -> input any path)

## Signing
* Legacy Kusama Account Extrinsic Signning with Single QR code
* Legacy Kusama Account Extrinsic Signning with Multiple QR code
* Legacy Ethereum Account Tx Signinng with MyCrypto
* Legacy Ethereum Account Message Signinng with MyCrypto
* Identity Kusama Account Extrinsic Signning with Single QR code
* Identity Polkadot Account Extrinsic Signning with Single QR code
* Identity Kusama Account Extrinsic Signning with Multiple QR code
* Identity Polkadot Account Extrinsic Signning with Multiple QR code
* After move the app into background and back to app, Signing will need pin input again.
* Identity Substrate Account with Password Signing should also works, which should always need password input
