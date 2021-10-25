![Parity Signer](https://wiki.parity.io/logo-parity-signer.jpg)

[<img src="./docs/src/res/github-badge.png" width="250"/>](https://github.com/paritytech/parity-signer/releases/)
[<img src="./docs/src/res/google-play-badge.png" width="250"/>](https://play.google.com/store/apps/details?id=io.parity.signer)
[<img src="./docs/src/res/app-store-badge.png" width="250"/>](https://itunes.apple.com/us/app/parity-signer/id1218174838)

# Parity Signer - Turn your smartphone into a hardware wallet

![Parity Signer](./docs/src/tutorials/images/logo-parity-signer.jpg)

==========================================================

# !!! Parity Signer is being refactored from the groud up !!!

The legacy app is still fully functional and the latest release is available on the apps store,
and the source for this is available in [legacy-4.6.2 branch]https://github.com/paritytech/parity-signer/tree/legacy-4.6.2)

[The legacy documentation for the published app is here](https://github.com/paritytech/parity-signer/tree/legacy-4.6.2/docs)


==========================================================

==========================================================

==========================================================

==========================================================

==========================================================

==========================================================

**Please read instructions before building or using the Signer**

Parity Signer is a mobile application that allows any smartphone to act as an air-gapped crypto wallet. This is also known as "cold storage".

You can create accounts in Substrate-based networks, sign messages/transactions, and transfer funds to and from these accounts without any sort of connectivity enabled on the device.

You must turn off or even physically remove the smartphone's Wifi, Mobile Network, and Bluetooth to ensure that the mobile phone containing these accounts will not be exposed to any online threat. Switching to airplane mode suffices in many cases.

### Legacy versions

Older versions of this app could be useful for development, however, they are not safe for use in production. They are available at following branches:

- [Last public release with React Native](https://github.com/paritytech/parity-signer/tree/legacy-4.5.3)
- [Non-ascii characters fix and some transaction parsing](https://github.com/paritytech/parity-signer/tree/legacy-4.6.2)
- [Metadata types import and message parsing in RN](https://github.com/paritytech/parity-signer/tree/legacy-metadataRN)
- [Rust backend with RN frontend](https://github.com/paritytech/parity-signer/tree/legacy-rust)

## License

Parity-Signer is [GPL 3.0 licensed](LICENSE).
