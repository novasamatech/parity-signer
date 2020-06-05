![Parity Signer](https://wiki.parity.io/images/logo-parity-signer.jpg)

[<img src="./res/github-badge.png" width="250"/>](https://github.com/paritytech/parity-signer/releases/)
[<img src="./res/google-play-badge.png" width="250"/>](https://play.google.com/store/apps/details?id=io.parity.signer)
[<img src="./res/app-store-badge.png" width="250"/>](https://itunes.apple.com/us/app/parity-signer/id1218174838)

# Parity Signer - Turn your smartphone into a hardware wallet

Parity Signer is a mobile application that allows any smartphone to act as an air-gapped crypto wallet. This is also known as "cold storage".

You can create Kusama and Ethereum accounts, sign messages/transactions, and transfer funds to and from these accounts without any sort of connectivity enabled on the device.

You must turn off or even physically remove the smartphone's Wifi, Mobile Network, and Bluetooth to ensure that the mobile phone containing these accounts will not be exposed to any online threat.

**Disabling the mobile phone's networking abilities is a requirement for the app to be used as intended.**

Have a look at the tutorial on our wiki to learn how to use [Parity Signer together with Polkadot-js app](https://wiki.parity.io/Parity-Signer-Mobile-App-Apps-Kusama-tutorial) for Kusama,  or [MyCrypto app](https://wiki.parity.io/Parity-Signer-Mobile-App-MyCrypto-tutorial) and [Parity Fether](https://wiki.parity.io/Parity-Signer-Mobile-App-Fether-tutorial) for Ethereum.

Any data transfer from or to the app happens using QR code. By doing so, the most sensitive piece of information, the private keys, will never leave the phone. The Parity Signer mobile app can be used to store any Kusama or Ethereum account, this includes KSM, ETH, ETC as well as Ether from various testnets (Kovan, Ropsten...).

## Getting Start

- [Building](https://github.com/paritytech/parity-signer/wiki/Building)
- [Testing](https://github.com/paritytech/parity-signer/wiki/Testing)
- [Troubleshooting](https://github.com/paritytech/parity-signer/wiki/Troubleshooting)
- [Publishing](https://github.com/paritytech/parity-signer/wiki/Publishing)

## Changes from 4.3.1
From [4.3.1](https://github.com/paritytech/parity-signer/commit/ea5786c85661d9b176795b9386af640b3e73aff3) we use the latest prebuild NDK toolchains for building rust libraries for android, so that we do not need to build the standalone NDK toolchains manually. If you have built or develop Parity Signer before 4.3.1, please download the latest NDK [here](https://developer.android.com/ndk/downloads)(tested with r21c) and point the `NKD_HOME` environment variable to it with e.g. `export NDK_HOME=/path/to/latest/ndk`

## License

Parity-Signer is [GPL 3.0 licensed](LICENSE).
