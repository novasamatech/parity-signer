//! This crate is intended to support the
//! [Vault](https://github.com/paritytech/parity-signer) from the active
//! (non air-gapped) side.
//!
//! This crate is mainly used to:
//!
//! - fetch network data through RPC calls
//! - prepare Vault update and derivation import payloads
//! - generate Vault update QR codes, either signed or unsigned, and
//!   derivations import QR codes, to be scanned into Vault
//! - maintain the `hot` database on the network-connected device, to store and
//!   manage the data that went into update QR codes
//! - maintain Vault default network metadata set in `defaults` crate and
//!   prepare the `cold` database for the Vault release
//!
//! # Supported Vault updates
//!
//! Crate `generate_message` can generate and the Vault can accept following
//! updates:
//!
//! - `add-specs`, to add a new network (i.e. the network specs) into the Vault
//! - `load-metadata`, to load into the Vault the network metadata, for
//!   networks that already have corresponding network specs entry in the Vault
//!   database
//! - `load-types`, to load types information (it is used to support the
//!   transactions parsing in networks with legacy metadata, `RuntimeMetadata`
//!   version below `V14`)
//!
//! Updates are assembled as `Vec<u8>` and could be transformed into:
//!
//! - `PNG` QR codes, static or dynamic multiframe depending on the data size
//! - hex-encoded string (for tests)
//!
//! Information in `add-specs`, `load-metadata` and `load-types` could be either
//! signed or unsigned. Using signed updates is strongly encouraged.
//!
//! Update has following general structure:
//!
//! <table>
//!     <tr>
//!         <td>prelude <code>[0x53, 0x<encryption code>, 0x<payload code>]</code></td>
//!         <td>verifier public key (if signed)</td>
//!         <td>update payload</td>
//!         <td>signature (if signed)</td>
//!         <td>reserved tail, currently empty</td>
//!     </tr>
//! </table>
//!
//! `<encryption code>` indicates encryption algorithm that was used
//! to sign the update:
//!
//! <table>
//!     <tr>
//!         <td><code>0x00</code></td>
//!         <td>Ed25519</td>
//!     </tr>
//!     <tr>
//!         <td><code>0x01</code></td>
//!         <td>Sr25519</td>
//!     </tr>
//!     <tr>
//!         <td><code>0x02</code></td>
//!         <td>Ecdsa</td>
//!     </tr>
//!     <tr>
//!         <td><code>0xff</code></td>
//!         <td>unsigned</td>
//!     </tr>
//! </table>
//!
//! Update payloads content is described in [definitions::qr_transfers].
//!
//! <table>
//!     <tr>
//!         <th>update payload</th>
//!         <th>update content type</th>
//!         <th>data signed, <code>to_sign</code> form</th>
//!         <th>data in payload, <code>to_transfer</code> form</th>
//!     </tr>
//!     <tr>
//!         <td><code>add_specs</code></td>
//!         <td><code>ContentAddSpecs</code></td>
//!         <td>SCALE encoded <code>NetworkSpecs</code></td>
//!         <td>double SCALE encoded <code>NetworkSpecs</code></td>
//!     </tr>
//!     <tr>
//!         <td><code>load_metadata</code></td>
//!         <td><code>ContentLoadMeta</code></td>
//!         <td>concatenated SCALE encoded metadata vector and network genesis hash</td>
//!         <td>concatenated SCALE encoded metadata vector and network genesis hash</td>
//!     </tr>
//!     <tr>
//!         <td><code>load_types</code></td>
//!         <td><code>ContentLoadTypes</code></td>
//!         <td>SCALE encoded <code>Vec&ltTypeEntry&gt</code></td>
//!         <td>double SCALE encoded <code>Vec&ltTypeEntry&gt</code></td>
//!     </tr>
//! </table>
//!
//! Note that the update payloads are build in such a way that the length of
//! the payload always could be easily found, thus allowing to separate update
//! payload, signature and reserved tail in Vault when accepting the update.
//! The tail is reserved to future-proof the updates if the multi-signing is
//! ever implemented for them. Currently the tail is empty.
//!
//! # Updates generation
//!
//! Updates are generated in following stages:
//!
//! 1. make update payload
//! 2. (optional) make signature for update payload
//! 3. make update QR code (optionally signed), that could be scanned into
//!    Vault
//!
//! Steps (1) and (3) are done in `generate_message`, the signature is produced
//! in other tools, except the test "signed" updates with Alice as a verifier,
//! when the signature is produced while making QR code during step (3).
//!
//! Signature could be produced with Subkey or with Vault. For update signing
//! it is recommended to use a dedicated key, not used for transactions. This
//! way, if the signed data was not really the update data, but something else
//! posing as the update data, the signature produced could not do any damage.
//!
//! If the Vault is used to produce the signature, it should be a dedicated
//! Vault with no verifier or weak key verifier for the network: before the
//! signature is produced, an unsigned or easily signed update must be loaded
//! into Vault.
//!
//! # Available commands
//!
//! ## Display content of the metadata `METATREE` tree of the hot database
//!
//! `$ cargo run show metadata`
//!
//! Prints for each entry in hot database [`METATREE`](constants::METATREE)
//! tree:
//!
//! - network name
//! - network metadata version
//! - hexadecimal metadata hash
//! - hexadecimal block hash for the block at which the metadata was fetched
//!
//! Note that for each network a maximum of 2 metadata entries is stored in the
//! hot database at any time.
//!
//! ## Display content of the address book `ADDRESS_BOOK` tree of the hot database
//!
//! `$ cargo run show networks`
//!
//! Prints for each entry in hot database
//! [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree:
//!
//! - address book title for the network `<network_name>-<network_encryption>`,
//!   used only to distinguish between address book entries
//! - URL address at which RPC calls are made for the network
//! - network encryption
//! - additional marker that the network is a default one, i.e. entry has not
//!   changed since the database generation
//! - network title as it will be displayed in Vault, from
//!   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
//!
//! ## Show network specs for a network, as recorded in the hot database
//!
//! `$ cargo run show specs <ADDRESS BOOK TITLE>`
//!
//! Prints network address book title and corresponding
//! [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
//! from [`SPECSTREEPREP`](constants::SPECSTREEPREP) tree of the hot
//! database.
//!
//! ### Example
//!
//! `$ cargo run show specs westend-sr25519`
//!
//! ## Check external file with hex-encoded metadata
//!
//! `$ cargo run show check-file <METADATA FILE>`
//!
//! Asserts that:
//!
//! - the file contains valid metadata, with retrievable network name and
//!   version
//! - if the metadata for same network name and version is in the hot
//!   database, it completely matches the one from the file
//!
//! ### Example
//!
//! `$ cargo run show check-file "../defaults/release_metadata/kusama9230"`
//!
//! ## Show metadata fetch block history from `META_HISTORY` tree of the hot database
//!
//! `$ cargo run show block-history`
//!
//! Prints block hashes at which the network metadata was fetched as it first
//! got in the database. If the metadata is from `.wasm` file, there is no entry
//! until a proper metadata fetch from a node is done with some associated block
//! hash.
//!
//! [`META_HISTORY`](constants::META_HISTORY) tree stores all block hashes that
//! were ever encountered on successful new metadata fetch, and clears only on
//! the database reset.
//!
//! Block hashes could be useful should silent metadata updates (metadata change
//! with no version bump) happen again.
//!
//! ## Prepare `add_specs` update payload
//!
//! `$ cargo run add-specs [OPTIONS] <-d|-f|-k|-p|-t> <--all|--name <NAME>|--url <ADDRESS>>`
//!
//! A file is generated in dedicated [`FOLDER`](constants::FOLDER) to
//! (optionally) be signed and later be transformed into `add_specs` update
//! QR. Output file name is `sign_me_add_specs_<network_name>_<encryption>`.
//!
//! Setting keys that could be used in command line (maximum one):
//!
//! - `-d`: do **not** update the database, make RPC calls, and produce
//!   output files
//! - `-f`: do **not** run RPC calls, produce output files using data already in
//!   the database
//! - `-p`: update or check database through RPC calls, do **not** produce any
//!   output files
//! - `-t` (no setting key defaults here): update or check database through RPC
//!   calls, produce output files
//!
//! <table>
//!     <tr>
//!         <th>setting key</th>
//!         <th>hot database update</th>
//!         <th>RPC calls</th>
//!         <th>output update payload</th>
//!     </tr>
//!     <tr>
//!         <td><code>-d</code></td>
//!         <td>-</td>
//!         <td>+</td>
//!         <td>+</td>
//!     </tr>
//!     <tr>
//!         <td><code>-f</code></td>
//!         <td>-</td>
//!         <td>-</td>
//!         <td>+</td>
//!     </tr>
//!     <tr>
//!         <td><code>-p</code></td>
//!         <td>+</td>
//!         <td>+</td>
//!         <td>-</td>
//!     </tr>
//!     <tr>
//!         <td><code>-t</code></td>
//!         <td>+</td>
//!         <td>+</td>
//!         <td>+</td>
//!     </tr>
//! </table>
//!
//! Reference keys (exactly only one has to be used):
//!
//! - `--all`: all networks with entries in the
//!   [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree of the hot database
//! - `--name` followed by single network address book title: for a network with
//!   existing record in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
//! - `--url` followed by single URL address: reserved for networks with no
//!   record yet in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
//!
//! `--all` key could be used with `--pass-errors` key, to stop processing after first
//! error.
//!
//! `--encryption` key to override specifying encryption algorithm supported by the
//! network is optional for `--name` reference key (since there is already an entry in
//! the database with specified encryption) and mandatory for `--url` reference key.
//! Supported variants are:
//!
//! - `ed25519`
//! - `sr25519`
//! - `ecdsa`
//!
//! Sequence invoking token override could be used when processing an
//! individual network that has multiple allowed decimals and unit values
//! retrieved as arrays of equal size. To override token, key `--token-decimals`
//! followed by `u8` decimals value and key `--token-unit` `String` unit value is used.
//! By default, if no token override in provided, such networks have `0u8` decimals
//! and `UNIT` unit set up.
//!
//! Title override could be used when processing an individual network, to set
//! the title under which the network will be displayed in Vault, should the
//! `add-specs` payload be accepted. Non-default networks, if the title override
//! is not specified, have title `<network_name>-<network_encryption>`.
//!
//! Not all setting and reference key combinations are compatible, and not all
//! overrides are supported. Users are encouraged to comment if they need some
//! other than current key combinations available.
//!
//! <table>
//!     <tr>
//!         <th>setting key</th>
//!         <th>reference key</th>
//!         <th>reference argument</th>
//!         <th>encryption override</th>
//!         <th>token override</th>
//!         <th>title override</th>
//!         <th>action</th>
//!     </tr>
//!     <tr>
//!         <td><code>-d</code></td>
//!         <td><code>-u</code></td>
//!         <td><code>url_address</code></td>
//!         <td>mandatory</td>
//!         <td>possible, if token array fetched</td>
//!         <td>possible</td>
//!         <td>- make RPC calls<br>
//!             - apply overrides<br>
//!             - make payload file<br>
//!             Note: database is <b>not</b> used
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-f</code></td>
//!         <td><code>-a</code></td>
//!         <td></td>
//!         <td colspan="3">blocked</td>
//!         <td>- get all network specs entries from the database<br>
//!             - make payload file(s)<br>
//!             Note: only the data from the database is used
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-f</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>address_book_title</code></td>
//!         <td>possible</td>
//!         <td>blocked, no way to check that the token override is reasonable</td>
//!         <td>possible</td>
//!         <td>- get address book entry for <code>address_book_title</code><br>
//!             - get corresponding network specs entry<br>
//!             - apply overrides<br>
//!             - make payload file<br>
//!             Note: only the data from the database and override(s) are used
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-p</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>address_book_title</code></td>
//!         <td>possible</td>
//!         <td>possible, if token array fetched</td>
//!         <td>possible</td>
//!         <td>- get address book entry for <code>address_book_title</code><br>
//!             - get corresponding network specs entry<br>
//!             - make RPC calls, check that the entry remains correct<br>
//!             - apply overrides<br>
//!             - update database
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-p</code></td>
//!         <td><code>-u</code></td>
//!         <td><code>url_address</code></td>
//!         <td>mandatory</td>
//!         <td>possible, if token array fetched</td>
//!         <td>possible</td>
//!         <td>- make RPC calls<br>
//!             - apply overrides<br>
//!             - update database<br>
//!             Note: reserved for networks with no entries in the database
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-t</code> or none declared</td>
//!         <td><code>-n</code></td>
//!         <td><code>address_book_title</code></td>
//!         <td>possible</td>
//!         <td>possible, if token array fetched</td>
//!         <td>possible</td>
//!         <td>- get address book entry for <code>address_book_title</code><br>
//!             - get corresponding network specs entry<br>
//!             - make RPC calls, check that the entry remains correct<br>
//!             - apply overrides<br>
//!             - update database if needed<br>
//!             - make payload file
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-t</code> or none declared</td>
//!         <td><code>-u</code></td>
//!         <td><code>url_address</code></td>
//!         <td>mandatory</td>
//!         <td>possible, if token array fetched</td>
//!         <td>possible</td>
//!         <td>- make RPC calls<br>
//!             - apply overrides<br>
//!             - update database if needed<br>
//!             - make payload file<br>
//!             Note: reserved for networks with no entries in the database
//!         </td>
//!     </tr>
//! </table>
//!
//! ### Examples
//!
//! Make `add_specs` update payload for a known network from the hot database:
//!
//! `$ cargo run add-specs --name westend-sr25519`
//!
//! Make `add-specs` update payload for a new network:
//!
//! `$ cargo run add-specs -d -u wss://rococo-rpc.polkadot.io --encryption sr25519 --title Rococo`
//!
//! Make `add-specs` update payload for a new network with token set:
//!
//! `$ cargo run add-specs -d -u wss://acala.polkawallet.io --encryption sr25519 --token-decimals 12 --token-unit ACA --title Acala`
//!
//! ## Prepare `load_metadata` update payload
//!
//! `$ cargo run load-metadata [OPTIONS] <-d|-f|-k|-p|-t>`
//!
//! A file is generated in dedicated [`FOLDER`](constants::FOLDER) to
//! (optionally) be signed and later be transformed into `load_metadata`
//! update QR. Output file name is
//! `sign_me_load_metadata_<network_name>V<version>`.
//!
//! Setting keys that could be used in command line (maximum one):
//!
//! - `-d`: do **not** update the database, make RPC calls, and produce
//!   output files
//! - `-f`: do **not** run RPC calls, produce output files from database as
//!   it is
//! - `-k`: update database through RPC calls, produce output files only for
//!   **new** database entries
//! - `-p`: update database through RPC calls, do **not** produce any output
//!   files
//! - `-t` (no setting key defaults here): update database through RPC
//!   calls, produce output files
//!
//! <table>
//!     <tr>
//!         <th>setting key</th>
//!         <th>hot database update</th>
//!         <th>RPC calls</th>
//!         <th>output update payload</th>
//!     </tr>
//!     <tr>
//!         <td><code>-d</code></td>
//!         <td>-</td>
//!         <td>+</td>
//!         <td>+</td>
//!     </tr>
//!     <tr>
//!         <td><code>-f</code></td>
//!         <td>-</td>
//!         <td>-</td>
//!         <td>+</td>
//!     </tr>
//!     <tr>
//!         <td><code>-k</code></td>
//!         <td>+</td>
//!         <td>+</td>
//!         <td>only new entries</td>
//!     </tr>
//!     <tr>
//!         <td><code>-p</code></td>
//!         <td>+</td>
//!         <td>+</td>
//!         <td>-</td>
//!     </tr>
//!     <tr>
//!         <td><code>-t</code></td>
//!         <td>+</td>
//!         <td>+</td>
//!         <td>+</td>
//!     </tr>
//! </table>
//!
//! Network metadata updates quite often, compared to `add-specs` command there
//! is also setting key `-k` to print only the data that was not in the hot
//! database before the fetch.
//!
//! Reference keys (exactly only one has to be used):
//!
//! - `-a,--all`: all networks with entries in the
//!   [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree of the hot database
//! - `-n,--name` followed by single network name: for a network with existing
//!   record in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
//! - `-u,--url` followed by single URL address: reserved for networks with no
//!   record yet in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
//!
//! `-a` key could be used with `--pass-errors` key, to stop processing after first
//! error.
//!
//! `load-metadata` has no overrides available. Not all setting and reference
//! key combinations are compatible, and not all overrides are supported. Users
//! are encouraged to comment if they need some other than current key
//! combinations available.
//!
//! <table>
//!     <tr>
//!         <th>setting key</th>
//!         <th>reference key</th>
//!         <th>reference argument</th>
//!         <th>action</th>
//!     </tr>
//!     <tr>
//!         <td><code>-d</code></td>
//!         <td><code>-a</code></td>
//!         <td></td>
//!         <td>- get all URL addresses <b>from the database</b><br>
//!             - make RPC calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - make payload file(s)<br>
//!             Note: database is needed to get URL addresses
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-d</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get URL address <b>from the database</b> for the <code>network_name</code><br>
//!             - make RPC calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - make payload file<br>
//!             Note: database is needed to get URL address
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-d</code></td>
//!         <td><code>-u</code></td>
//!         <td><code>url_address</code></td>
//!         <td>- make RPC calls<br>
//!             - make payload file<br>
//!             Note: database is <b>not</b> used
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-f</code></td>
//!         <td><code>-a</code></td>
//!         <td></td>
//!         <td>- get all metadata entries from the database<br>
//!             - make payload file(s)
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-f</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get all metadata entries for the <code>network_name</code> from the database<br>
//!             - make payload file(s)
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-k</code></td>
//!         <td><code>-a</code></td>
//!         <td></td>
//!         <td>- get all URL addresses from the database<br>
//!             - make RPC calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed<br>
//!             - make payload file for each new entry
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-k</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get URL address from the database for the <code>network_name</code><br>
//!             - make RPC calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed<br>
//!             - make payload file if the entry is new
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-p</code></td>
//!         <td><code>-a</code></td>
//!         <td></td>
//!         <td>- get all URL addresses from the database<br>
//!             - make RPC calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-p</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get URL address from the database for the <code>network_name</code><br>
//!             - make RPC calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-t</code> or none declared</td>
//!         <td><code>-a</code></td>
//!         <td></td>
//!         <td>- get all URL addresses from the database<br>
//!             - make RPC calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed<br>
//!             - make payload file(s)
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-t</code> or none declared</td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get URL address from the database for the <code>network_name</code><br>
//!             - make RPC calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed<br>
//!             - make payload file
//!         </td>
//!     </tr>
//! </table>
//!
//! ### Examples
//!
//! Check metadata updates and make `load_metadata` update payloads for latest
//! metadata for all known networks:
//!
//! `$ cargo run load-metadata -a`
//!
//! Make `load_metadata` update payload for a network not in the database:
//!
//! `$ cargo run load-metadata -d -u wss://rococo-rpc.polkadot.io`
//!
//! ## Prepare `load_types` update payload
//!
//! `$ cargo run load-types`
//!
//! A file is generated in dedicated [`FOLDER`](constants::FOLDER) to
//! (optionally) be signed and later be transformed into `load_types` update QR.
//! Output file name is `sign_me_load_types`.
//!
//! ## Generate update QR and/or hexadecimal string file
//!
//! Raw `[u8]` update payloads, as prepared by `add_specs`, `load_metadata` or
//! `load_types` commands get transformed into update QR codes (to be scanned
//! into the Vault) or textfiles with hexadecimal data (for tests).
//!
//! There are two commands for generating updates: `make` and `sign`.
//!
//! Command `make` is used to generate:
//!
//! - signed updates with a valid signature, associated public key and
//!   encryption algorithm
//! - test signed updates, i.e. updates signed by a key with
//!   [Alice seed phrase](constants::ALICE_SEED_PHRASE) and derivation `//Alice`,
//!   with encryption algorithm chosen by user, for tests
//! - unsigned updates
//!
//! Signature for `make` command is generated for contents of raw `[u8]` update
//! payload file using, for example, Subkey.
//!
//! Command `sign` is used to generate signed updates with a valid
//! [`SufficientCrypto`](definitions::crypto::SufficientCrypto) produced by
//! Vault. Vault exports `SufficientCrypto` produced for one of its keys
//! as a static QR code, this QR code content goes into command line.
//!
//! Update QR and/or hexadecimal string file are produced in
//! [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
//!
//! Keys and most arguments (except file paths) are not case-sensitive.
//!
//! The validity of the signature or `SufficientCrypto`, if provided, is checked
//! before assembling update.
//!
//! The payload is checked to be valid, with decodeable content. If default
//! output file name is used, it is generated based on the payload content.
//!
//! <table>
//!     <tr>
//!         <th><code>msg</code></th>
//!         <th>default update file name</th>
//!     </tr>
//!     <tr>
//!         <td><code>add-specs</code></td>
//!         <td><code>add_specs_&ltnetwork_name&gt-&ltnetwork_encryption&gt</code></td>
//!     </tr>
//!     <tr>
//!         <td><code>load-metadata</code></td>
//!         <td><code>load_metadata_&ltnetwork_name&gtV&ltmetadata_version&gt</code></td>
//!     </tr>
//!     <tr>
//!         <td><code>load-types</code></td>
//!         <td><code>load_types</code></td>
//!     </tr>
//! </table>
//!
//! Names for Alice-signed updates have additional tail
//! `_Alice-<alice_signature_encryption>`.
//!
//! Names for unsigned updates have additional tail `_unsigned`.
//!
//! ### `make` command
//!
//! `$ cargo run make <keys> <arguments>`
//!
//! Keys to be used in command line:
//!
//! - Key `--goal` followed by the type to generate
//!    - `qr` will generate only a png QR code
//!    - `text` will generate only text file with hex-encoded update.
//!    - default, i.e. if goal is not provided, both QR code and text file are generated.
//!
//! - Key `--crypto` followed by encryption used to make update signature:
//!    - `ed25519`
//!    - `sr25519`
//!    - `ecdsa`
//!    - `none` if the message is not verified
//!
//! - Key `--msg` followed by update type:
//!    - `load-types`
//!    - `load-metadata`
//!    - `add-specs`
//!
//! - Key `--verifier` (can be entered only if the `--crypto` argument was
//!   `ed25519`, `sr25519`, or `ecdsa`), followed by:
//!    - `Alice` to generate messages "verified" by
//!       [Alice seed phrase](constants::ALICE_SEED_PHRASE) with derivation `//Alice`
//!    - `-hex` followed by hex public key
//!    - `-file` followed by the path in dedicated [`FOLDER`](constants::FOLDER)
//!       for file with public key as raw bytes
//!
//! - Key `--payload` followed by file path in dedicated
//!   [`FOLDER`](constants::FOLDER) containing already generated payload as
//!   raw bytes
//!
//! - Key `--signature` (can be entered only if the `--crypto` argument was
//!   `ed25519`, `sr25519`, or `ecdsa` **and** `--verifier` is not `Alice`),
//!   followed by:
//!    - `-hex` followed by hex signature
//!    - `-file` followed by the path in dedicated [`FOLDER`](constants::FOLDER)
//!       for file with signature as raw bytes
//!
//! - Optional key `-name` followed by path override for export file in
//!   dedicated [`EXPORT_FOLDER`](constants::EXPORT_FOLDER)
//!
//! ### `sign` command
//!
//! `$ cargo run make <keys> <arguments>`
//!
//! Keys to be used in command line:
//!
//! - Key `--goal` followed by the type to generate
//!    - `qr` will generate only a png QR code
//!    - `text` will generate only text file with hex-encoded update.
//!    - default, i.e. if goal is not provided, both QR code and text file are generated.
//!
//! - Key `-sufficient` followed by:
//!    - `-hex` followed by hexadecimal string with contents of Vault-produced
//!       `SufficientCrypto` QR code
//!    - `-file` followed by file path in dedicated
//!       [`FOLDER`](constants::FOLDER) for raw bytes file with contents of
//!       Vault-produced `SufficientCrypto` QR code
//!
//! - Key `-msg` followed by message type:
//!    - `load-types`
//!    - `load-metadata`
//!    - `add-specs`
//!
//! - Key `--payload` followed by file path in dedicated
//!   [`FOLDER`](constants::FOLDER) containing already generated payload as
//!   raw bytes
//!
//! - Optional key `-name` followed by path override for export file in
//!   dedicated [`EXPORT_FOLDER`](constants::EXPORT_FOLDER)
//!
//! Generating `SufficientCrypto` in Vault is suggested mainly for update
//! distribution purposes. A dedicated (i.e. used only for updates signing),
//! kept physically safe Vault is strongly suggested, with a dedicated key
//! for updates signing. As the Vault can accept only payloads with
//! verifier not weaker than the one used before, and the whole purpose of
//! the process is to generate a signature for payload, it is expected that
//! this isolated Vault will receive unsigned or weakly signed updates,
//! thoroughly check them and export `SufficientCrypto`, so that a signed
//! update could be made for other, routinely used Vault devices.
//!
//! ### Examples: generate `load_metadata` QR code for westend metadata version 9200.
//!
//! Update payload `sign_me_load_metadata_westendV9200` is already in dedicated
//! [`FOLDER`](constants::FOLDER).
//!
//! #### `make` for external signature
//!
//! `$ cargo run make --goal qr --crypto <encryption> --msg load-metadata
//! --verifier-hex <public key> --payload sign_me_load_metadata_westendV9200
//! --signature-hex <signature>`
//!
//! Here `<signature>` is hexadecimal signature generated for the contents of
//! the payload file for `<public_key>` using `<encryption>` algorithm.
//!
//! Output file is `load_metadata_westendV9200` in
//! [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
//!
//! Example:
//!
//! `$ cargo run make --goal qr --crypto sr25519 --msg load-metadata --verifier-hex
//! 46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a --payload
//! sign_me_load_metadata_westendV9200 --signature-hex
//! 125717599cd057bfe6db7b111274cbda796d2543467400110552fa1c62dc087a7acefb53b68716f1e34f8af6bf13ab45d70d50655fd39483c64f3f057418748a`
//!
//! #### `make` for test verifier Alice
//!
//! Payloads signed by Alice are used for testing in Vault. The signature
//! in this case is generated automatically and is not supplied in command
//! line.
//!
//! `$ cargo run make --goal qr --crypto <encryption> --msg load-metadata
//! --verifier Alice --payload sign_me_load_metadata_westendV9200`.
//!
//! Output file is `load_metadata_westendV9200_Alice-<encryption>` in
//! [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
//!
//! Example:
//!
//! `$ cargo run make --goal qr --crypto sr25519 --msg load-metadata --verifier Alice
//! --payload sign_me_load_metadata_westendV9200`
//!
//! #### `make` with no signature
//!
//! `$ cargo run make --goal qr --crypto none --msg load-metadata --payload
//! sign_me_load_metadata_westendV9200`
//!
//! Output file is `load_metadata_westendV9200_unverified` in
//! [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
//!
//! Example:
//!
//! `$ cargo run make --goal qr --crypto none --msg load-metadata --payload
//! sign_me_load_metadata_westendV9200`
//!
//! #### `sign`
//!
//! Here `<hex_sufficient>` is hex-encoded data from
//! [`SufficientCrypto`](definitions::crypto::SufficientCrypto) QR code produced
//! by the Vault.
//!
//! `$ cargo run sign --goal qr --sufficient-hex <hex_sufficient> --msg
//! load-metadata --payload sign_me_load_metadata_westendV9200`
//!
//! Output file is `load_metadata_westendV9200` in
//! [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
//!
//! Example:
//!
//! `$ cargo run sign --goal qr --sufficient-hex
//! 0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47aceef7c58b5f952b6233b8aba5beb6f0000c8ca7f7cc16b7ada7cd45026fc3f3ec2289dd90dab0dfac38dfe3be843231443ddd30a3f3bbabb5cefcd2bbcef908c
//! --msg load-metadata --payload sign_me_load_metadata_westendV9200`
//!
//! ## Remove a single metadata entry from the `METATREE`
//!
//! `$ cargo run remove --name <network_name> --version <metadata_version>`
//!
//! Removes only the specified entry from the [`METATREE`](constants::METATREE).
//!
//! The entry in [`META_HISTORY`](constants::META_HISTORY) remains. Should the
//! same metadata version be retrieved afterwards, the `META_HISTORY` entry will
//! be updated to a block hash from more recent fetch, as the metadata from
//! old block saved in the database would not be necessarily the same as the one
//! being recorded in the database now.
//!
//! ## Remove all data associated with a network
//!
//! `$ cargo run remove --title <address_book_title>`
//!
//! This will remove:
//! - address book entry
//!   [`AddressBookEntry`](definitions::metadata::AddressBookEntry) from
//!   [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree
//! - network specs
//!   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
//!   from [`SPECSTREEPREP`](constants::SPECSTREEPREP) tree
//! - all associated metadata entries from [`METATREE`](constants::METATREE)
//!   if there are no other address book entries this metadata is associated
//!   with
//! - all associated meta block history entries from
//!   [`META_HISTORY`](constants::META_HISTORY) if there are no other address book
//!   entries this block history entries are associated with
//!
//! ## Restore hot database to default state
//!
//! `$ cargo run restore-defaults`
//!
//! Removes old hot database and generates new one with default values at
//! default path [`HOT_DB_NAME`](constants::HOT_DB_NAME).
//!
//! By default, hot database contains:
//!
//! - [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) entries for default networks
//! - [`SPECSTREEPREP`](constants::SPECSTREEPREP) entries for default networks
//! - types information in [`SETTREE`](constants::SETTREE)
//! - **no** metadata entries in [`METATREE`](constants::METATREE)
//! - **no** meta block history entries in
//!   [`META_HISTORY`](constants::META_HISTORY)
//!
//! Default networks are Polkadot, Kusama, and Westend.
//!
//! ## Generate default cold release database
//!
//! `$ cargo run make-cold-release <optional path>`
//!
//! Removes old cold release database and generates new one with default values
//! (unitiniated) at user-provided path or, if no valid path is given, at
//! default path [`COLD_DB_NAME_RELEASE`](constants::COLD_DB_NAME_RELEASE).
//!
//! By default, the uninitiated cold release database contains:
//!
//! - [`SPECSTREE`](constants::SPECSTREE) entries for default networks
//! - [`VERIFIERS`](constants::VERIFIERS) entries for default networks, with
//!   verifiers set to the general one
//! - two latest metadata versions for default networks in
//!   [`METATREE`](constants::METATREE)
//! - default types information and clean danger status in
//!   [`SETTREE`](constants::SETTREE)
//!
//! Note that the general verifier is not specified and history is not
//! started. This will be done only in Vault itself. Before initialization,
//! the cold release database could not be used by Vault.
//!
//! ## Transfer metadata from hot database to cold release database
//!
//! `$ cargo run transfer_meta_to_cold_release <optional path>`
//!
//! Metadata from hot database is transferred to cold release database at
//! user-provided path or, if no valid path is given, at default path
//! [`COLD_DB_NAME_RELEASE`](constants::COLD_DB_NAME_RELEASE).
//!
//! Metadata is transferred only for the networks that are known to the cold
//! database, i.e. the ones having
//! [`OrderedNetworkSpecs`](definitions::network_specs::OrderedNetworkSpecs) entry in
//! [`SPECSTREE`](constants::SPECSTREE).

