# Scope  

# Terminology  

Signer receives information over the air-gap as QR codes. QR codes are read as
`u8` vectors, and must always be parsed by Signer before use.  

QR codes could contain information that user wants to sign with one of the
Signer keys or the update information to ensure smooth Signer operation without
reset or connecting to the network.  

## QR code types  

- Signable, to generate and export signature:  

   - Transaction: from call, gets later processed on chain if signature goes
   through online client  

   - Message
   
- Update, for Signer inner functionality:  

   - add network specs

   - load metadata
   
   - load types
   
- Derivations import, for bulk derivations import

- Testing

# QR code structure  

(padding, length indicator, shift)

Every QR code content starts with a prelude `[0x53, 0x<encryption code>,
0x<payload code>]`.

`0x53` is always expected and indicates Substrate-related content.

`<encryption code>` for signables indicates encryption algorithm that will be
 used to generate the signature:

- `0x00` stands for Ed25519
- `0x01` stands for Sr25519
- `0x02` stands for Ecdsa

`<encryption code>` for updates indicates encryption algorithm that was used to
sign the update:

- `0x00` stands for Ed25519
- `0x01` stands for Sr25519
- `0x02` stands for Ecdsa
- `0xff` means the update is not signed

Derivations import and testing are always unsigned, with `<encryption code>`
always `0xff`.

Signer supports following `<payload code>` variants:

- `0x00` legacy mortal transaction
- `0x02` transaction (both mortal and immortal)
- `0x03` message, **content under discussion**
- `0x80` load metadata update
- `0x7f` load compressed metadata update, **proposal only**
- `0x81` load types update
- `0xc1` add specs update
- `0xde` derivations import
- `0xf0` testing parser card display

Note: old UOS specified `0x00` as mortal transaction and `0x02` as immortal one,
but currently both mortal and immortal transactions from polkadot-js are `0x02`.

## Shared QR code processing sequence:

1. Read QR code, try interpreting it, and get the hexadecimal string from into
Rust (hexadecimal string is getting changes to raw bytes soon).
If QR code is not processable, nothing happens and the scanner keeps trying to
catch a processable one.
2. Analyze prelude: is it Substrate? is it a known payload type? If not, Signer
always produces an error and suggests to scan supported payload.

Further processing is done based on the payload type.

## Transaction

Transaction has following structure:

<table>
    <tr>
        <td>prelude</td><td>public key</td><td>SCALE-encoded call data</td><td>SCALE-encoded extensions</td><td>network genesis hash</td>
    </tr>
</table>

Public key is the key that can sign the transaction. Its length depends on the
`<encryption code>` declared in transaction prelude:

| Encryption | Public key length, bytes |
|:-|:-|
| Ed25519 | 32 |
| Sr25519 | 32 |
| Ecdsa | 33 |

Call data is `Vec<u8>` representation of transaction content. Call data must be
parsed by Signer prior to signature generation and becomes a part of signed
blob. Within transaction, the call data is SCALE-encoded, i.e. effectively is
prefixed with compact of its length in bytes.

Extensions contain data additional to the call data, and also are part of a
signed blob. Typical extensions are Era, Nonce, metadata version, etc.
Extensions content and order, in principle, can vary between the networks and
metadata versions.

Network genesis hash determines the network in which the transaction is created.
At the moment genesis hash is fixed-length 32 bytes.

Thus, the transaction structure could also be represented as:

<table>
    <tr>
        <td>prelude</td><td>public key</td><td>compact of call data length</td><td>**call data**</td><td>**SCALE-encoded extensions**</td><td>network genesis hash</td>
    </tr>
</table>

Bold-marked transaction pieces are used in the blob for which the signature is
produced. If the blob is short, 257 bytes or below, the signature is produced
for it as is. For blobs longer than 257 bytes, 32 byte hash (`blake2-rfc`) is
signed instead. This is inherited from earlier Signer versions, and is currently
compatible with polkadot-js.

### Transaction parsing sequence

