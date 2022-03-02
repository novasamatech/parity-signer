![Parity Signer](./tutorials/images/logo-parity-signer.jpg)

[<img src="./res/github-badge.png" width="250"/>](https://github.com/paritytech/parity-signer/releases/)
[<img src="./res/google-play-badge.png" width="250"/>](https://play.google.com/store/apps/details?id=io.parity.signer)
[<img src="./res/app-store-badge.png" width="250"/>](https://itunes.apple.com/us/app/parity-signer/id1218174838)

# Parity Signer - Turn your smartphone into a hardware wallet

![Parity Signer](./tutorials/images/logo-parity-signer.jpg)

Parity Signer is a mobile application that allows any smartphone to act as an air-gapped crypto wallet. This is also known as "cold storage".

You can create accounts in Substrate-based networks, sign messages/transactions, and transfer funds to and from these accounts without any sort of connectivity enabled on the device.

You must turn off or even physically remove the smartphone's Wifi, Mobile Network, and Bluetooth to ensure that the mobile phone containing these accounts will not be exposed to any online threat. Switching to airplane mode suffices in many cases.

**Disabling the mobile phone's networking abilities is a requirement for the app to be used as intended, check our [wiki](./wiki/Security-And-Privacy.md) for more details.**

Have a look at the tutorial on our wiki to learn how to use [Parity Signer together with Polkadot-js app](./tutorials/Kusama-tutorial.md).

Any data transfer from or to the app happens using QR code. By doing so, the most sensitive piece of information, the private keys, will never leave the phone. The Parity Signer mobile app can be used to store any Substrate account, this includes Polkadot (DOT) and Kusama (KSM) networks.

## Key features

- This is not a complete cryptowallet in itself. The Signer does not sync with blockchain, so it does not know your account balance, whether transactions were successful or even if the account exists! This is a **cold wallet** app only stores keys, reads and signs messages. It should always be used with **hot wallet** like [polkadot.js](https://polkadot.js.org/apps).
- The Signer alone does not make your accounts secure. **You must maintain security yourself**. Airgap should be only part of your security protocol, improper use of Signer could still lead to loss of funds and/or secrets.
- When properly used, Signer provides best achievable security with Substrate networks to-date.

## System requirements

Currently Signer is available only for iOS. Android version is coming soon.

## Getting Started

**These tutorials and docs are heavily outdated at the moment, please use them as references or help improving**

If you are upgrading from older version of Signer, please see [changelog](./wiki/Changelog.md) and [upgrading Signer](./wiki/Upgrading.md)

Please note that the Signer app is an advanced tool designed for maximum security and complex features. In many use cases, more user-friendly tools would be sufficient.

[Getting started guide](./tutorials/Start.md)

### Tutorials

- [Signing with Pokadot.js Apps](./tutorials/Kusama-tutorial.md)
- [Recover Account from Polkadot.js Apps](./tutorials/Recover-Account-Polkadotjs.md)
- [Manage Accounts on Parity Signer](./tutorials/Hierarchical-Deterministic-Key-Derivation.md)
- [Update New Network](./tutorials/New-Network.md)

### Wiki

- [Security and Privacy](./wiki/Security-And-Privacy.md)
- [Development](./wiki/Development.md)
- [Building and Publishing](./wiki/Building-And-Publishing.md)
- [Testing](./wiki/Test.md)
- [Troubleshooting](./wiki/Troubleshooting.md)
- [QA Check List](./wiki/QA.md)
- [Key derivations](https://substrate.dev/docs/en/knowledgebase/integrate/subkey)
- [Changelog](./wiki/Changelog.md)
- [QR encoding definition](https://github.com/maciejhirsz/uos)

### Legacy versions

Older versions of this app could be useful for development, however, they are not safe for use in production. They are available at following branches:

- [Last public release with React Native](https://github.com/paritytech/parity-signer/tree/legacy-4.5.3)
- [Non-ascii characters fix and some transaction parsing](https://github.com/paritytech/parity-signer/tree/legacy-4.6.2)
- [Metadata types import and message parsing in RN](https://github.com/paritytech/parity-signer/tree/legacy-metadataRN)
- [Rust backend with RN frontend](https://github.com/paritytech/parity-signer/tree/legacy-rust)

## License

Parity-Signer is [GPL 3.0 licensed](LICENSE).