//! ## Make derivations import QR and/or hexadecimal string file
//!
//! `$ cargo run derivations --goal <GOAL> --title <TITLE> --derivations <DERIVATIONS>`
//!
//! Keys to be used in command line:
//!
//! - `<GOAL>`: `qr` will generate only apng QR code, `text`
//!   will generate only text file with hex-encoded update. By default, i.e. if
//!   content key is not provided, both QR code and text file are generated.
//!   `<optional_target_key>` is expected immediately after `derivations` command,
//!   if at all; keys to follow could go in any order, but with argument
//!   immediately following the key.
//!
//! - Key `--derivations` followed by file path in `/generate_message/` folder.
//!   File with derivations contains valid derivations, each on its own line. Only
//!   suitable derivations will be processed. Processed derivations are also
//!   printed for user to check.
//!
//! - Key `--title` followed by network address book title, to indicate to
//!   which network the derivations belong.
//!
//! Output file is in `/generate_message/` folder, file name would be
//! `derivations-<address_book_title>`.
//!
//! ## Prepare payload for `load_metadata` update from `.wasm` file
//!
//! `$ cargo run unwasm [OPTIONS] --filename <FILENAME>`
//!
//! This command extracts metadata from `.wasm` file and uses this metadata to
//! produce `load_metadata` update payload. Only networks with network specs
//! entries in the hot database could be processed with `unwasm` command, since
//! the `load_metadata` update payload in addition to metadata requires also
//! network genesis hash. `unwasm` command could be used to generate update QR
//! codes before the metadata becomes accessible from the node.
//!
//! Network name found in the metadata is used to find
//! [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) for
//! the network. `NetworkSpecs` are used to get genesis hash and to check
//! base58 prefix, it the network metadata has base58 prefix inside.
//!
//! A raw bytes update payload file is generated in dedicated
//! [`FOLDER`](constants::FOLDER) to (optionally) be signed and later be
//! transformed into `load_metadata` update QR. Update payload file name is
//! `sign_me_load_metadata_<network_name>V<version>`.
//!
//! By default, metadata extracted from `.wasm` file is added to the database.
//! Optional `-d` key could be used is database should **not** be updated.
//! If the metadata gets entered in the database (i.e. no `-d` key used),
//! [`META_HISTORY`](constants::META_HISTORY) gets no entry. Block hash will be
//! added if the same metadata is later fetched from a node.
//!
//! ## Make metadata file for `defaults` release metadata set
//!
//! `$ cargo run  meta-default-file --name <NETWORK NAME> --version <NETWORK VERSION>`
//!
//! Produces file with hex-encoded network metadata from the hot database
//! [`METATREE`](constants::METATREE) entry.
//!
//! Output file named `<network_name><metadata_version>` is generated in
//! dedicated [`EXPORT_FOLDER`](constants::EXPORT_FOLDER). It contains
//! hexadecimal network metadata.
//!
//! ### Example
//!
//! `$ cargo run meta-default-file --name westend --version 9230`
//!
//! ## Make file with hexadecimal network metadata fetched for specific block hash from provided address
//!
//! `$ cargo run meta-at-block --url <RPC URL> --block-hash <BLOCK HASH>`
//!
//! Output file named `<network_name><metadata_version>_<block_hash>` is
//! generated in dedicated [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
//! It contains hexadecimal network metadata.
//!
//! This command does not address or update the hot database.
//!
//! ### Example
//!
//! `$ cargo run meta-at-block --url wss://westend-rpc.polkadot.io --block
//! 780812df50c4006d1865742269fe4ca339c097e61d6279cce91ebc58f5aebada`
#![deny(unused)]
#![deny(rustdoc::broken_intra_doc_links)]