1. Cut the QR data and get:

    - encryption (1 `u8` from prelude)
    - transaction author public key, its length matching the encryption (32 or
    33 `u8` immediately after the prelude)
    - network genesis hash (32 `u8` at the end)
    - SCALE-encoded call data and SCALE-encoded extensions as a combined blob
    (everything that remains in between the transaction author public kay and
    the network genesis hash)

    If the data length is insufficient, Signer produces an error and suggests to
load non-damaged transaction.

2. Search the Signer database for the network specs (from the network genesis
hash and encryption).

    If the network specs are not found, Signer shows:

    - public key and encryption of the transaction author key
    - error message, that suggests to add network with found genesis hash

3. Search the Signer database for the address key (from the transaction author
public key and encryption). Signer will try to interpret and display the
transaction in any case. Signing will be possible only if the parsing is
successful and the address key is known to Signer and is extended to the network
in question.

    - Address key not found. Signing not possible. Output shows:

        - public key and encryption of the transaction author key
        - call and extensions parsing result
        - warning message, that suggests to add the address into Signer
    
    - Address key is found, but it is not extended to the network used. Signing
    not possible. Output shows:
        
        - detailed author key information (base58 representation, identicon,
        address details such as address being passworded etc)
        - call and extensions parsing result
        - warning message, that suggests extending the address into the network
        used
    
    - Address key is found and is extended to the network used. Signer will
    proceed to try and interpret the call and extensions. Detailed author
    information will be output regardless of the parsing outcome. <- this is not so currently, need to fix it.
    The signing will be allowed only if the parsing is successful.
    
4. Separate the call and extensions. Call is prefixed by its length compact,
the compact is cut off, the part with length that was indicated in the compact
goes into call data, the part that remains goes into extensions data.

    If no compact is found or the length is insufficient, Signer produces an
error that call and extensions could not be separated.

5. Get the metadata set from the Signer database, by the network name from the
network specs. Metadata is used to interpret extensions and then the call
itself.

    If there are no metadata entries for the network at all, Signer produces an
error and asks to load the metadata.

    `RuntimeMetadata` versions supported by Signer are `V12`, `V13`, and `V14`.
The crucial feature of the `V14` is that the metadata contains the description
of the types used in the call and extensions production. `V12` and `V13` are
legacy versions and provide only text identifires for the types, and in order to
use them, the supplemental types information is needed.

5. Process the extensions.

    Signer already knows in which network the transaction was made, but does not
yet know the metadata version. Metadata version must be one of the signable
extensions. At the same time, the extensions and their order are recorded in the
network metadata. Thus, all metadata entries from the set are checked, from
newest to oldest, in an attempt to find metadata that both decodes the
extensions and has a version that matches the metadata version decoded from the
extensions.

    If processing extensions with a single metadata entry results in an error,
the next metadata entry is tried. The errors would be displayed to user only if
all attempts with existing metadata have failed.

    Typically, the extensions are quite stable in between the metadata versions
and in between the networks, however, they can be and sometimes are different.

    In legacy metadata (`RuntimeMetadata` version being `V12` and `V13`)
extensions have identifiers only, and in Signer the extensions for `V12` and
`V13` are hardcoded as:

    - `Era` era
    - `Compact(u64)` nonce
    - `Compact(u128)` tip
    - `u32` metadata version
    - `u32` tx version
    - `H256` genesis hash
    - `H256` block hash
    
    If the extensions could not be decoded as the standard set or not all
extensions blob is used, the Signer rejects this metadata version and adds error
into the error set.

    Metadata `V14` has extensions with both identifiers and properly described
