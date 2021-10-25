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

==========================================================

# !!! Parity Signer is being refactored from the ground up !!!

The legacy app is still **safe to use and fully functional** and this release is published 
on the app stores. The source and documentation in the `./docs` directory for these are 
available in release branches (see below). 

[The legacy documentation for the published app is here](https://github.com/paritytech/parity-signer/tree/legacy-4.6.2/docs)

**NOTE: The `./docs` directory in this branch is under ***heavy development and is not updated for**
**the refactor at this time.**

==========================================================

### Legacy versions

Published versions for use are available at following branches:

- [Last public release with React Native](https://github.com/paritytech/parity-signer/tree/legacy-4.5.3)
- [Non-ascii characters fix and some transaction parsing](https://github.com/paritytech/parity-signer/tree/legacy-4.6.2)

> Older versions of this app could be useful for reference, but **they are not safe for use**
> to store keys in production.

- [Metadata types import and message parsing in RN](https://github.com/paritytech/parity-signer/tree/legacy-metadataRN)
- [Rust backend with RN frontend](https://github.com/paritytech/parity-signer/tree/legacy-rust)

## License

Parity-Signer is [GPL 3.0 licensed](LICENSE).
