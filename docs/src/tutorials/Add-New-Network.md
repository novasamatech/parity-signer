# Add New Network

Parity Signer supports adding any substrate-based networks or updating and existing network via QR code.

After you've installed [required software](#Prerequisites), you need to add *Network's Specs* to Signer and add *Network Metadata* for this network, so that Signer is able to decode, and you could read and verify transactions you are signing on this network.

If you need to update metadata for already existing network you only need to update *Network Metadata*.
Off-the-shelf Signer comes with networks that you can update by scanning a multipart QR codes that containt recent metadata for these networks at [Metadata Update Portal](https://metadata.parity.io/).

*Network Specs*

1. [Get](#get-network-spec)
2. [Sign](#sign-network-spec)
3. [Feed into Signer](#feed-network-spec-into-signer)

*Network Metadata*

4. [Get](#get-network-metadata)
5. [Sign](#sign-network-metadata)
6. [Feed into Signer](#feed-network-metadata-into-signer)

### Prerequisites

- [ ] Websocket URL of the network (we maintain an [address book](https://github.com/paritytech/parity-signer/blob/master/rust/generate_message/address_book))
- [ ] [rust](https://www.rust-lang.org/tools/install)
- [ ] [subkey](https://docs.substrate.io/v3/tools/subkey/#installation)
- [ ] a clone of [parity-signer repository](https://github.com/paritytech/parity-signer). In the repository go to `generate_message` crate:

```
cd rust/generate_message
cargo build
cargo run restore_defaults
```

With `subkey` you need to generate a private and public keys (`<secret-phrase>` and `Public key (hex)`). You will be able to update a network only with metadata that is signed with the same keys as network specs that defined the network for your Signer instance.

```
subkey generate
```

---

## Network Specs

### Get Network Specs

In `parity-signer/rust/generate_message`

```
cargo run add_spec -u <network-ws-url> -<crypto>

```
```
// i.g.
cargo run add_specs -u wss://statemint-rpc.polkadot.io -sr25519

```

For networks supporting several tokens:

```
cargo run add_spec -u <network-ws-url> -<crypto> -token <decimals> <SYMBOL>

```
```
// i.g.
cargo run add_spec -u wss://karura-rpc-0.aca-api.network -sr25519 -token 12 KAR

```

Now your `<specs-file>` is in `parity-signer/rust/files/for_signing`


### Sign Network Spec

#### Get signature

In `parity-signer/rust/files/for_signing`

```
cat <spec-file> | subkey sign --suri <seed-phrase>
```

```
// i.g.
cat sign_me_add_specs_statemint_sr25519 | subkey sign --suri "fluid upon nuclear level middle funny suffer author group shrimp tornado relax"
```

This will return a `<signature>` you need to make a signed QR.

#### Make signed QR

In `parity-signer/rust/generate_message`

```
cargo run make -qr -crypto <crypto> -msgtype add_specs -payload <spec-file> -verifier -hex <signature>
```

```
// i.g.
cargo run make -qr -crypto sr25519 -msgtype add_specs -payload sign_me_add_specs_statemint_sr25519 -verifier -hex 0x927c307614dba6ec42f84411cc1e93c6579893859ce5a7ac3d8c2fb1649d1542 -signature -hex fa3ed5e1156d3d51349cd9bb4257387d8e32d49861c0952eaff1c2d982332e13afa8856bb6dfc684263aa3570499e067d4d78ea2dfa7a9b85e8ea273d3a81a86
```

Now your `<spec-qr>` is in `parity-signer/rust/files/signed`

### Feed Network Specs into Signer

In Signer open scanner, scan your your `<spec-qr>` and approve chain specs.


## Network Metadata

### Get Network Metadata

In `parity-signer/rust/generate_message`

```
cargo run load_metadata -d -u `<network-ws-url>`
```

```
// i.g.
cargo run load_metadata -d -u wss://statemint-rpc.polkadot.io
```

This will fetch fresh `<metadata-file>`, update the database with it, and - most relevant to us currently - generate file with message body in `parity-signer/rust/files/for_signing`. 

### Sign Network Metadata

#### Get Signature

In `parity-signer/rust/files/for_signing`

```
cat <metadata-file> | subkey sign --suri <seed-phrase>
```

```
// i.g.
cat sign_me_load_metadata_statemintV800 | subkey sign --suri "fluid upon nuclear level middle funny suffer author group shrimp tornado relax"
```

#### Make signed QR

In `parity-signer/rust/generate_message`

```
cargo run make -qr -crypto <crypto> -msgtype load_metadata -payload <metadata-file> -verifier -hex <signature>
```

```
// i.g.
cargo run make -qr -crypto sr25519 -msgtype load_metadata -payload sign_me_load_metadata_statemintV800 -verifier -hex 0x927c307614dba6ec42f84411cc1e93c6579893859ce5a7ac3d8c2fb1649d1542 -signature -hex 6a8f8dab854bec99bd8534102a964a4e71f4370683e7ff116c84d7e8d5cb344efd3b90d27059b7c8058f5c4a5230b792009c351a16c007237921bcae2ede2d84
```

This QR might take some time to be generated. After it is finished you can find your `<metadata-qr>` in `parity-signer/rust/files/signed`. It is a multipart QR-"movie", if you image viewer does not render it correctly, we suggest to open it in a browser.

### Feed Network Metadata into Signer

In Signer open scanner, scan your your `<metadata-qr>` and accept new metadata.

---

Congratulations! You've fetched network specs, signed them, fed them into Signer, fetched recent metadata for the network, signed and fed it into Signer as well. Now you are ready to safely sign transactions on this network.
