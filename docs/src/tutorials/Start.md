# Starting with Signer

This is suggested usage pattern; you should adjust it to your security protocol if you are certain you know what you are doing.

## Installation

### Factory reset the phone

The Signer should be installed in most secure environment possible. To achieve that, the phone should be reset to factory state.

Wipe the phone to factory state. This is good time to install newer version of operating system if you like. Make sure your system is genuine by all means provided by OS vendor.

### Set up phone

Before installing the Signer, you need to set up the phone. It is essential that you enable sufficient authentication method; your secret seeds in Signer are as safe as the phone is. Seed secrets are protected with hardware encryption based on vendor authentification protocol. Other than that, you might want to select dark mode (Signer remains dark for historic reasons).

### Install Signer

Download signed application through application store or from github. Make sure the signature is valid! Install the app. Do not start the app just yet!

### Disable network

Before starting the Signer, you should make sure that network is disabled. Many operating systems allow only partial network monitoring; although there are network detection features in Signer, they are limited and only have informational function. **User is responsible for maintaining airgapped state!** The simplest way to disable connectivity is setting the phone in airplane mode. Advanced users might want to use physical methods to further protect the phone from connections. Perform all preparations before starting the Signer app!

## First start

When you first launch Signer, it prompts you to read and accept terms and conditions and privacy policy. Once that is done, the database is pre-populated with built-in networks and Signer is ready for use. It could [import network data](./Add-New-Network.md) or [read transactions](./Kusama-tutorial.md), but to sign anything you need to create keys.

### Create keys

Open key manager by tapping bottom left symbol. On fresh start you will be prompted to create seed (otherwise you could always create more seeds by tapping `New seed` button in Key Manager). Enter any convenient seed name (it does not matter anything and is not used anywhere except for this particulat Signer device) and - if you would like to use custom seed phrase - switch to recovery mode and type the seed phrase. Custom seed phrase should be used only to recover or import existing key(s), do not input custom seed phrase unless it is properly random! **Security of your accounts relies on randomness of seed phrase**. If you are generating new seed phrase, use built-in random generator and do not input a custom seed phrase.

Once you click `create` button, you will be prompted to authenticate yourself. This will happen every time cruptographic engine of the phone is used to handle seeds - on all creations, backups, derivations and signatures and in some OS versions on starting the Signer.

You will see the created secret seed. Please back it up on paper and store it in safe place. If you lose your Signer device or it will become non-functional, you will be able to recover your keys using this seed phrase. Anyone could recover your keys with knowledge of this phrase. If you lose this seed phrase, though, **it will be impossible to recover your keys**. You can check the seed phrase anytime in Settings menu, but make sure that it is backed up at all times.

Once you dismiss seed phrase backup screen, the seed and some associated keys will be created. For every network known to the Signer, a network root derivation key will be generated, hard-derived from seed phrase with network name. A root key will be generated and made available in all networks. **Do not use the root key unless you know what you do!**.

To learn more on key generation, read [subkey specifications](https://substrate.dev/docs/en/knowledgebase/integrate/subkey) that Signer follows tightly and [Signer key management](./Hierarchical-Deterministic-Key-Derivation.md).

### Export public key

Once you have a keypair you would like to use, you should first export it to hot wallet. Tap the key and select `Export` button. You will see the export QR code you can use with hot wallet.

Details on [signing with Pokadot.js Apps](./Kusama-tutorial.md)
