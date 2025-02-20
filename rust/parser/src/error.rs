use definitions::error::MetadataError;

/// Parse error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Metadata(#[from] MetadataError),

    #[error(transparent)]
    DefinitionsError(#[from] definitions::error::Error),

    #[error(transparent)]
    Codec(#[from] parity_scale_codec::Error),

    #[error("Can not separate method from extensions, bad transaction.")]
    SeparateMethodExtensions,

    /// Errors occurring during the decoding procedure.
    #[error("Error parsing incoming transaction content. {0}")]
    Decoding(#[from] ParserDecodingError),

    /// Errors occurring because the metadata
    /// [`RuntimeMetadataV14`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/v14/struct.RuntimeMetadataV14.html)
    /// has extensions not acceptable in existing safety paradigm for
    /// signable transactions.
    #[error(transparent)]
    FundamentallyBadV14Metadata(#[from] ParserMetadataError),

    /// While parsing transaction with certain version of network metadata,
    /// found that the version found in signable extensions does not match
    /// the version of the metadata used for parsing.
    ///
    /// Transaction parsing in Vault is done by consecutively checking all
    /// available metadata for a given network name, starting with the highest
    /// available version, and looking for a matching network version in the
    /// parsed extensions.
    ///
    /// For `RuntimeMetadataV12` and `RuntimeMetadataV13` the extensions set
    /// is a fixed one, whereas for `RuntimeMetadataV14` is may vary and is
    /// determined by the metadata itself.
    #[error(
        "Network spec version decoded from extensions ({as_decoded}) \
        differs from the version in metadata ({in_metadata})."
    )]
    WrongNetworkVersion {
        /// metadata version from transaction extensions, as found through
        /// parsing process
        as_decoded: String,

        /// metadata version actually used for parsing, from the `Version`
        /// constant in `System` pallet of the metadata
        in_metadata: u32,
    },

    //
    #[error(
        "Network name mismatch. In metadata: {name_metadata}, \
        in network specs: {name_network_specs}"
    )]
    NetworkNameMismatch {
        name_metadata: String,
        name_network_specs: String,
    },

    #[error(
        "Decoding transactions with metadata V12 and V13 uses pre-existing \
        types info. Loaded default types info is empty."
    )]
    NoTypes,

    #[error(
        "Decoding transactions with metadata V12 and V13 uses \
        pre-existing types info. Error generating default types info."
    )]
    DefaultTypes,
}

/// Parse result.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum ParserMetadataError {
    #[error("metadata extensions have no era")]
    NoEra,

    #[error("metadata extensions have no block hash")]
    NoBlockHash,

    #[error("metadata extensions have no network metadata version")]
    NoVersionExt,

    #[error("metadata extensions have more than one era")]
    EraTwice,

    #[error("metadata extensions have more than one genesis hash")]
    GenesisHashTwice,

    #[error("metadata extensions have more than one block hash")]
    BlockHashTwice,

    #[error("metadata extensions have more than one network metadata version")]
    SpecVersionTwice,
}

