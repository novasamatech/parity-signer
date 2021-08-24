![Parity Signer](https://wiki.parity.io/images/logo-parity-signer.jpg)

[<img src="./res/github-badge.png" width="250"/>](https://github.com/paritytech/parity-signer/releases/)
[<img src="./res/google-play-badge.png" width="250"/>](https://play.google.com/store/apps/details?id=io.parity.signer)
[<img src="./res/app-store-badge.png" width="250"/>](https://itunes.apple.com/us/app/parity-signer/id1218174838)

# Parity Signer - Turn your smartphone into a hardware wallet

![Parity Signer](./docs/tutorials/images/logo-parity-signer.jpg)

Parity Signer is a mobile application that allows any smartphone to act as an air-gapped crypto wallet. This is also known as "cold storage".

You can create accounts in Substrate-based networks, sign messages/transactions, and transfer funds to and from these accounts without any sort of connectivity enabled on the device.

You must turn off or even physically remove the smartphone's Wifi, Mobile Network, and Bluetooth to ensure that the mobile phone containing these accounts will not be exposed to any online threat. Switching to airplane mode suffices in many cases.

**Disabling the mobile phone's networking abilities is a requirement for the app to be used as intended, check our [wiki](./docs/wiki/Security-And-Privacy.md) for more details.**

Have a look at the tutorial on our wiki to learn how to use [Parity Signer together with Polkadot-js app](./docs/tutorials/Kusama-tutorial.md).

Any data transfer from or to the app happens using QR code. By doing so, the most sensitive piece of information, the private keys, will never leave the phone. The Parity Signer mobile app can be used to store any Substrate account, this includes Polkadot (DOT) and Kusama (KSM) networks.

## Getting Started

**These tutorials and docs are heavily outdated at the moment, please use them as references or help improving**

### Tutorials

- [Signing with Pokadot.js Apps](./docs/tutorials/Kusama-tutorial.md)
- [Recover Account from Polkadot.js Apps](./docs/tutorials/Recover-Account-Polkadotjs.md)
- [Manage Accounts on Parity Signer](./docs/tutorials/Hierarchical-Deterministic-Key-Derivation.md)
- [Update New Network](./docs/tutorials/New-Network.md)

### Wiki

- [Security and Privacy](./docs/wiki/Security-And-Privacy.md)
- [Development](./docs/wiki/Development.md)
- [Building and Publishing](./docs/wiki/Building-And-Publishing.md)
- [Testing](./docs/wiki/Test.md)
- [Troubleshooting](./docs/wiki/Troubleshooting.md)
- [QA Check List](./docs/wiki/QA.md)
- [HDKD Feature](./docs/wiki/HDKD.md)
- [Changelog](./docs/wiki/Changelog.md)

### Legacy versions

Older versions of this app could be useful for development, however, they are not safe for use in production. They are available at following branches:

- [Last public release with React Native](https://github.com/paritytech/parity-signer/tree/legacy-4.5.3)
- [Non-ascii characters fix and some transaction parsing](https://github.com/paritytech/parity-signer/tree/legacy-4.6.2)
- [Metadata types import and message parsing in RN](https://github.com/paritytech/parity-signer/tree/legacy-metadataRN)
- [Rust backend with RN frontend](https://github.com/paritytech/parity-signer/tree/legacy-rust)

## License

Parity-Signer is [GPL 3.0 licensed](LICENSE).