use constants::FPS_DEN;
use db_handling::{
    default_cold_release, default_hot,
    helpers::{prep_types, transfer_metadata_to_cold},
};

pub mod fetch_metadata;
pub mod helpers;
use helpers::{
    debug_meta_at_block, generate_bulk_transaction_qr, generate_key_info_export_to_qr,
    generate_qr_code,
};
pub mod interpret_specs;
mod load;
use load::{gen_load_meta, meta_default_file, unwasm};
mod make_message;
use make_message::make_message;
pub mod parser;
use parser::{Command, Show};
mod remove;
use remove::remove_info;
mod show;
use show::{check_file, show_block_history, show_metadata, show_networks, show_specs};
mod specs;
use specs::gen_add_specs;

mod error;
pub use error::{Error, Result};

/// Process incoming command as interpreted by parser.
pub fn full_run(command: Command) -> Result<()> {
    match command {
        Command::Show { s: show, db_path } => {
            let database = sled::open(db_path)?;
            match show {
                Show::Metadata => show_metadata(&database),
                Show::Networks => show_networks(&database),
                Show::Specs { s: title } => show_specs(&database, title),
                Show::CheckFile { s: path } => check_file(&database, path),
                Show::BlockHistory => show_block_history(&database),
            }
        }
        Command::Specs { s: instruction } => gen_add_specs(instruction),
        Command::Load(instruction) => gen_load_meta(instruction),
        Command::Types { db_path, files_dir } => {
            let database = sled::open(db_path)?;
            Ok(prep_types(&database)?.write(files_dir.join("sign_me_load_types"))?)
        }
        Command::Sign(make) | Command::Make(make) => make_message(make),
        Command::Remove { r: info, db_path } => {
            let database = sled::open(db_path)?;
            remove_info(&database, info)
        }
        Command::RestoreDefaults { db_path } => {
            let db = sled::open(db_path)?;

            Ok(default_hot(Some(&db))?)
        }
        Command::MakeColdRelease { path } => {
            let db = path.and_then(|path| sled::open(path).ok());

            Ok(default_cold_release(db.as_ref())?)
        }
        Command::TransferMetaToColdRelease { cold_db, hot_db } => {
            let hot_db = sled::open(hot_db)?;
            let cold_db = sled::open(cold_db)?;
            Ok(transfer_metadata_to_cold(&hot_db, &cold_db)?)
        }
        Command::Unwasm {
            filename,
            update_db,
            db_path,
            files_dir,
        } => {
            let database = sled::open(db_path)?;
            unwasm(&database, &filename, update_db, files_dir)
        }
        Command::MetaDefaultFile {
            name,
            version,
            db_path,
            export_dir,
        } => {
            let database = sled::open(db_path)?;
            meta_default_file(&database, &name, version, export_dir)
        }
        Command::MetaAtBlock {
            url,
            block_hash,
            export_dir,
        } => debug_meta_at_block(&url, &block_hash, export_dir),
        Command::EncodeToQr {
            path,
            hex,
            chunk_size,
            dst_file,
        } => {
            let data = if let Some(hex) = hex {
                hex::decode(hex).unwrap()
            } else if let Some(path) = path {
                std::fs::read(path).unwrap()
            } else {
                panic!("path or hex data required");
            };

            generate_qr_code(&data, chunk_size, FPS_DEN, dst_file)
        }
        Command::KeyInfoExportToQr {
            dst_file,
            chunk_size,
            fps,
            keys_num,
        } => generate_key_info_export_to_qr(dst_file, chunk_size, fps, keys_num),
        Command::BulkTransactionTestPayload {
            dst_file,
            tx_count,
            chunk_size,
            from,
            output_format,
        } => generate_bulk_transaction_qr(dst_file, tx_count, chunk_size, from, output_format),
    }
}