/// Errors directly related to transaction parsing
///
/// Signable transactions are differentiated based on prelude:
///
/// - `53xx00` mortal transactions
/// - `53xx02` immortal transactions
/// - `53xx03` text message transactions
///
/// `53xx00` and `53xx02` transactions contain encoded transaction data, and
/// are parsed prior to signing using the network metadata. Transaction is
/// generated in client, for certain address and within certain network.
/// To parse the transaction and to generate the signature, Vault must
/// have the network information (network specs and correct network metadata)
/// and the public address-associated information in its database.
///
/// `53xx00` and `53xx02` transcations consist of:
///
/// - prelude, `53xx00` or `53xx02`, where `xx` stands for the encryption
///   algorithm associated with address and network used
/// - public key corresponding to the address that can sign the transaction
/// - encoded call data, the body of the transaction
/// - extensions, as set in the network metadata
/// - genesis hash of the network in which the transaction was generated
///
/// Parsing process first separates the prelude, public key, genesis hash and
/// the combined call + extensions data.
///
/// The call information is SCALE-encoded into `Vec<u8>` bytes and then those
/// bytes are SCALE-encoded again, so that the call data contained in the
/// transaction consists of `compact` with encoded call length in bytes
/// followed by the `Vec<u8>` with the encoded data.
///
/// Call and extensions are cut based on the call length declared at the start
/// of the combined call + extensions data.
///
/// Then the extensions are decoded, and it is checked that the metadata version
/// in extensions coincides with the metadata version used for the decoding.
///
/// Decoding the extensions for metadata with `RuntimeMetadataV12` or
/// `RuntimeMetadataV13` is using a static set of extensions, namely:
///
/// - [`Era`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/generic/enum.Era.html)
/// - nonce, compact `u64`
/// - transaction tip, compact `u128`
/// - metadata version, `u32`
/// - tx version, `u32`
/// - network genesis hash, `[u8; 32]`
/// - block hash, `[u8; 32]`
///
/// Decoding the extensions for metadata with `RuntimeMetadataV14` uses
/// dynamically acquired set of extensions from the metadata itself.
///
/// After the extensions, the call data itself is decoded using the network
/// metadata. Each call first byte is the index of the pallet.
///
/// Metadata with `RuntimeMetadataV12` or `RuntimeMetadataV13` has only type
/// names associated with call arguments. Vault finds what the types really
/// are and how to decode them by using the types information that must be in
/// Vault database.
/// For `RuntimeMetadataV12` or `RuntimeMetadataV13` the second byte in call is
/// the index of the method within the pallet, and thes Vault finds the types
/// used by the method and proceeds to decode the call data piece by piece.
///
/// Metadata with `RuntimeMetadataV14` has types data in-built in the metadata
/// itself, and the types needed to decode the call are resolved during the
/// decoding. For `RuntimeMetadataV14` the second byte in call is also
/// the index of the method within the pallet, but this already goes into the
/// type resolver.
///
/// Calls may contain nested calls, for `RuntimeMetadataV12` or
/// `RuntimeMetadataV13` metadata the call decoding always starts with pallet
/// and method combination processing. For `RuntimeMetadataV14` metadata the
/// nested calls are processed through the type resolver, i.e. the pallet index
/// is processed independently only on the start of the decoding.
///
/// `53xx03` transaction consists of:
///
/// - prelude `53xx03`, where `xx` stands for the encryption algorithm
///   associated with address and network used
/// - public key corresponding to the address that can sign the transaction
/// - SCALE-encoded `String` contents of the message
/// - genesis hash of the network in which the transaction was generated
///
/// Vault assumes that every byte of the transaction will be processed, and
/// shows an error if this is not the case.
#[derive(Debug, thiserror::Error)]
pub enum ParserDecodingError {
    /// Transaction was announced by the prelude to be mortal (`53xx00`),
    /// but has `Era::Immortal` in extensions
    #[error("Expected mortal transaction due to prelude format. Found immortal transaction.")]
    UnexpectedImmortality,

    /// Transaction was announced by the prelude to be immortal (`53xx02`),
    /// but has `Era::Mortal(_, _)` in extensions
    #[error("Expected immortal transaction due to prelude format. Found mortal transaction.")]
    UnexpectedMortality,

