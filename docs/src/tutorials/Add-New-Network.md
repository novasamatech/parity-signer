# Add New Network

Polkadot Vault supports adding any substrate-based networks or updating an existing network via QR code.

After you've installed [required software](#Prerequisites), you need to add *Network's Specs* to Vault and add *Network Metadata* for this network, so that Vault is able to decode, and you could read and verify transactions you are signing on this network.

If you need to update metadata for already existing network you only need to update *Network Metadata*.
Off-the-shelf Vault comes with networks that you can update by scanning a multipart QR codes that contain recent metadata for these networks at [Metadata Update Portal](https://metadata.parity.io/).

*Network Specs*

1. [Get](#get-network-spec)
2. [Sign](#sign-network-spec)
3. [Feed into Vault](#feed-network-spec-into-signer)

*Network Metadata*

4. [Get](#get-network-metadata)
5. [Sign](#sign-network-metadata)
6. [Feed into Vault](#feed-network-metadata-into-signer)

### Prerequisites

- [ ] Network details
    - [ ] RPC endpoint (websocket URL)\
    *Hint: You can RPC endpoints for some of the public networks e.g. in [polkadot-js/apps repository](https://github.com/polkadot-js/apps/tree/master/packages/apps-config/src/endpoints)*
    - [ ] Encryption algorithm
- [ ] [rust](https://www.rust-lang.org/tools/install)
- [ ] a clone of [parity-signer repository](https://github.com/paritytech/parity-signer)
- [ ] [subkey](https://docs.substrate.io/v3/tools/subkey/#installation)
- [ ] Dedicated keypair specifically for signing updates\
*Please make sure you have a backup `<secret-phrase>` and `Public key (hex)` of this keypair. You will be able to update a network only with metadata that is signed with the same keypair as network specs. You can generate it with any tool of your choice, e.g with [subkey](https://docs.substrate.io/v3/tools/subkey/#installation):* `subkey generate`.

<!-- "TODO suggested strategy to handle this keypair: password 
derivation, dedicated signer -->

<br/>

---

<br/>

Let's get started!

## Add Network Specs

### Get Network Specs

In `parity-signer/rust/generate_message`

```
cargo run add-specs -u <network-ws-url> --encryption <crypto>

```
```
// e.g.
cargo run add-specs -u wss://statemint-rpc.polkadot.io --encryption sr25519

```

For networks supporting several tokens:

```
cargo run add-specs -d -u <network-ws-url> --encryption <crypto> --token-decimals <decimals> --token-unit <SYMBOL>

```
```
// e.g.
cargo run add-specs -d -u wss://karura-rpc-0.aca-api.network --encryption sr25519 --token-decimals 12 --token-unit KAR

```

<br/>

Now your `<specs-file>` is in `parity-signer/rust/files/for_signing`.

*Hint: you can read more about [interface with hot database](https://github.com/paritytech/parity-signer/blob/master/rust/generate_message/src/lib.rs) if you want to maintain it.*

<!-- TODO more about hot database, suggested use of hot database" -->

### Sign Network Spec

#### Get signature

In `parity-signer/rust/files/for_signing`

```
cat <spec-file> | subkey sign --suri <seed-phrase-and-derivation>
```

```
// e.g.
cat sign_me_add_specs_statemint_sr25519 | subkey sign --suri "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"
```

This will return a `<signature>` you need to make a signed QR.

#### Make signed QR

In `parity-signer/rust/generate_message`

```
cargo run --release make --goal qr --crypto <crypto> --msg add-specs --payload <spec-file> --verifier-hex <public-key> --signature-hex <signature>
```

```
// e.g.
cargo run --release make --goal qr --crypto sr25519 --msg add-specs --payload sign_me_add_specs_statemint_sr25519 --verifier-hex 0x927c307614dba6ec42f84411cc1e93c6579893859ce5a7ac3d8c2fb1649d1542 --signature-hex fa3ed5e1156d3d51349cd9bb4257387d8e32d49861c0952eaff1c2d982332e13afa8856bb6dfc684263aa3570499e067d4d78ea2dfa7a9b85e8ea273d3a81a86
```

<br/>

Now your `<spec-qr>` is in `parity-signer/rust/files/signed`

### Feed Network Specs into Vault

In Vault open scanner, scan your `<spec-qr>` and approve chain specs.

<br/>

## Add Network Metadata

### Get Network Metadata

In `parity-signer/rust/generate_message`

```
cargo run load-metadata -d -u `<network-ws-url>`
```

```
// e.g.
cargo run load-metadata -d -u wss://statemint-rpc.polkadot.io
```

<br/>

This will fetch fresh `<metadata_file>`, update the database with it, and - most relevant to us currently - generate file with message body in `parity-signer/rust/files/for_signing`.

### Sign Network Metadata

#### Get Signature

In `parity-signer/rust/files/for_signing`

```
cat <metadata-file> | subkey sign --suri <seed-phrase-and-derivation>
```

```
// e.g.
cat sign_me_load_metadata_statemintV800 | subkey sign --suri "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"
```

#### Make signed QR

In `parity-signer/rust/generate_message`

```
cargo run --release make --goal qr --crypto <crypto> --msg load-metadata --payload <metadata-file> --verifier-hex <public-key> --signature-hex <signature>
```

```
// e.g.
cargo run --release make --goal qr --crypto sr25519 --msg load-metadata --payload sign_me_load-metadata_statemintV800 --verifier-hex 0x927c307614dba6ec42f84411cc1e93c6579893859ce5a7ac3d8c2fb1649d1542 --signature-hex 6a8f8dab854bec99bd8534102a964a4e71f4370683e7ff116c84d7e8d5cb344efd3b90d27059b7c8058f5c4a5230b792009c351a16c007237921bcae2ede2d84
```

This QR might take some time to be generated. After it is finished you can find your `<metadata-qr>` in `parity-signer/rust/files/signed`. It is a multipart QR-"movie", if your image viewer does not render it correctly, we suggest to open it in a browser.

### Feed Network Metadata into Vault

In Vault open scanner, scan your `<metadata-qr>` and accept new metadata.

<br/>

---

<br/>

Congratulations! You've fetched network specs, signed them, fed them into Vault, fetched recent metadata for the network, signed and fed it into Vault as well. Now you are ready to safely sign transactions on this network.