types, and Signer decodes extensions as they are recorded in the metadata. For
this, 
[`ExtrinsicMetadata`](https://paritytech.github.io/substrate/master/frame_metadata/v14/struct.ExtrinsicMetadata.html)
part of the metadata
[`RuntimeMetadataV14`](https://paritytech.github.io/substrate/master/frame_metadata/v14/struct.RuntimeMetadataV14.html)
is used. Vector `signed_extensions` in `ExtrinsicMetadata` is scanned twice,
first for types in `ty` of the
[`SignedExtensionMetadata`](https://paritytech.github.io/substrate/master/frame_metadata/v14/struct.SignedExtensionMetadata.html)
and then for types in `additional_signed` of the `SignedExtensionMetadata`. The
types, when resolved through the types database from the metadata, allow to cut
correct length blobs from the whole SCALE-encoded extensions blob and decode
them properly.

    If any of these small decodings fails, the metadata version gets rejected by
the Signer and an error is added to the error set. Same happens if after all
extensions are scanned, some part of extensions blob remains unused.

    There are some special extensions that must be treated separately. The
`identifier` in `SignedExtensionMetadata` and `ident` segment of the type
[`Path`](https://paritytech.github.io/substrate/master/scale_info/struct.Path.html)
are used to trigger types interpretation as specially treated extensions. Each
`identifier` is encountered twice, once for `ty` scan, and once for
`additional_signed` scan. In some cases only one of those types has non-empty
content, in some cases it is both. To distinguish the two, the type-associated
path is used, which points to where the type is defined in Substrate code.
Type-associated path has priority over the identifier.

    Path triggers:

    | Path | Type is interpreted as |
    | :- | :- |
    | `Era` | `Era` |
    | `CheckNonce` | `Nonce` |
    | `ChargeTransactionPayment` | tip, gets displayed as balance with decimals and unit corresponding to the network specs |

    Identifier triggers, are used if the path trigger was not activated:

    | Identifier | Type, if not empty and if there is no path trigger, is interpreted as | Note |
    | :- | :- | :- |
    | `CheckSpecVersion` | metadata version | gets checked with the metatada version from the metadata |
    | `CheckTxVersion` | tx version | |
    | `CheckGenesis` | network genesis hash | must match the genesis hash that was cut from the tail of the transaction |
    | `CheckMortality` | block hash | must match the genesis hash if the transaction is immortal; `Era` has same identifier, but is distinguished by the path |
    | `CheckNonce` | nonce | |
    | `ChargeTransactionPayment` | tip, gets displayed as balance with decimals and unit corresponding to the network specs |

     If the extension is not a special case, it is displayed as normal parser
output and does not participate in deciding if the transaction could be signed.

    After all extensions are processed, the decoding must yield following
extensions:

    - exactly one `Era`
    - exactly one `Nonce` <- this is not so currently, fix it
    - exactly one `BlockHash`
    - exactly one `GenesisHash` <- this is not so currently, fix it
    - exactly one metadata version

    If the extension set is different, this results in Signer error for this
particular metadata version, this error goes into error set.
    
    The extensions in the metadata are checked on the metadata loading step,
long before any transactions are even produced. Metadata with incomplete
extensions causes a warning on `load_metadata` update generation step, and
another one when an update with such metadata gets loaded into Signer.
Nevertheless, such metadata loading into Signer is allowed, as there could be
other uses for metadata except signable transaction signing. Probably.
    
    If the metadata version in extensions does not match the metadata version
of the metadata used, this results in Signer error for this particular metadata
version, this error goes into error set.
    
    If the extensions are completely decoded, with correct set of the special
extensions and the metadata version from the extensions match the metadata
version of the metadata used, the extensions are considered correctly parsed,
and Signer can proceed to the call decoding.

    If all metadata entries from the Signer database were tested and no suitable
solution is found, Signer produces an error stating that all attempts to decode
extensions have failed. This could be used by variety of reasons (see above),
but so far the most common one observed was users having the metadata in Signer
not up-to-date with the metadata on chain. Thus, the error must have a
recommendation to update the metadata first.

6. Process the call data.

    After the metadata with correct version is established, it is used to parse
the call data itself. Each call begins with `u8` pallet index, this is the
decoding entry point.

    For `V14` metadata the correct pallet is found in the set of available ones
in `pallets` field of
[`RuntimeMetadataV14`](https://paritytech.github.io/substrate/master/frame_metadata/v14/struct.RuntimeMetadataV14.html),
by `index` field in corresponding
[`PalletMetadata`](https://paritytech.github.io/substrate/master/frame_metadata/v14/struct.PalletMetadata.html).
The `calls` field of this `PalletMetadata`, if it is `Some(_)`, contains
[`PalletCallMetadata`](https://paritytech.github.io/substrate/master/frame_metadata/v14/struct.PalletCallMetadata.html)
that provides the available calls enum described in `types` registry of the
`RuntimeMetadataV14`. For each type in the registry, including this calls enum,
encoded data size is determined, and the decoding is done according to the type.

    For `V12` and `V13` metadata the correct pallet is also found by scanning
the available pallets and searching for correct pallet index. Then the call is
found using the call index (second `u8` of the call data). Each call has
associated set of argument names and argument types, however, the argument type
is just a text identifier. The type definitions are not in the metadata and
transactions decoding requires supplemental types information. By default, the
Signer contains types information that was constructed for Westend when Westend
was still using `V13` metadata and it was so far reasonably sufficient for
simple transactions parsing. If the Signer does not find the type information in
the database **and** has to decode the transaction using `V12` or `V13`
metadata, error is produced, indicating that there are no types. Elsewise, for
each encountered argument type the encoded data size is determined, and the
decoding is done according to the argument type.
    
    There are types requiring special display:
    
    - calls (for cases when a call contains other calls)
    - numbers that are processed as the balances
    
    Calls in `V14` parsing are distinguished by `Call` in `ident` segment of the
type [`Path`](https://paritytech.github.io/substrate/master/scale_info/struct.Path.html).
Calls in `V12` and `V13` metadata are distinguished by any element of the set
of calls type identifiers in string argument type.

    At the moment the numbers that should be displayed as balance in
transacrtions with `V14` metadata are determined by the type name `type_name` of
the corresponding
[`Field`](https://paritytech.github.io/substrate/master/scale_info/struct.Field.html)
being:
    
    - `Balance`
    - `T::Balance`
    - `BalanceOf<T>`
    - `ExtendedBalance`
    - `BalanceOf<T, I>`
    - `DepositBalance`
    - `PalletBalanceOf<T>`

    Similar identifiers are used in `V12` and `V13`, the checked value is the
string argument type itself.

    There could be other instances when the number should be displayed as
balance. However, sometimes the balance is **not** the balance in the units
in the network specs, for example in the `assets` pallet. See issue
[#1050](https://github.com/paritytech/parity-signer/issues/1050) and comments
there for details.
    
    If no errors were encountered while parsing and all call data was used in
the process, the transaction is considered parsed and is displayed to the user,
either ready for signing (if all other checks have passed) or as read-only.

7. If the user chooses to sign the transaction, the Signer produces QR code with
signature, that should be read back into the hot side. As soon as the signature
QR code is generated, the Signer considers the transaction signed.

    All signed transactions are entered in the history log, and could be seen
and decoded again from the history log. Transactions not signed by the user do
not go in the history log.

    If the key used for the transaction is passworded, user has three attempts
to enter the password correctly. Each incorrect password entry is reflected in
the history.

    In the time interval between Signer displaying the parsed transaction and
the user approving it, the transaction details needed to generate the signature
and history log details are temporarily stored in the database. The temporary
storage gets cleared each time before and after use. Signer extracts the stored
transaction data only if the database checksum stored in navigator state is
same as the the current checksum of the database. If the password is entered
incorrectly, the database is updated with "wrong password" history entry, and
the checksum in the state gets updated accordingly. Eventually, all transaction
info can and will be moved into state itself and temporary storage will not be
used.

### Example

Alice makes transfer to Bob in Westend network.

Transaction:

`530102d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e`

| Part | Meaning | Byte position |
|:-|:-|:-|
| `53` | Substrate-related content | 0 |
| `01` | Sr25519 encryption algorithm | 1 |
| `02` | Transaction | 2 |
| `d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d` | Alice public key | 3..=34 |
| `a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817` | SCALE-encoded call data | 35..=76 |
| `a4` | Compact call data length, 41 | 35 |
| `0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817` | Call data | 36..=76 |
| `04` | Pallet index 4 in metadata, entry point for decoding | 36 |
| `b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33` | Extensions | 77..=153 |
| `e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e` | Westend genesis hash | 154..=185 |

#### Call content is parsed using Westend metadata, in this particular case westend9010

| Call part | Meaning |
|:-|:-|
| `04` | Pallet index 4 (`Balances`) in metadata, entry point for decoding |
| `03` | Method index 3 in pallet 4 (`transfer_keep_alive`), search in metadata what the method contains. Here it is `MultiAddress` for transfer destination and `Compact(u128)` balance. |
| `00` | Enum variant in `MultiAddress`, `AccountId` |
| `8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48` | Associated `AccountId` data, Bob public key |
| `0700e8764817` | `Compact(u128)` balance. Amount paid: 100000000000 or, with Westend decimals and unit, 100.000000000 mWND. |

#### Extensions content

| Extensions part | Meaning |
|:-|:-|
| `b501` | Era: phase 27, period 64 |
| `b8` | Nonce: 46 |
| `00` | Tip: 0 pWND |
| `32230000` | Metadata version: 9010 |
| `05000000` | Tx version: 5 |
| `e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e` | Westend genesis hash |
| `538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33` | Block hash |

## Message

Message has following structure:

<table>
    <tr>
        <td>prelude</td><td>public key</td><td>`<Bytes> [u8] slice </Bytes>`</td><td>network genesis hash</td>
    </tr>
</table>

The message itself is the `[u8]` slice wrapped in `<Bytes>..</Bytes>`.
Only `[u8]` slice is rendered for user, however whole payload, including
`<Bytes>..</Bytes>` wrapping is getting signed.

`[u8]` slice is represented as String if all bytes are valid UTF-8. If not all
bytes are valid UTF-8 or if the `<Bytes>..</Bytes>` wrapping is not found,
Signer produces an error.

It is critical that the message payloads are always clearly distinguishable from
the transaction payloads, i.e. it is never possible to trick user to sign
transaction posing as a message.

`<Bytes>..</Bytes>` wrapping of the messages mean that the would-be sneaked call
is done in pallet with index `60` (`<` as byte) with method `66` (`B` as byte),
and that the extensions (typically, the last one is block hash) end in
`</Bytes>`. This could be any Substrate-compatible metadata. To ensure that
`<Bytes>..</Bytes>` wrapping is safe, pallet with index `60` and with method
`66` get blocked in Substrate.

## Update

Update has following general structure:

<table>
    <tr>
        <td>prelude</td><td>verifier public key (if signed)</td><td>update payload</td><td>reserved tail</td><td>signature (if signed)</td>
    </tr>
</table>

Note that the `verifier public key` and `signature` parts appear only in signed
uploads. Preludes `[0x53, 0xff, 0x<payload code>]` are followed only by the
update payload.

| Encryption | Public key length, bytes | Signature length, bytes |
|:-|:-| :- |
| Ed25519 | 32 | 64 |
| Sr25519 | 32 | 64 |
| Ecdsa | 33 | 65 |
| no encryption | 0 | 0 |

`reserved tail` currently is not used and is expected to be empty. It could be
used later if the multisignatures are introduced for the updates. Expecting
`reserved tail` in update processing is done to keep code continuity in case
multisignatures introduction ever happens.

Because of the `reserved tail`, the `update payload` length has to be always
exactly declared, so that the `update payload` part could be cut correctly from
the update.

Detailed description of the update payloads and form in which they are used in
update itself and for generating update signature, could be found in Rust module
`definitions::qr_transfers`.

### `add_specs` update payload

Introduces a new network to Signer, i.e. adds network specs to the Signer
database.

Update payload consists of **double** SCALE-encoded `NetworkSpecsToSend` (second
SCALE is to have the exact payload length).

Payload signature is generated for SCALE-encoded `NetworkSpecsToSend`.

Network specs identifier in the Signer database is `NetworkSpecsKey`, a key
built from encryption used by the network and the network genesis hash. There
could be networks with multiple encryption algorithms supported, thus the
encryption encryption is part of the key. Network specs could be different for
networks with same genesis hash and different encryptions, except the base58
prefix. The reason is that the base58 prefix can be a part of the network
metadata, and the network metadata is not encryption-specific.

### `load_metadata` update payload

Loads metadata for a network already known to Signer, i.e. for a network with
network specs in the Signer database.

Update payload consists of concatenated SCALE-encoded metadata `Vec<u8>` and
network genesis hash (H256, always 32 bytes).

Same blob is used to generate the signature.

Network metadata identifier in the Signer database is `MetaKey`, a key built
from the network name and network metadata version.

Network metadata that can get into Signer and can be used by Signer only if it
complies with following requirements:

- metadata vector starts with `b"meta"` prelude
- part of the metadata vector after `b"meta"` prelude is decodeable as [`RuntimeMetadata`](https://paritytech.github.io/substrate/master/frame_metadata/enum.RuntimeMetadata.html)
- `RuntimeMetadata` version of the metadata is `V12`, `V13` or `V14`
- Metadata has `System` pallet
- There is `Version` constant in `System` pallet
- `Version` is decodable as [`RuntimeVersion`](https://paritytech.github.io/substrate/master/sp_version/struct.RuntimeVersion.html)
- If the metadata contain base58 prefix, it must be decodeable as `u16` or `u8`

Additionally, if the metadata `V14` is received, its associated extensions will
be scanned and user will be warned if the extensions are incompatible with
transactions signing.

Also in case of the metadata `V14` the type of the encoded data stored in the
`Version` constant is also stored in the metadata types registry and in
principle could be different from `RuntimeVersion` above. At the moment, the
type of the `Version` is hardcoded, and any other types would not be processed
and would get rejected with an error.

### `load_types` update payload

Loads types information.

Type information is needed to decode transactions made in networks with metadata
RuntimeMetadata version V12 or V13.

Most of the networks are already using RuntimeMetadata version V14, which has
types information incorporated in the metadata itself.

The `load_types` update is expected to become obsolete soon.

Update payload consists of **double** SCALE-encoded `Vec<TypeEntry>` (second
SCALE is to have the exact payload length).

Payload signature is generated for SCALE-encoded `Vec<TypeEntry>`.

### Verifiers

Signer can accept both verified and non-verified updates, however, information
once verified can not be replaced or updated by a weaker verifier without full
Signer reset.

A verifier could be `Some(_)` with corresponding public key inside or `None`.
All verifiers for the data follow trust on first use principle. If new
verifier is set up instead of the old one, all the data that was verified by the
old verifier gets removed to avoid confusion regarding which verifier has signed
the data.

General verifier is the strongest and the most reliable verifier known to the
Signer. General verifier could sign all kinds of updates. By default the Signer
uses Parity-associated key as general verifier, but users can remove it and set
their own. There could be only one general verifier at any time.

Custom verifier could be used for a network information that was verified, but
not with general verifier. There could be as many as needed custom verifiers at
any time, and custom verifier could switched to the general verifier seamlessly.

### Shared update processing sequence:

1. Cut the QR data and get:

    - encryption used by verifier (1 `u8` from prelude)
    - (only if the update is signed, i.e. the encryption is **not** `0xff`):
        
        - update verifier public key, its length matching the encryption (32 or
        33 `u8` immediately after the prelude)
        - update verifier signature, its length matching the encryption (64 or
        65 `u8` at the end)
        
    - concatenated update payload and reserved tail

    If the data length is insufficient, Signer produces an error and suggests to
load non-damaged transaction.

2. Using the payload type from the prelude, determine the update payload length
and cut payload from the reserved tail, which remains unused.

    If the data length is insufficient, Signer produces an error and suggests to
load non-damaged transaction.

3. Verify the signature for the payload. If this fails, Signer produces an error
indicating that the update signature is invalid.

### `add_specs` processing sequence

1. If the signature is valid, update payload is decoded and then processed
according to the prelude. If the decoding is not successful, Signer produces an
error.


### `load_meta` processing sequence

1. Update payload is cut into network metadata and network genesis hash.

### `load_types` processing sequence



