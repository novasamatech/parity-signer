//! This crate is intended to support the
//! [Signer](https://github.com/paritytech/parity-signer) from the active
//! (non air-gapped) side.
//!
//! This crate is mainly used to:
//!
//! - fetch network data through rpc calls
//! - prepare Signer update and derivation import payloads
//! - generate Signer update QR codes, either signed or unsigned, and
//! derivations import QR codes, to be scanned into Signer
//! - maintain the `hot` database on the network-connected device, to store and
//! manage the data that went into update QR codes
//! - maintain Signer default network metadata set in `default` crate and
//! prepare the `cold` database for the Signer release
//!
//! # Supported Signer updates
//!
//! Crate `generate_message` can generate and the Signer can accept following
//! updates:
//!
//! - `add_specs`, to add a new network (i.e. the network specs) into the Signer
//! - `load_metadata`, to load into the Signer the network metadata, for
//! networks that already have corresponding network specs entry in the Signer
//! database
//! - `load_types`, to load types information (it is used to support the
//! transactions parsing in networks with legacy metadata, `RuntimeMetadata`
//! version below V14)
//!
//! Updates are assembled as `Vec<u8>` and could be transformed into:
//!
//! - `png` QR codes, static or dynamic multiframe depending on the data size
//! - hex-encoded string (for tests)
//!
//! Information in `add_specs`, `load_metadata` and `load_types` could be either
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
//!         <td>SCALE encoded <code>NetworkSpecsToSend</code></td>
//!         <td>double SCALE encoded <code>NetworkSpecsToSend</code></td>
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
//! payload, signature and reserved tail in Signer when accepting the update.
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
//! Signer
//!
//! Steps (1) and (3) are done in `generate_message`, the signature is produced
//! in other tools, except the test "signed" updates with Alice as a verifier,
//! when the signature is produced while making QR code during step (3).
//!
//! Signature could be produced with Subkey or with Signer. For update signing
//! it is recommended to use a dedicated key, not used for transactions. This
//! way, if the signed data was not really the update data, but something else
//! posing as the update data, the signature produced could not do any damage.
//!
//! If the Signer is used to produce the signature, it should be a dedicated
//! Signer with no verifier or weak key verifier for the network: before the
//! signature is produced, an unsigned or easily signed update must be loaded
//! into Signer.
//!
//! # Derivations import
//!
//! Crate `generate_message` can generate derivations import for bulk import of
//! password-free derivations.
//!
//! Derivations import has following structure:
//!
//! <table>
//!     <tr>
//!         <td>prelude</td>
//!         <td>derivations import payload</td>
//!     </tr>
//! </table>
//!
//! Derivations imports are unsigned, and always have the same prelude,
//! `53ffde`. The payload content is
//! [`ContentDerivations`](definitions::qr_transfers::ContentDerivations) in
//! `to_transfer` form.
//!
//! Derivations import data is assembled as `Vec<u8>` and could be transformed
//! into:
//!
//! - `png` QR code, static or dynamic multiframe depending on the data size
//! - hex-encoded string (for tests)
//!
//! Derivations imports are generated from user-provided derivations list and
//! network information. User provides network address book title when
//! generating the update, the update itself contains network genesis hash and
//! [`Encryption`](definitions::crypto::Encryption).
//!
//! Only password-free derivation are getting in the update. On generation the
//! user-provided derivation list is searched for valid derivations: each line
//! is a separate derivation, only soft (`/`) and hard (`//`) derivations are
//! allowed, any incorrectly formatted or passworded (with `///<password>` part)
//! derivations are skipped. `generate_message` prints the suitable derivations
//! found.
//!
//! When the update is scanned into the Signer, only password-free valid
//! derivations are expected to be found in the derivations set, otherwise the
//! Signer will produce an error. If derivations set gets accepted for a certain
//! seed, Signer tries to create derived keys for all derivations.
//!
//! If a derivation produces exactly same public key with exactly same
//! derivation path as already in the database or in the import, it get ignored
//! causing no error. If a derivation produces same public key as already in the
//! database or in the import, but with **different** derivation path, it causes
//! an error and all derivations set gets rejected. Note that currently the
//! checking happens only after the seed is already fed into the Signer.
//!
//! # Available commands
//!
//! ## Display content of the metadata `METATREE` tree of the hot database
//!
//! `$ cargo run show -metadata`
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
//! `$ cargo run show -networks`
//!
//! Prints for each entry in hot database
//! [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree:
//!
//! - address book title for the network `<network_name>-<network_encryption>`,
//! used only to distinguish between address book entries
//! - url address at which rpc calls are made for the network
//! - network encryption
//! - additional marker that the network is a default one, i.e. entry has not
//! changed since the database generation
//! - network title as it will be displayed in Signer, from
//! [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
//!
//! ## Show network specs for a network, as they are recorded in the hot database
//!
//! `$ cargo run show -specs <address_book_title>`
//!
//! Prints network address book title and corresponding
//! [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
//! from [`SPECSTREEPREP`](constants::SPECSTREEPREP) tree of the hot
//! database.
//!
//! ### Example
//!
//! `$ cargo run show -specs westend-sr25519`
//!
//! ## Check external file with hex-encoded metadata
//!
//! `$ cargo run check_file <path>`
//!
//! Asserts that:
//!
//! - the file contains valid metadata, with retrievable network name and
//! version
//! - if the metadata for same network name and version is in the hot
//! database, it completely matches the one from the file
//!
//! ### Example
//!
//! `$ cargo run check_file "../defaults/release_metadata/kusama9230"`
//!
//! ## Show metadata fetch block history from `META_HISTORY` tree of the hot database
//!
//! `$ cargo run show -block_history`
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
//! `$ cargo run add_specs <keys> <argument(s)> <overrides>`
//!
//! A file is generated in dedicated [`FOLDER`](constants::FOLDER) to
//! (optionally) be signed and later be transformed into `add_specs` update
//! QR. Output file name is `sign_me_add_specs_<name>_<encryption>`.
//!
//! Setting keys that could be used in command line (maximum one):
//!
//! - `-d`: do **not** update the database, make rpc calls, and produce
//! output files
//! - `-f`: do **not** run rpc calls, produce output files using data already in
//! the database
//! - `-p`: update or check database through rpc calls, do **not** produce any
//! output files
//! - `-t` (no setting key defaults here): update or check database through rpc
//! calls, produce output files
//!
//! <table>
//!     <tr>
//!         <th>setting key</th>
//!         <th>hot database update</th>
//!         <th>rpc calls</th>
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
//! - `-a`: all networks with entries in the
//! [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree of the hot database
//! - `-n` followed by single network address book title: for a network with
//! existing record in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
//! - `-u` followed by single url address: reserved for networks with no
//! record yet in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
//!
//! `-a` key could be used with `-s` key, to stop processing after first
//! error.
//!
//! Override key specifying encryption algorithm supported by the network is
//! optional for `-n` reference key (since there is already an entry in the
//! database with specified encryption) and mandatory for `-u` reference key.
//! Supported variants are:
//!
//! - `-ed25519`
//! - `-sr25519`
//! - `-ecdsa`
//!
//! Sequence invoking token override could be used when processing an
//! individual network that has multiple allowed decimals and unit values
//! retrieved as arrays of equal size. To override token, key `-token` followed
//! by `u8` decimals value and `String` unit value is used. By default, if no
//! token override in provided, such networks have `0u8` decimals and `UNIT`
//! unit set up.
//!
//! Title override could be used when processing an individual network, to set
//! the title under which the network will be displayed in Signer, should the
//! `add_specs` payload be accepted. Non-default networks, if the title override
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
//!         <td>- make rpc calls<br>
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
//!             - make rpc calls, check that the entry remains correct<br>
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
//!         <td>- make rpc calls<br>
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
//!             - make rpc calls, check that the entry remains correct<br>
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
//!         <td>- make rpc calls<br>
//!             - apply overrides<br>
//!             - update database if needed<br>
//!             - make payload file<br>
//!             Note: reserved for networks with no entries in the database
//!         </td>
//!     </tr>
//! </table>
//!
//! ## Prepare `load_metadata` update payload
//!
//! `$ cargo run load_metadata <key(s)> <(argument)>`
//!
//! A file is generated in dedicated [`FOLDER`](constants::FOLDER) to
//! (optionally) be signed and later be transformed into `load_metadata`
//! update QR. Output file name is `sign_me_load_metadata_<name>V<version>`.
//!
//! Setting keys that could be used in command line (maximum one):
//!
//! - `-d`: do **not** update the database, make rpc calls, and produce
//! output files
//! - `-f`: do **not** run rpc calls, produce output files from database as
//! it is
//! - `-k`: update database through rpc calls, produce output files only for
//! **new** database entries
//! - `-p`: update database through rpc calls, do **not** produce any output
//! files
//! - `-t` (no setting key defaults here): update database through rpc
//! calls, produce output files
//!
//! <table>
//!     <tr>
//!         <th>setting key</th>
//!         <th>hot database update</th>
//!         <th>rpc calls</th>
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
//! Network metadata updates quite often, compared to `add_specs` command there
//! is also setting key `-k` to print only the data that was not in the hot
//! database before the fetch.
//!
//! Reference keys (exactly only one has to be used):
//!
//! - `-a`: all networks with entries in the
//! [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree of the hot database
//! - `-n` followed by single network name: for a network with existing
//! record in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
//! - `-u` followed by single url address: reserved for networks with no
//! record yet in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
//!
//! `-a` key could be used with `-s` key, to stop processing after first
//! error.
//!
//! `load_metadata` has no overrides available. Not all setting and reference
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
//!         <td>- get all url addresses <b>from the database</b><br>
//!             - make rpc calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - make payload file(s)<br>
//!             Note: database is needed to get url addresses
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-d</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get url address <b>from the database</b> for the <code>network_name</code><br>
//!             - make rpc calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - make payload file<br>
//!             Note: database is needed to get url address
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-d</code></td>
//!         <td><code>-u</code></td>
//!         <td><code>url_address</code></td>
//!         <td>- make rpc calls<br>
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
//!         <td>- get all url addresses from the database<br>
//!             - make rpc calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed<br>
//!             - make payload file for each new entry
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-k</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get url address from the database for the <code>network_name</code><br>
//!             - make rpc calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed<br>
//!             - make payload file if the entry is new
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-p</code></td>
//!         <td><code>-a</code></td>
//!         <td></td>
//!         <td>- get all url addresses from the database<br>
//!             - make rpc calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-p</code></td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get url address from the database for the <code>network_name</code><br>
//!             - make rpc calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-t</code> or none declared</td>
//!         <td><code>-a</code></td>
//!         <td></td>
//!         <td>- get all url addresses from the database<br>
//!             - make rpc calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed<br>
//!             - make payload file(s)
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>-t</code> or none declared</td>
//!         <td><code>-n</code></td>
//!         <td><code>network_name</code></td>
//!         <td>- get url address from the database for the <code>network_name</code><br>
//!             - make rpc calls<br>
//!             - verify name, genesis hash, base58 prefix<br>
//!             - update the database if needed<br>
//!             - make payload file
//!         </td>
//!     </tr>
//! </table>
#![deny(unused_crate_dependencies)]

