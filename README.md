<!-- ![Stylo](logo.jpg) -->

<!-- [<img src="./res/github-badge.png" width="250"/>]()
[<img src="./res/google-play-badge.png" width="250"/>]()
[<img src="./res/app-store-badge.png" width="250"/>]() -->

# Stylo - An offline wallet for your offline phone

Stylo is a mobile application that allows any smartphone to act as an air-gapped crypto wallet. This is also known as "cold storage".

You can create Substrate and Ethereum accounts, sign messages/transactions, and transfer funds to and from these accounts without any sort of connectivity enabled on the device.

You must turn off or even physically remove the smartphone's Wifi, Mobile Network, and Bluetooth to ensure that the mobile phone containing these accounts will not be exposed to any online threat.

**Disabling the mobile phone's networking abilities is a requirement for the app to be used as intended, check our [wiki](./docs/wiki/Security-And-Privacy.md) for more details.**

Have a look at the tutorial on our wiki to learn how to use [Stylo together with Polkadot-js app](./docs/tutorials/Kusama-tutorial.md),  or [MyCrypto app](./docs/tutorials/MyCrypto-tutorial.md).

Any data transfer from or to the app happens using QR code. By doing so, the most sensitive piece of information, the private keys, will never leave the phone. The Stylo mobile app can be used to store any Polkadot or Ethereum account, this includes KSM, ETH, ETC as well as Ether from various testnets (Kovan, Ropsten...).

## Getting Start

### Tutorials

- [Signing with Pokadot.js apps or extension](./docs/tutorials/Kusama-tutorial.md)
- [Recover Account from Polkadot.js Apps](./docs/tutorials/Recover-Account-Polkadotjs.md)
- [Manage Accounts on Stylo](./docs/tutorials/Hierarchical-Deterministic-Key-Derivation.md)
- [Signing with MyCrypto](./docs/tutorials/MyCrypto-tutorial.md)
- [Update New Network](./docs/tutorials/New-Network.md)

### Wiki

- [Security and Privacy](./docs/wiki/Security-And-Privacy.md)
- [Development](./docs/wiki/Development.md)
- [Building and Publishing](./docs/wiki/Building-And-Publishing.md)
- [Testing](./docs/wiki/Test.md)
- [Troubleshooting](./docs/wiki/Troubleshooting.md)
- [QA Check List](./docs/wiki/QA.md)

## License

Stylo is [GPL 3.0 licensed](LICENSE).
