use sled::{Db, Tree, open, IVec};
use hex;
use constants::{ADDRTREE, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, TYPES, VERIFIERS};
use db_handling::{db_transactions::{TrDbColdSign, TrDbColdStub}, helpers::check_metadata};
use definitions::{history::Event, keyring::{AddressKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey, VerifierKey}, metadata::{MetaValues, VersionDecoded}, network_specs::{ChainSpecs, ChainSpecsToSend, CurrentVerifier, Verifier}, qr_transfers::ContentLoadTypes, types::TypeEntry, users::AddressDetails};
use parity_scale_codec::Decode;
use frame_metadata::RuntimeMetadata;
use meta_reading::decode_metadata::{get_meta_const_light};

use crate::{cards::Warning, error::{Error, BadInputData, DatabaseError, SystemError}};

/// Wrapper for `open` with crate error (card)
fn open_db (database_name: &str) -> Result<Db, Error> {
    match open(database_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Wrapper for `open_tree` with crate error (card)
fn open_tree (database: &Db, tree_name: &[u8]) -> Result<Tree, Error> {
    match database.open_tree(tree_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Wrapper for `get` with crate error (card)
fn get_from_tree(key: &Vec<u8>, tree: &Tree) -> Result<Option<IVec>, Error> {
    match tree.get(key) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Function to get the network specs from the database or
/// return None if no specs are on record, with crate error (card)
pub (crate) fn checked_network_specs (network_specs_key: &NetworkSpecsKey, database_name: &str) -> Result<Option<ChainSpecs>, Error> {
    let database = open_db(&database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    match get_from_tree(&network_specs_key.key(), &chainspecs)? {
        Some(encoded_network_specs) => {
            match <ChainSpecs>::decode(&mut &encoded_network_specs[..]) {
                Ok(a) => Ok(Some(a)),
                Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedChainSpecs)),
            }
        },
        None => Ok(None),
    }
}

/// Function to get the address details from the database or
/// return None if no details are on record, with crate error (card)
pub (crate) fn checked_address_details (address_key: &AddressKey, database_name: &str) -> Result<Option<AddressDetails>, Error> {
    let database = open_db(&database_name)?;
    let identities = open_tree(&database, ADDRTREE)?;
    match get_from_tree(&address_key.key(), &identities)? {
        Some(encoded_address_details) => {
            match <AddressDetails>::decode(&mut &encoded_address_details[..]) {
                Ok(a) => Ok(Some(a)),
                Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedAddressDetails)),
            }
        },
        None => Ok(None),
    }
}

/// Function to decode hex string (possibly with `0x` start) into Vec<u8>, with crate error (card)
pub fn unhex(hex_entry: &str) -> Result<Vec<u8>, Error> {
    let hex_entry = {
        if hex_entry.starts_with("0x") {&hex_entry[2..]}
        else {hex_entry}
    };
    match hex::decode(hex_entry) {
        Ok(x) => Ok(x),
        Err(_) => return Err(Error::BadInputData(BadInputData::NotHex)),
    }
}

/// Function to try and get verifier for network using given VerifierKey, with crate error (card)
pub fn get_current_verifier (verifier_key: &VerifierKey, database_name: &str) -> Result<Option<CurrentVerifier>, Error> {
    let database = open_db(&database_name)?;
    let verifiers = open_tree(&database, VERIFIERS)?;
    match verifiers.get(verifier_key.key()) {
        Ok(Some(verifier_encoded)) => match <CurrentVerifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(Some(a)),
            Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedNetworkVerifier)),
        },
        Ok(None) => {
            if genesis_hash_in_specs(verifier_key, &database)? {return Err(Error::DatabaseError(DatabaseError::UnexpectedlyMetGenesisHash(verifier_key.key())))}
            Ok(None)
        },
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Function to get general verifier, with crate error (card)
pub (crate) fn get_general_verifier (database_name: &str) -> Result<Verifier, Error> {
    let database = open_db(&database_name)?;
    let settings = open_tree(&database, SETTREE)?;
    match settings.get(GENERALVERIFIER.to_vec()) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedGeneralVerifier)),
        },
        Ok(None) => return Err(Error::DatabaseError(DatabaseError::NoGeneralVerifier)),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Function to search for genesis hash corresponding to a given verifier key
