# Parity Signer Accounts Management

Parity Signer v4 has introduced the Hierarchical Deterministic Key Derivation (HDKD) feature for Substrate networks. This article explains how to use this feature.

* Notice: The UI maybe variant for different versions, but the functionalities are the same in v4 version.

## Introduction

Seed is the starting point for generating accounts. The seed itself does not have any network affiliation. For Substrate networks, generating a new accounts means entering a derivation path and choosing a network. With this feature, you can manage as many accounts as needed with just one seed phrase safely stored.

## Key Generation

### Create an account for a Substrate based network.

Key generation also refers to accounts creation, with your created Identity:
- Go to key manager and create a new seed or select an existing one
- Choose a network
- Tap on any key
- Tap `Derive` or `N+1` Button
- In path derivation screen, input any path and name you like (or accept naming suggestion)
- (optional) type password
- Tap `Derive` Button
- Done, you can start using new address.

### The form of path

Paths also refer to the Chaincodes which described in [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki/), though it is different from BIP-32 style:
* **Soft** derivation starts with a single slash, like: `/soft`
* **Hard** derivation starts with a double slash, like: `//hard`

Users are able to create any combination of hard derivation with `//` and/or soft derivation with `/`.

The encoded string are limited to 32 Bytes.

For technical information about the soft and hard derivations on Substrate, please refer to introduction [here](https://github.com/paritytech/parity-signer/wiki/HDKD-on-Parity-Signer#hdkd-general-on-substrate).

Path also could contain optional **password**; in Subkey standard password is prefixed with `///`. However, for convenience, Signer device has separate password entry field with password confirmation, thus do not add `///` to derivation field, it will result in error - instead omit `///` and type password into its' special field. It will not be stored on the device and will be required for any operation that requires private key of the account. There is no way to restore this password if it is lost so please back it up carefully.

### Further notes

* With the same BIP32 seed users could create keys under different networks.
* Each derived account is bound to certain networks, which prevents it to be misused in another network until it is explicitly added for that network as well. Root account is available for all networks by default.

## References:
1. [https://github.com/w3f/schnorrkel](https://github.com/w3f/schnorrkel)
2. [https://wiki.polkadot.network/docs/en/learn-keys](https://wiki.polkadot.network/docs/en/learn-keys)
