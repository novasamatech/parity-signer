![Parity Signer](./docs/src/tutorials/images/logo-parity-signer.jpg)

<center>
<a href=https://github.com/paritytech/parity-signer/releases/> <img src="./docs/src/res/github-badge.png" width="100"/></a>
<a href=https://play.google.com/store/apps/details?id=io.parity.signer/> <img src="./docs/src/res/google-play-badge.png" width="100"/></a>
<a href=https://itunes.apple.com/us/app/parity-signer/id1218174838> <img src="./docs/src/res/app-store-badge.png" width="100"/></a>
</center>

# Parity Signer

Parity Signer is a mobile application that allows any smartphone to act as an air-gapped crypto
wallet. This is also known as "cold storage".

You can create accounts in Substrate-based networks, sign messages/transactions, and transfer
funds to and from these accounts without any sort of connectivity enabled on the device.

You must turn off or even physically remove the smartphone's Wifi, Mobile Network, and Bluetooth
to ensure that the mobile phone containing these accounts will not be exposed to any online threat.
Switching to airplane mode suffices in many cases.

# Documentation

Please read instructions before **using Signer for the first time** or before **upgrading**

https://paritytech.github.io/parity-signer/index.html

==========================================================

# !!! Parity Signer was refactored from the ground up !!!

Please consider updating to new version as soon as you can. Remember, that direct upgrade is impossible (and quite dangerous, as it requires airgap to be disabled) - so all app and its data should be removed before upgare. **Please follow the instructions above**.

==========================================================

### Legacy versions

- [Last public release with React Native](https://github.com/paritytech/parity-signer/tree/legacy-4.5.3)
- [Non-ascii characters fix and some transaction parsing](https://github.com/paritytech/parity-signer/tree/legacy-4.6.2)
- [Metadata types import and message parsing in RN](https://github.com/paritytech/parity-signer/tree/legacy-metadataRN)
- [Rust backend with RN frontend](https://github.com/paritytech/parity-signer/tree/legacy-rust)

## License

Parity-Signer is [GPL 3.0 licensed](LICENSE).