/// in SPECSTREE of the database, with crate error (card)
fn genesis_hash_in_specs (verifier_key: &VerifierKey, database: &Db) -> Result<bool, Error> {
    let genesis_hash = verifier_key.genesis_hash();
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let mut out = false;
    for x in chainspecs.iter() {
        if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
            match <ChainSpecs>::decode(&mut &network_specs_encoded[..]) {
                Ok(y) => {
                    let network_specs_key = NetworkSpecsKey::from_vec(&network_specs_key_vec.to_vec());
                    if network_specs_key != NetworkSpecsKey::from_parts(&y.genesis_hash.to_vec(), &y.encryption) {
                        return Err(Error::DatabaseError(DatabaseError::NetworkSpecsKeyMismatch(network_specs_key)))
                    }
                    if y.genesis_hash.to_vec() == genesis_hash {
                        out = true;
                        break;
                    }
                },
                Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedChainSpecs)),
            }
        }
    }
    Ok(out)
}


/// Function searches for full metadata for certain chain name and version in metadata database tree.
/// Checks that found full metadata indeed corresponds to the queried name and version;
/// in case of successful find produces a tuple of corresponding RuntimeMetadata and Option<u32>;
/// Option is None if the version of chain is the latest known one,
/// and Some(latest_version) if there are later versions available.
pub fn find_meta(network_name: &str, network_version: u32, database_name: &str) -> Result<(RuntimeMetadata, Option<u32>), Error> {
    
    let database = open_db(&database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    
    let mut meta = None;
    let mut other = false;
    let mut latest_version = network_version;
    
    let meta_key_prefix = MetaKeyPrefix::from_name(&network_name);
    for x in metadata.scan_prefix(meta_key_prefix.prefix()) {
        let (meta_key_vec, meta_found) = match x {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
        };
        let (_, version) = match MetaKey::from_vec(&meta_key_vec.to_vec()).name_version() {
            Ok(t) => t,
            Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedVersName)),
        };
        if version == network_version {meta = Some(meta_found)}
        else {
            other = true;
            if version > latest_version {latest_version = version}
        }
    }
    
    match meta {
        Some(m) => {
            if !m.starts_with(&vec![109, 101, 116, 97]) {return Err(Error::SystemError(SystemError::NotMeta))}
            if m[4] < 12 {
                return Err(Error::SystemError(SystemError::MetaVersionBelow12));
            }
            let data_back = RuntimeMetadata::decode(&mut &m[4..]);
            match data_back {
                Ok(metadata) => {
                // check if the name and version are same in metadata, i.e. the database is not damaged
                    match get_meta_const_light(&metadata) {
                        Ok(x) => {
                            match VersionDecoded::decode(&mut &x[..]) {
                                Ok(y) => {
                                    if (y.spec_version != network_version) || (y.specname != network_name) {return Err(Error::SystemError(SystemError::MetaMismatch))}
                                },
                                Err(_) => return Err(Error::SystemError(SystemError::VersionNotDecodeable))
                            }
                        },
                        Err(_) => return Err(Error::SystemError(SystemError::NoVersion))
                    };
                    if network_version < latest_version {
                        Ok((metadata, Some(latest_version)))
                    }
                    else {Ok((metadata, None))}
                },
                Err(_) => return Err(Error::SystemError(SystemError::UnableToDecodeMeta)),
            }
        },
        None => {
            if other {return Err(Error::DatabaseError(DatabaseError::NoMetaThisVersion))}
            else {return Err(Error::DatabaseError(DatabaseError::NoMetaAtAll))}
        },
    }
}

pub fn decode_input_metadata (meta: Vec<u8>) -> Result<MetaValues, Error> {
    if !meta.starts_with(&vec![109, 101, 116, 97]) {return Err(Error::BadInputData(BadInputData::NotMeta))}
    if meta[4] < 12 {return Err(Error::BadInputData(BadInputData::MetaVersionBelow12))}
    match RuntimeMetadata::decode(&mut &meta[4..]) {
        Ok(received_metadata) => {
            match get_meta_const_light(&received_metadata) {
                Ok(x) => {
                    match VersionDecoded::decode(&mut &x[..]) {
                        Ok(y) => {
                            Ok(MetaValues{
                                name: y.specname.to_string(),
                                version: y.spec_version,
                                meta,
                            })
                        },
                        Err(_) => return Err(Error::BadInputData(BadInputData::VersionNotDecodeable)),
                    }
                },
                Err(_) => return Err(Error::BadInputData(BadInputData::NoMetaVersion)),
            }
        },
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeMeta)),
    }
}

