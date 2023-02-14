![Polkadot Vault](./tutorials/images/logo-parity-signer.svg)

<div align="center">
    <a href="https://github.com/paritytech/parity-signer/releases"><img src="./res/github-badge.png" width="150"></a> <a href="https://play.google.com/store/apps/details?id=io.parity.signer"><img src="./res/google-play-badge.png" width="150"></a> <a href="https://itunes.apple.com/us/app/parity-signer/id1218174838"><img src="./res/app-store-badge.png" width="150"></a><br><br>
</div>

# Polkadot Vault - Turn your smartphone into a hardware wallet

Polkadot Vault is a mobile application that allows any smartphone to act as an air-gapped crypto wallet. This is also known as "cold storage".

You can create accounts in Substrate-based networks, sign messages/transactions, and transfer funds to and from these accounts without any sort of connectivity enabled on the device.

You must turn off or even physically remove the smartphone's Wifi, Mobile Network, and Bluetooth to ensure that the mobile phone containing these accounts will not be exposed to any online threat. Switching to airplane mode suffices in many cases.

**Disabling the mobile phone's networking abilities is a requirement for the app to be used as intended, check our [wiki](./about/Security-And-Privacy.md) for more details.**

Have a look at the tutorial on our wiki to learn how to use [Polkadot Vault together with Polkadot-js app](./tutorials/Kusama-tutorial.md).

Any data transfer from or to the app happens using QR code. By doing so, the most sensitive piece of information, the private keys, will never leave the phone. The Polkadot Vault mobile app can be used to store any Substrate account, this includes Polkadot (DOT) and Kusama (KSM) networks.

## Key features

- This is not a complete crypto wallet in itself. The Vault does not sync with blockchain, so it does not know your account balance, whether transactions were successful or even if the account exists! This is a **cold wallet** app that only stores keys, reads and signs messages. It should always be used with **hot wallet** like [polkadot.js](https://polkadot.js.org/apps).
- The Vault alone does not make your accounts secure. **You must maintain security yourself**. Airgap should be only part of your security protocol, improper use of Vault could still lead to loss of funds and/or secrets.
- When properly used, Vault provides best achievable security with Substrate networks to-date.

## System requirements

Currently Vault is available only for iOS. Android version is coming soon.

## Getting Started

**These tutorials and docs are heavily outdated at the moment, please use them as references or help improving**

If you are upgrading from older version of Vault, please see [changelog](./about/Changelog.md) and [upgrading Vault](./tutorials/Upgrading.md)

Please note that the Vault app is an advanced tool designed for maximum security and complex features. In many use cases, more user-friendly tools would be sufficient.

[Getting started guide](./tutorials/Start.md)

### User Guides

- [Start](./tutorials/Start.md)
- [Upgrading](./tutorials/Upgrading.md)
- [Add New Network](./tutorials/Add-New-Network.md)
- [Kusama-tutorial](./tutorials/Kusama-tutorial.md)
- [Recover-Account-Polkadotjs](./tutorials/Recover-Account-Polkadotjs.md)

### About

- [About Polkadot Vault](./README.md)
- [FAQ](./about/FAQ.md)
- [Security-And-Privacy](./about/Security-And-Privacy.md)
- [Hierarchical-Deterministic-Key-Derivation](./tutorials/Hierarchical-Deterministic-Key-Derivation.md)

### Legacy versions

Older versions of this app could be useful for development, however, they are not safe for use in production. They are available at following branches:

- [Last public release with React Native](https://github.com/paritytech/parity-signer/tree/legacy-4.5.3)
- [Non-ascii characters fix and some transaction parsing](https://github.com/paritytech/parity-signer/tree/legacy-4.6.2)
- [Metadata types import and message parsing in RN](https://github.com/paritytech/parity-signer/tree/legacy-metadataRN)
- [Rust backend with RN frontend](https://github.com/paritytech/parity-signer/tree/legacy-rust)

## License

Polkadot-Vault is [GPL 3.0 licensed](LICENSE).
