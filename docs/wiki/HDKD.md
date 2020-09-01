# HDKD on Parity Signer

## HDKD data schema on Parity Signer v4

In the v4 version, the data schema is significantly refactored in order to support HDKD feature.

### Highlights of the new schema

* With the same BIP32 seed users could create accounts under different networks. But if a user has a brain wallet seed, it can only create Ethereum accounts. 
* Pin is now bound to the identity instead of the keypair.
* Additional derivation password could be set for identity in the future version, it will bind to identity.
* Users are able to create any combination of hard derivation with `//` or soft derivation with `/`, the path will be the index of the key pair/ account within a same seed.
* When receiving an upcoming signing request, Signer will first search in its address map `addresses` for the correct path, and then use the seed of the path for signing.
* Each derived keypair is coupled with a path ( which means it belongs to a certain network), which prevent it to be used in another network (e.g. derive a key pair under //kusama will only check the signing request with kusama's latest genesisHash, and won't sign a transaction on another Network)
* Network Schema could be updated by Scan QR code in the future version.

Furthermore, in the new Identity schema, we support hard spoon network in the Substrate network like Kusama, since the Substrate keypairs/accounts generated on Signer will decouple with network's genesisHash, but will use the path as identifier of their networks. From network genesisHash and path's meta data, the `accountId` is generated for signing.
```javascript
 const networkParams = getSubstrateNetworkParams(meta.path)
 const accountId = getAccountId(meta.address, networkParams.genesisHash)
```
Once the network is hard-spooned, users just need to update network params by scanning QR code, without requring new version of the Signer.

### Example Schema

```javascript
const networks = [
  {
    name: 'kusama',
    genesisHash: "0xb28bd355a4cd1df46bc56a2949bce8feb84eebc864f1c3f1f77668bd3b6559b3",
    prefix: 2,
    decimals: 12,
    pathID: 'kusama',
    title: 'Kusama CC2',
    unit: 'KSM',
    color: '#e6007a',
    protocol: 'substrate'
  },
  {
    color: '#64A2F4',
    ethereumChainId: '1',
    secondaryColor: colors.card_bgSolid,
    title: 'Ethereum Frontier',
    protocol: 'ethereum'
  }
];


const identities = [
  {
    encryptedSeed: '{"cipher":"aes-128-ctr","cipherparams":{"iv":e872394210984322432423,"cipherText":"a352dfg3g4245..."}}',
    derivationPassword: '',
    name: 'identity1',
    //NOTICE: meta is a Map Object
    meta: {
      '//kusma//funding/1': {
        name: 'funding account1',
        address: "DwGa4E65nbGafaGirjMH5kXbtT7jHuAcSXHLLkTePnqw95p",
        createdAt: 1571068850409,
        updatedAt: 1571078850509,
      },
      '//kusma//funding/2': {
        address: "EoXsgmP36WXkyCamoC9iHy7Gr2tZpEykz8rmFw3bAS1nArV",
        name: 'funding account2',
        createdAt: 1571068850409,
        updatedAt: 1571078850509,
        networkPathId: 'westend' //override the network
      },
      '1': {
        name: 'ethereum frontier account',
        address: "06Fc73F6797Ed62f46A4E6A13dD9366C5f2725D7",
        createdAt: 1571068850409,
        updatedAt: 1571078850509,
      }
    },
    //NOTICE: addresses is a Map Object
    addresses: {
      'DwGa4E65nbGafaGirjMH5kXbtT7jHuAcSXHLLkTePnqw95p': '//kusama//funding/1',
      'EoXsgmP36WXkyCamoC9iHy7Gr2tZpEykz8rmFw3bAS1nArV': '//kusama//funding/2',
      'ethereum:0x06Fc73F6797Ed62f46A4E6A13dD9366C5f2725D7@42': '1',
    }
  },
  {
    encryptedSeed: '{"cipher":"aes-128-ctr","cipherparams":{"iv":f872394210984322432423,"cipherText":"sad12343fdcdxa..."}}',
    derivationPassword: '',
    name: 'identityNew'
    //NOTICE: meta is a Map Object
    meta: {
      '//kusma//funding/1': {
        name: 'funding account1',
        address: "FJBQ3tv6w3qhYkhKWnxfdjPwhvCd879oUeQKt6u8bHyhqzX",
        createdAt: 1571068850409,
        updatedAt: 1571078850509,
      },
    },
    //NOTICE: addresses is a Map Object
    addresses: {
      "FJBQ3tv6w3qhYkhKWnxfdjPwhvCd879oUeQKt6u8bHyhqzX": '//kusama//funding/1',
    }
  }
];
```

## HDKD General on Substrate

### Keypair generation process Substrate

1. The recovery phrase goes into the `tiny-bip39` crate to produce entropy: https://docs.rs/tiny-bip39/0.6.2/bip39/struct.Mnemonic.html#method.entropy
2. Generate the "mini secret key" from entropy: https://docs.rs/substrate-bip39/0.3.1/substrate_bip39/fn.mini_secret_from_entropy.html
3. "mini secret key" can be used with `schnorrkel` to create the MiniSecretKey that can be expanded into a full keypair: https://docs.rs/schnorrkel/0.8.5/schnorrkel/keys/struct.MiniSecretKey.html
4. MiniSecretKey can be used for signing, or we can derive new secrets from it. https://github.com/paritytech/parity-signer/blob/0cb137a2a3717c178be6981f9d47129ef3067e5e/rust/signer/src/sr25519.rs#L48-L51

### The form of Path

Paths also refer to the Chaincodes which described in [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki/), it is totally different from BIP-32 style:
* Soft derivation starts with single slash, like: `/soft`
* Hard derivation starts with double slash, like: `//hard`

The encoded strings follows are limited to 32 Bytes.

### Difference of hard and soft derivated keys.

Both hard and soft derivation will create new keypair, and the following three ways work:

* Private parent key -> private child key
* Public parent key -> public child key (only works for soft keys)
* Private parent key -> public child key (for hard keys, it can only by the way that firstly derives child private key, and then derive child public key, for soften keys, there is an additional way is to first derive parent public key, and then derive the public key)

The key difference is that:
hard - Public key does not has a corresponding derivation, derived public key is not linked with parent public key, it can not be proved. 
soft - Public key has a corresponding derivation, derived public key is linked with parent public key, it can be proved. 

The soft derivation method has a pro here is that one could derive child public keys of a given parent key without knowing any private key.

### Use cases

Basically, the HDKD ease the management & storing of variant keys / recovery phrases. In addition to that it enables:

* distributing the keypairs in an organization.
* track different transactions by auditor / seller.
* sharing the whole wallet
* frequent transactions that cannot (easily) be associated
* distributing keys for multi-signature.

Especially for soft derivated keys:
One may distribute the public key instead of the private key, so that the owner's of the parent public key could derive child public key to monitor the transactions on the address.

### Potental Risks

Some related security reality is:

With child private key and path, could not find parent private key efficiently.
With fixed order of private key list, could not find parent private key efficiently.

And the followings are NOT true, which may lead to potential risks:

With parent and child public key, it is hard to find the path.
With parent public key and soften child private key, it is hard to find the parent private key.

NOTICE: If a derived soft child private key is compromised, and parent public key is exposed, then the risk for compromising parent private key will be raised. So the suggestion is if you use soft derivated child keypairs, then better not to sign and submit transactions with parent private key.

## References:
1. https://github.com/w3f/schnorrkel
2. https://wiki.polkadot.network/docs/en/learn-keys
3. https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki/