pub fn accept_meta_values (meta_values: &MetaValues, database_name: &str) -> Result<bool, Error> {
    let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
    let database = open_db(&database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    match metadata.get(meta_key.key()) {
        Ok(Some(a)) => {
            if a == meta_values.meta {Ok(false)}
            else {return Err(Error::BadInputData(BadInputData::MetaTotalMismatch))}
        },
        Ok(None) => Ok(true),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}
/*
/// Function to search for network_specs_key in chainspecs database tree
pub fn get_chainspecs (network_specs_key: &NetworkSpecsKey, chainspecs: &Tree) -> Result<ChainSpecs, Error> {
    match get_from_tree(&network_specs_key.key(), chainspecs)? {
        Some(x) => match <ChainSpecs>::decode(&mut &x[..]) {
            Ok(y) => Ok(y),
            Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedChainSpecs)),
        },
        None => return Err(Error::DatabaseError(DatabaseError::NoNetwork)),
    }
}
*/
/// Function to check if the chaispecs are already in the database
pub fn specs_are_new (network_specs: &ChainSpecsToSend, database_name: &str) -> Result<bool, Error> {
    let network_specs_key = NetworkSpecsKey::from_parts(&network_specs.genesis_hash.to_vec(), &network_specs.encryption);
    let database = open_db(&database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(encoded_known_network_specs)) => {
            match <ChainSpecs>::decode(&mut &encoded_known_network_specs[..]) {
                Ok(a) => {
                    if (a.base58prefix != network_specs.base58prefix)|(a.decimals != network_specs.decimals)|(a.encryption != network_specs.encryption)|(a.name != network_specs.name)|(a.unit != network_specs.unit) {return Err(Error::BadInputData(BadInputData::ImportantSpecsChanged))}
                    let is_known = (a.color == network_specs.color) && (a.logo == network_specs.logo) && (a.path_id == network_specs.path_id) && (a.secondary_color == network_specs.secondary_color) && (a.title == network_specs.title);
                    Ok(!is_known)
                },
                Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedChainSpecs)),
            }
        },
        Ok(None) => Ok(true),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// function to search database for the TypeEntry vector
pub fn get_types (database_name: &str) -> Result<Vec<TypeEntry>, Error> {
    let database = open_db(&database_name)?;
    let settings = open_tree(&database, SETTREE)?;
    match get_from_tree(&TYPES.to_vec(), &settings)? {
        Some(a) => {
            match <Vec<TypeEntry>>::decode(&mut &a[..]) {
                Ok(x) => {
                    if x.len()==0 {return Err(Error::DatabaseError(DatabaseError::NoTypes))}
                    Ok(x)
                },
                Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedTypesDatabase)),
            }
        },
        None => return Err(Error::DatabaseError(DatabaseError::NoTypes)),
    }
}

/// function to add network specs to stub with crate error
pub fn stub_add_network_specs (stub: TrDbColdStub, specs: &ChainSpecsToSend, current_verifier: &CurrentVerifier, general_verifier: &Verifier, database_name: &str) -> Result<TrDbColdStub, Error> {
    match stub.add_network_specs(specs, current_verifier, general_verifier, &database_name) {
        Ok(a) => Ok(a),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Temporary(e.to_string()))),
    }
}

/// function to put stub to storage and get checksum, with crate error
pub fn stub_store_and_get_checksum (stub: TrDbColdStub, database_name: &str) -> Result<u32, Error> {
    match stub.store_and_get_checksum(&database_name) {
        Ok(a) => Ok(a),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Temporary(e.to_string()))),
    }
}

/// function to put sign to storage and get checksum, with crate error
pub fn sign_store_and_get_checksum (sign: TrDbColdSign, database_name: &str) -> Result<u32, Error> {
    match sign.store_and_get_checksum(&database_name) {
        Ok(a) => Ok(a),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Temporary(e.to_string()))),
    }
}

fn print_affected (metadata_set: &Vec<MetaValues>, network_specs_set: &Vec<ChainSpecs>) -> String {
    let mut out_metadata = String::new();
    let mut out_network_specs = String::new();
    for (i, x) in metadata_set.iter().enumerate() {
        if i>0 {out_metadata.push_str(", ");}
        out_metadata.push_str(&format!("{}{}", x.name, x.version));
    }
    for (i, x) in network_specs_set.iter().enumerate() {
        if i>0 {out_network_specs.push_str(", ");}
        out_network_specs.push_str(&x.title);
    }
    if out_network_specs.len()==0 {out_network_specs = String::from("none");}
    if out_metadata.len()==0 {out_metadata = String::from("none");}
    format!("Affected network specs entries: {}; affected metadata entries: {}.", out_network_specs, out_metadata)
}