    /// Genesis hash cut from the end of the transaction does not match the one
    /// found in the extensions
    #[error(
        "Genesis hash values from decoded extensions and from used network specs do not match."
    )]
    GenesisHashMismatch,

    /// In immortal transaction the block hash from the extensions is the
    /// network genesis hash.
    ///
    /// This error happens when block hash is different with the genesis hash
    /// cut from the end of the transaction.
    #[error("Block hash for immortal transaction not matching genesis hash for the network.")]
    ImmortalHashMismatch,

    /// Error decoding the extensions using metadata with `RuntimeMetadataV12`
    /// or `RuntimeMetadataV13`, with default extensions set.
    #[error("Unable to decode extensions for V12/V13 metadata using standard extensions set.")]
    ExtensionsOlder,

    /// Used only for `RuntimeMetadataV12` or `RuntimeMetadataV13`,
    /// indicates that method index (second byte of the call data) is not valid
    /// for the pallet with found name.
    #[error("Method number {method_index} not found in pallet {pallet_name}.")]
    MethodNotFound {
        /// index of the method, second byte of the call data
        method_index: u8,

        /// name of the pallet, found from the first byte of the call data
        pallet_name: String,
    },

    /// Used only for all calls in `RuntimeMetadataV12` or `RuntimeMetadataV13`,
    /// and for entry call in `RuntimeMetadataV14` metadata. First byte of the
    /// call data is not a valid pallet index.
    ///
    /// Associated data is what was thought to be a pallet index.
    #[error("Pallet with index {0} not found.")]
    PalletNotFound(u8),

    /// Only for entry call in `RuntimeMetadataV14`. Pallet found via first byte
    /// of the call has no associated calls.
    ///
    /// Associated data is the pallet name.
    #[error("No calls found in pallet {0}.")]
    NoCallsInPallet(String),

    /// Only for `RuntimeMetadataV14`. Found type index could not be resolved
    /// in types registry
    #[error("Referenced type could not be resolved in v14 metadata.")]
    V14TypeNotResolved,

    /// Only for `RuntimeMetadataV12` and `RuntimeMetadataV13`. Argument type
    /// could not be taken out of `DecodeDifferent` construction.
    #[error("Argument type error.")]
    ArgumentTypeError,

    /// Only for `RuntimeMetadataV12` and `RuntimeMetadataV13`. Argument name
    /// could not be taken out of `DecodeDifferent` construction.
    #[error("Argument name error.")]
    ArgumentNameError,

    /// Parser was trying to find an encoded
    /// [`compact`](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/struct.Compact.html),
    /// in the bytes sequence, but was unable to.
    #[error("Expected compact. Not found it.")]
    NoCompact,

    /// Parser was expecting more data.
    #[error("Data too short for expected content.")]
    DataTooShort,

    /// Parser was unable to decode the data piece into a primitive type.
    ///
    /// Associated data is primitive identifier.
    #[error("Unable to decode part of data as {0}.")]
    PrimitiveFailure(String),

    /// SCALE-encoded `Option<_>` can have as a first byte:
    ///
    /// - `0` if the value is `None`
    /// - `1` if the value is `Some`
    /// - `2` if the value is `Some(false)` for `Option<bool>` encoding
    ///
    /// This error appears if the parser encounters something unexpected in the
    /// first byte of encoded `Option<_>` instead.
    #[error("Encountered unexpected Option<_> variant.")]
    UnexpectedOptionVariant,

    /// Only for `RuntimeMetadataV12` and `RuntimeMetadataV13`.
    /// Decoding
    /// [`IdentityFields`](https://docs.substrate.io/rustdocs/latest/pallet_identity/struct.IdentityFields.html)
    /// requires having correct type information for
    /// [`IdentityField`](https://docs.substrate.io/rustdocs/latest/pallet_identity/enum.IdentityField.html)
    /// in types information. If types information has no entry for
    /// `IdentityFields` or it is not an enum, this error appears.
    #[error("IdentityField description error.")]
    IdFields,

    /// Parser processes certain types as balance (i.e. transforms the data
    /// into appropriate float using decimals and units provided).
    /// For some types the balance representation is not possible, this error
    /// occurs if the parser tried to process as a balance some type not
    /// suitable for it.
    #[error("Unexpected type encountered for Balance.")]
    BalanceNotDescribed,

    /// SCALE-encoded enum can have as a first byte only correct index of the
    /// variant used.
    ///
    /// This error appears if the first byte is an invalid variant index.
    #[error("Encountered unexpected enum variant.")]
    UnexpectedEnumVariant,

    /// Parser found that type declared as a
    /// [`compact`](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/struct.Compact.html)
    /// has inner type that could not be encoded as a `compact`
    #[error("Unexpected type inside comact.")]
    UnexpectedCompactInsides,

    /// Only for `RuntimeMetadataV12` and `RuntimeMetadataV13`.
    /// Parser has encountered a type that could not be interpreted using the
    /// existing types information.
    ///
    /// Associated data is the type description as it was received by parser
    /// from the metadata.
    #[error("No description found for type {0}.")]
    UnknownType(String),

    /// Only for `RuntimeMetadataV14`.
    /// While decoding
    /// [`BitVec<T,O>`](https://docs.rs/bitvec/1.0.0/bitvec/vec/struct.BitVec.html),
    /// parser encountered `T` type not implementing
    /// [`BitStore`](https://docs.rs/bitvec/1.0.0/bitvec/store/trait.BitStore.html).
    #[error("Declared type is not suitable BitStore type for BitVec.")]
    NotBitStoreType,

    /// Only for `RuntimeMetadataV14`.
    /// While decoding
    /// [`BitVec<T,O>`](https://docs.rs/bitvec/1.0.0/bitvec/vec/struct.BitVec.html),
    /// parser encountered `O` type not implementing
    /// [`BitOrder`](https://docs.rs/bitvec/1.0.0/bitvec/order/trait.BitOrder.html).
    #[error("Declared type is not suitable BitOrder type for BitVec.")]
    NotBitOrderType,

    /// Only for `RuntimeMetadataV14`.
    /// Parser failed to decode
    /// [`BitVec<T,O>`](https://docs.rs/bitvec/1.0.0/bitvec/vec/struct.BitVec.html),
    /// even though `T` and `O` types were suitable.
    #[error("Could not decode BitVec.")]
    BitVecFailure,

    /// Only for `RuntimeMetadataV14`.
    /// Parser failed to decode data slice as
    /// [`Era`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/generic/enum.Era.html).
    #[error("Could not decode Era.")]
    Era,

    /// Parser expects to use all data in decoding. This error appears if some
    /// data was not used in parsing of the method.
    #[error("After decoding the method some data remained unused.")]
    SomeDataNotUsedMethod,

    /// Only for `RuntimeMetadataV14`.
    /// Parser expects to use all data in decoding. This error appears if some
    /// data from extensions is not used in the decoding.
    #[error("After decoding the extensions some data remained unused.")]
    SomeDataNotUsedExtensions,
}