use constants::{COLD_DB_NAME_RELEASE, HOT_DB_NAME, TYLO};
use db_handling::{
    default_cold_release, default_hot,
    helpers::{prep_types, transfer_metadata_to_cold},
};
use definitions::error_active::{Active, ErrorActive};

mod derivations;
use derivations::process_derivations;
pub mod fetch_metadata;
mod helpers;
use helpers::debug_meta_at_block;
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

/// Function to process incoming command as interpreted by parser

pub fn full_run(command: Command) -> Result<(), ErrorActive> {
    match command {
        Command::Show(x) => match x {
            Show::Metadata => show_metadata(),
            Show::Networks => show_networks(),
            Show::Specs(title) => show_specs(title),
            Show::CheckFile(path) => check_file(path),
            Show::BlockHistory => show_block_history(),
        },
        Command::Specs(instruction) => gen_add_specs(instruction),
        Command::Load(instruction) => gen_load_meta(instruction),
        Command::Types => prep_types::<Active>(HOT_DB_NAME)?.write(TYLO),
        Command::Make(make) => make_message(make),
        Command::Remove(info) => remove_info(info),
        Command::RestoreDefaults => default_hot(),
        Command::MakeColdRelease(opt_path) => default_cold_release(opt_path),
        Command::TransferMetaRelease(opt_path) => {
            let cold_database_path = match opt_path {
                Some(ref path) => path.to_str().unwrap_or(COLD_DB_NAME_RELEASE),
                None => COLD_DB_NAME_RELEASE,
            };
            transfer_metadata_to_cold(HOT_DB_NAME, cold_database_path)
        }
        Command::Derivations(x) => process_derivations(x),
        Command::Unwasm {
            filename,
            update_db,
        } => unwasm(&filename, update_db),
        Command::MetaDefaultFile { name, version } => meta_default_file(&name, version),
        Command::MetaAtBlock { url, block_hash } => debug_meta_at_block(&url, &block_hash),
    }
}