fn collect_set (verifier_key: &VerifierKey, chainspecs: &Tree, metadata: &Tree) -> Result<(Vec<MetaValues>, Vec<ChainSpecs>), Error> {
    let mut metadata_set: Vec<MetaValues> = Vec::new();
    let mut network_specs_set: Vec<ChainSpecs> = Vec::new();
    let genesis_hash = verifier_key.genesis_hash();
    let mut name_found = None;
    for x in chainspecs.iter() {
        if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
            let network_specs_key = NetworkSpecsKey::from_vec(&network_specs_key_vec.to_vec());
            let network_specs = match <ChainSpecs>::decode(&mut &network_specs_encoded[..]) {
                Ok(y) => {
                    if network_specs_key != NetworkSpecsKey::from_parts(&y.genesis_hash.to_vec(), &y.encryption) {
                        return Err(Error::DatabaseError(DatabaseError::NetworkSpecsKeyMismatch(network_specs_key)))
                    }
                    y
                },
                Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedChainSpecs)),
            };
            if network_specs.genesis_hash.to_vec() == genesis_hash {
                name_found = match name_found {
                    Some(n) => {
                        if n != network_specs.name {return Err(Error::DatabaseError(DatabaseError::DifferentNamesSameGenesisHash(genesis_hash)))}
                        Some(n)
                    },
                    None => Some(network_specs.name.to_string()),
                 };
                network_specs_set.push(network_specs);
            }
        }
    }
    if let Some(name) = name_found {
        let meta_key_prefix = MetaKeyPrefix::from_name(&name);
        for y in metadata.scan_prefix(meta_key_prefix.prefix()) {
            if let Ok((meta_key_vec, meta_stored)) = y {
                let meta_key = MetaKey::from_vec(&meta_key_vec.to_vec());
                let (name, version) = match meta_key.name_version() {
                    Ok(a) => a,
                    Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedVersName)),
                };
                let meta = match check_metadata(meta_stored.to_vec(), &name, version) {
                    Ok(a) => a,
                    Err(_) => return Err(Error::SystemError(SystemError::MetaMismatch)),
                };
                metadata_set.push(MetaValues{name, version, meta});
            }
        }
    }
    Ok((metadata_set, network_specs_set))
}

pub (crate) struct GeneralHold {
    pub (crate) metadata_set: Vec<MetaValues>,
    pub (crate) network_specs_set: Vec<ChainSpecs>,
    pub (crate) types: bool,
}

impl GeneralHold {
    /// function to show entries depending on general verifier
    pub (crate) fn show(&self) -> String {
        let part = print_affected(&self.metadata_set, &self.network_specs_set);
        if self.types {format!("{} Types information is purged.", part)}
        else {part}
    }
    /// function to find all entries in the database that were verified by general verifier
    pub (crate) fn get(database_name: &str) -> Result<Self, Error> {
        let mut metadata_set: Vec<MetaValues> = Vec::new();
        let mut network_specs_set: Vec<ChainSpecs> = Vec::new(); // all are verified by general_verifier
        let mut verifier_set: Vec<VerifierKey> = Vec::new();
    
        let database = open_db(&database_name)?;
        let metadata = open_tree(&database, METATREE)?;
        let chainspecs = open_tree(&database, SPECSTREE)?;
        let settings = open_tree(&database, SETTREE)?;
        let verifiers = open_tree(&database, VERIFIERS)?;
        for x in verifiers.iter() {
            if let Ok((verifier_key_vec, current_verifier_encoded)) = x {
                let verifier_key = VerifierKey::from_vec(&verifier_key_vec.to_vec());
                let current_verifier = match <CurrentVerifier>::decode(&mut &current_verifier_encoded[..]) {
                    Ok(a) => a,
                    Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedNetworkVerifier)),
                };
                if let CurrentVerifier::General = current_verifier {verifier_set.push(verifier_key)}
            }
        }
        for verifier_key in verifier_set.iter() {
            let (new_metadata_set, new_network_specs_set) = collect_set(verifier_key, &chainspecs, &metadata)?;
            metadata_set.extend_from_slice(&new_metadata_set);
            network_specs_set.extend_from_slice(&new_network_specs_set);
        }
        let types = match settings.contains_key(TYPES) {
            Ok(a) => a,
            Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
        };
        Ok(Self{
            metadata_set,
            network_specs_set,
            types,
        })
    }
    pub (crate) fn upd_stub (&self, stub: TrDbColdStub, new_general_verifier: &Verifier, database_name: &str) -> Result<TrDbColdStub, Error> {
        let former_general_verifier = get_general_verifier(&database_name)?;
        let types_vec = get_types (&database_name)?;
        let types = ContentLoadTypes::generate(&types_vec);
        let mut out = stub;
        out = out.new_history_entry(Event::Warning(Warning::GeneralVerifierAppeared(&self).show()));
        for x in self.metadata_set.iter() {out = out.remove_metadata(x)}
        for x in self.network_specs_set.iter() {out = out.remove_network_specs(x, &CurrentVerifier::General, &former_general_verifier)}
        if self.types {out = out.remove_types(&types, &former_general_verifier)}
        out = out.new_general_verifier(new_general_verifier);
        Ok(out)
    }
}

pub (crate) struct Hold {
    pub (crate) metadata_set: Vec<MetaValues>,
    pub (crate) network_specs_set: Vec<ChainSpecs>,
}

impl Hold {
    /// function to show entries depending on former verifier
    pub (crate) fn show(&self) -> String {
        print_affected(&self.metadata_set, &self.network_specs_set)
    }
    /// function to find all entries in the database corresponding to given verifier_key, that was used to store the former verifier
    pub (crate) fn get(verifier_key: &VerifierKey, database_name: &str) -> Result<Self, Error> {
        let database = open_db(&database_name)?;
        let metadata = open_tree(&database, METATREE)?;
        let chainspecs = open_tree(&database, SPECSTREE)?;
        let (metadata_set, network_specs_set) = collect_set(verifier_key, &chainspecs, &metadata)?;
        Ok(Self{
            metadata_set,
            network_specs_set,
        })
    }
    pub (crate) fn upd_stub (&self, stub: TrDbColdStub, verifier_key: &VerifierKey, former_verifier: &Verifier, new_verifier: &CurrentVerifier, hold_release: HoldRelease, database_name: &str) -> Result<TrDbColdStub, Error> {
        let general_verifier = get_general_verifier(&database_name)?;
        let mut out = stub;
        let warning = match hold_release {
            HoldRelease::General => Warning::VerifierChangingToGeneral{verifier_key, hold: &self}.show(),
            HoldRelease::Custom => Warning::VerifierChangingToCustom{verifier_key, hold: &self}.show(),
        };
        out = out.new_history_entry(Event::Warning(warning));
        for x in self.metadata_set.iter() {out = out.remove_metadata(x)}
        for x in self.network_specs_set.iter() {out = out.remove_network_specs(x, &CurrentVerifier::Custom(former_verifier.to_owned()), &general_verifier)}
        out = out.new_network_verifier(verifier_key, new_verifier, &general_verifier);
        Ok(out)
    }
}

pub (crate) enum HoldRelease {
    General,
    Custom
}

#[cfg(test)]
mod tests {
    use db_handling::{cold_default::populate_cold_no_metadata};
    use super::*;
    use definitions::{keyring::VerifierKey, network_specs::{CurrentVerifier, Verifier}};
    use hex;
    use crate::cards::Card;
    
    #[test]
    fn find_westend_verifier() {
        let dbname = "for_tests/find_westend_verifier";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let verifier_key = VerifierKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap());
        let westend_verifier = match get_current_verifier(&verifier_key, &dbname) {
            Ok(a) => a,
            Err(e) => panic!("{}", Card::Error(e).card(&mut 0,0)),
        };
        assert!(westend_verifier == Some(CurrentVerifier::General));
        std::fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn not_find_mock_verifier() {
        let dbname = "for_tests/not_find_mock_verifier";
        populate_cold_no_metadata(dbname, Verifier::None).unwrap();
        let verifier_key = VerifierKey::from_parts(&hex::decode("62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b").unwrap());
        match get_current_verifier(&verifier_key, &dbname) {
            Ok(Some(_)) => panic!("Found network key that should not be in database."),
            Ok(None) => (),
            Err(e) => panic!("Error looking for mock verifier, {}", Card::Error(e).card(&mut 0,0)),
        }
        std::fs::remove_dir_all(dbname).unwrap();
    }
}
