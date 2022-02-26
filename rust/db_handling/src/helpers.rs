use parity_scale_codec::{Decode, Encode};
use sled::{Db, Tree, Batch, open};

use constants::{ADDRTREE, DANGER, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, TYPES, VERIFIERS};
use definitions::{danger::DangerRecord, error::{DatabaseSigner, EntryDecodingSigner, ErrorSigner, ErrorSource, NotFoundSigner, Signer}, keyring::{AddressKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey, VerifierKey}, metadata::MetaValues, network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier}, types::TypeEntry, users::{AddressDetails}};

/// Wrapper for `open`
pub fn open_db <T: ErrorSource> (database_name: &str) -> Result<Db, T::Error> {
    open(database_name).map_err(<T>::db_internal)
}

/// Wrapper for `open_tree`
pub fn open_tree <T: ErrorSource> (database: &Db, tree_name: &[u8]) -> Result<Tree, T::Error> {
    database.open_tree(tree_name).map_err(<T>::db_internal)
}

/// Wrapper to assemble a Batch for removing all elements of a tree
/// (to add into transaction where clear_tree should be)
pub fn make_batch_clear_tree <T: ErrorSource> (database_name: &str, tree_name: &[u8]) -> Result<Batch, T::Error> {
    let database = open_db::<T>(database_name)?;
    let tree = open_tree::<T>(&database, tree_name)?;
    let mut out = Batch::default();
    for (key, _) in tree.iter().flatten() {out.remove(key)}
    Ok(out)
}

/// Function to try and get from the Signer database the _valid_ current verifier for network using VerifierKey
pub fn try_get_valid_current_verifier (verifier_key: &VerifierKey, database_name: &str) -> Result<Option<ValidCurrentVerifier>, ErrorSigner> {
    let general_verifier = get_general_verifier(database_name)?;
    let database = open_db::<Signer>(database_name)?;
    let verifiers = open_tree::<Signer>(&database, VERIFIERS)?;
    match verifiers.get(verifier_key.key()) {
        Ok(Some(verifier_encoded)) => match <CurrentVerifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => {
                match a {
                    CurrentVerifier::Valid(b) => {
                        if let ValidCurrentVerifier::Custom(ref custom_verifier) = b {
                            if (custom_verifier == &general_verifier)&&(general_verifier != Verifier(None)) {return Err(ErrorSigner::Database(DatabaseSigner::CustomVerifierIsGeneral(verifier_key.to_owned())))}
                        }
                        Ok(Some(b))
                    },
                    CurrentVerifier::Dead => Err(ErrorSigner::DeadVerifier(verifier_key.to_owned())),
                }
            },
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::CurrentVerifier(verifier_key.to_owned())))),
        },
        Ok(None) => {
            if let Some((network_specs_key, _)) = genesis_hash_in_specs(verifier_key, &database)? {return Err(ErrorSigner::Database(DatabaseSigner::UnexpectedGenesisHash{verifier_key: verifier_key.to_owned(), network_specs_key}))}
            Ok(None)
        },
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to get from the Signer database the current verifier for network using VerifierKey, returns error if not found
pub fn get_valid_current_verifier (verifier_key: &VerifierKey, database_name: &str) -> Result<ValidCurrentVerifier, ErrorSigner> {
    try_get_valid_current_verifier(verifier_key, database_name)?
        .ok_or_else(|| ErrorSigner::NotFound(NotFoundSigner::CurrentVerifier(verifier_key.to_owned())))
}

/// Function to search for genesis hash corresponding to a given verifier key
/// in SPECSTREE of the Signer database
/// If there are more than one network corresponding to the same genesis hash,
/// outputs network specs key for the network with the lowest order
pub fn genesis_hash_in_specs (verifier_key: &VerifierKey, database: &Db) -> Result<Option<(NetworkSpecsKey, NetworkSpecs)>, ErrorSigner> {
    let genesis_hash = verifier_key.genesis_hash();
    let chainspecs = open_tree::<Signer>(database, SPECSTREE)?;
    let mut specs_set: Vec<(NetworkSpecsKey, NetworkSpecs)> = Vec::new();
    let mut found_base58prefix = None;
    for (network_specs_key_vec, network_specs_encoded) in chainspecs.iter().flatten() {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        let network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(&network_specs_key, network_specs_encoded)?;
        if network_specs.genesis_hash.to_vec() == genesis_hash {
            found_base58prefix = match found_base58prefix {
                Some(base58prefix) => {
                    if base58prefix == network_specs.base58prefix {Some(base58prefix)}
                    else {return Err(ErrorSigner::Database(DatabaseSigner::DifferentBase58Specs{genesis_hash: network_specs.genesis_hash, base58_1: base58prefix, base58_2: network_specs.base58prefix}))}
                },
                None => Some(network_specs.base58prefix),
            };
            specs_set.push((network_specs_key, network_specs))
        }
    }
    specs_set.sort_by(|a, b| a.1.order.cmp(&b.1.order));
    match specs_set.get(0) {
        Some(a) => Ok(Some(a.to_owned())),
        None => Ok(None),
    } 
}

/// Function to get general Verifier from the Signer database
/// Note that not finding general verifier is always an error.
pub fn get_general_verifier (database_name: &str) -> Result<Verifier, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let settings = open_tree::<Signer>(&database, SETTREE)?;
    match settings.get(GENERALVERIFIER.to_vec()) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::GeneralVerifier))),
        },
        Ok(None) => Err(ErrorSigner::NotFound(NotFoundSigner::GeneralVerifier)),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to display general Verifier from the Signer database
pub fn display_general_verifier (database_name: &str) -> Result<String, ErrorSigner> {
    Ok(get_general_verifier(database_name)?.show_card())
}

/// Function to try and get types information from the database
/// Applicable to both Active side and Signer side
pub fn try_get_types <T: ErrorSource> (database_name: &str) -> Result<Option<Vec<TypeEntry>>, T::Error> {
    let database = open_db::<T>(database_name)?;
    let settings = open_tree::<T>(&database, SETTREE)?;
    match settings.get(TYPES) {
        Ok(Some(types_info_encoded)) => {
            match <Vec<TypeEntry>>::decode(&mut &types_info_encoded[..]) {
                Ok(a) => Ok(Some(a)),
                Err(_) => Err(<T>::faulty_database_types()),
            }
        },
        Ok(None) => Ok(None),
        Err(e) => Err(<T>::db_internal(e)),
    }
}

/// Function to get types information from the database, returns error if not found
/// Applicable to both Active side and Signer side
pub fn get_types <T: ErrorSource> (database_name: &str) -> Result<Vec<TypeEntry>, T::Error> {
    try_get_types::<T>(database_name)?
        .ok_or_else(|| <T>::types_not_found())
}

/// Function to try and get network specs from the Signer database
pub fn try_get_network_specs (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<Option<NetworkSpecs>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(network_specs_encoded)) => Ok(Some(NetworkSpecs::from_entry_with_key_checked::<Signer>(network_specs_key, network_specs_encoded)?)),
        Ok(None) => Ok(None),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to get network specs from the Signer database, returns error if not found
pub fn get_network_specs (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<NetworkSpecs, ErrorSigner> {
    try_get_network_specs(database_name, network_specs_key)?
        .ok_or_else(|| ErrorSigner::NotFound(NotFoundSigner::NetworkSpecs(network_specs_key.to_owned())))
}

/// Function to try and get address details from the Signer database
pub fn try_get_address_details (database_name: &str, address_key: &AddressKey) -> Result<Option<AddressDetails>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let identities = open_tree::<Signer>(&database, ADDRTREE)?;
    match identities.get(address_key.key()) {
        Ok(Some(address_details_encoded)) => Ok(Some(AddressDetails::from_entry_with_key_checked::<Signer>(address_key, address_details_encoded)?)),
        Ok(None) => Ok(None),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to get address details from the Signer database, returns error if not found
pub fn get_address_details (database_name: &str, address_key: &AddressKey) -> Result<AddressDetails, ErrorSigner> {
    try_get_address_details(database_name, address_key)?
        .ok_or_else(|| ErrorSigner::NotFound(NotFoundSigner::AddressDetails(address_key.to_owned())))
}

/// Function to collect MetaValues corresponding to given network name.
/// Applicable to both Active side and Signer side
pub fn get_meta_values_by_name <T: ErrorSource> (database_name: &str, network_name: &str) -> Result<Vec<MetaValues>, T::Error> {
    let database = open_db::<T>(database_name)?;
    let metadata = open_tree::<T>(&database, METATREE)?;
    let mut out: Vec<MetaValues> = Vec::new();
    let meta_key_prefix = MetaKeyPrefix::from_name(network_name);
    for x in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
        let meta_values = MetaValues::from_entry_checked::<T>(x)?;
        if meta_values.name == network_name {out.push(meta_values)}
    }
    Ok(out)
}

/// Function to get MetaValues corresponding to given network name and version.
/// Returns error if finds nothing.
/// Applicable to both Active side and Signer side.
pub fn get_meta_values_by_name_version <T: ErrorSource> (database_name: &str, network_name: &str, network_version: u32) -> Result<MetaValues, T::Error> {
    let database = open_db::<T>(database_name)?;
    let metadata = open_tree::<T>(&database, METATREE)?;
    let meta_key = MetaKey::from_parts(network_name, network_version);
    match metadata.get(meta_key.key()) {
        Ok(Some(meta)) => MetaValues::from_entry_name_version_checked::<T>(network_name, network_version, meta),
        Ok(None) => Err(<T>::metadata_not_found(network_name.to_string(), network_version)),
        Err(e) => Err(<T>::db_internal(e)),
    }
}

/// Function to modify existing batch for ADDRTREE with incoming vector of additions
pub (crate) fn upd_id_batch (mut batch: Batch, adds: Vec<(AddressKey, AddressDetails)>) -> Batch {
    for (address_key, address_details) in adds.iter() {batch.insert(address_key.key(), address_details.encode());}
    batch
}

/// Function to verify checksum in Signer database
pub fn verify_checksum (database: &Db, checksum: u32) -> Result<(), ErrorSigner> {
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(<Signer>::db_internal(e)),
    };
    if checksum != real_checksum {return Err(ErrorSigner::Database(DatabaseSigner::ChecksumMismatch))}
    Ok(())
}

/// Function to get the danger status from the Signer database.
/// Function interacts with user interface.
pub fn get_danger_status(database_name: &str) -> Result<bool, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let settings = open_tree::<Signer>(&database, SETTREE)?;
    match settings.get(DANGER.to_vec()) {
        Ok(Some(a)) => DangerRecord::from_ivec(&a).device_was_online(),
        Ok(None) => Err(ErrorSigner::NotFound(NotFoundSigner::DangerStatus)),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    use definitions::{keyring::VerifierKey, network_specs::{ValidCurrentVerifier, Verifier}};
    use hex;
    use std::fs;
    
    use crate::{cold_default::{populate_cold_no_metadata, populate_cold_release, signer_init_no_cert, signer_init_with_cert}, manage_history::{device_was_online, reset_danger_status_to_safe}};

    #[test]
    fn get_danger_status_properly () {
        let dbname = "for_tests/get_danger_status_properly";
        populate_cold_release(dbname).unwrap();
        signer_init_no_cert(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == false, "Expected danger status = false after the database initiation.");
        device_was_online(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == true, "Expected danger status = true after the reported exposure.");
        reset_danger_status_to_safe(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == false, "Expected danger status = false after the danger reset.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn display_general_verifier_properly() {
        let dbname = "for_tests/display_general_verifier_properly";
        populate_cold_release(dbname).unwrap();
        signer_init_no_cert(dbname).unwrap();
        let print = display_general_verifier(dbname).unwrap();
        assert!(print == r#""public_key":"","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea20000002e49444154789cedcd410100200c0021ed1f7ab6381f8302dc99393f8833e28c3823ce8833e28c3823ce8833fbe20724cf59c50a861d5c0000000049454e44ae426082","encryption":"none""#, "Got: {}", print);
        signer_init_with_cert(dbname).unwrap();
        let print = display_general_verifier(dbname).unwrap();
        assert!(print == r#""public_key":"c46a22b9da19540a77cbde23197e5fd90485c72b4ecf3c599ecca6998f39bd57","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000abd49444154789c7557097454e515fefeb7cd9b25934042820211886c9204900402c8264b50e400258280d0ca762278b0ca625b9082158ab21c7b400a85822c874221201e018f0610651142c3225b58840849484220c9ccbc376fef7d8350eb72cffc7366e6bdf77fffbdf7bbdfbdc31cc7c1af184fcba21533cbb23a1b86315837f4decc614de3fc711618105115de348d1a49928e89a2b84f1084c3eeedb45c136899b47e66bf06fce8015dd7476b9a36dd34cd1cdbb621891ea89a822f4e1420aaabe8dbf905344e4a85120d83e778f03c7f990eb1d6e3f1ac628c45690bd7019bd6ff01fd12700cd4b2ccac4844d9601a46bae390738cb98b3ef06cfad2116cdf91430ec780a75a36631be61f7092e2931dc3d0387a966ee420086295d7eb9d4687d819fbed813d02fb29700c943c9ca428ca5ac7b1c1091e9397028cbce5fd1e0ec7fef319c6cc7d0e097152ccc3f2bb2a164e5d8c292fbe85da88e57a6cdb866a598622d2f620f0e5b466d0beaeb9078801fe183806aaaaeabbb4e630bace09b2a987ef0815275741ad2d45938c91a88d4fc7d0d7db8323df445140f53d13eb16ec44afd466b8faf572889e20523a4f823f25830ea0d0ee8c27aff704028161b4bf6b31f087c031d068343a291c0eade5383a3585571064eefca681a8b9f00d780f258a329533b5101f9dfc1a7fdfb600167d1fdc2b0f0bc62fc0e9d559d06a55da06f02625a2637e1178b98103c732693351f6ca1f06fc81d7e8b29b73cb058e7d20c6666951bd4814241846d4967d715cd5cd1338b33a079c2851da04186185bc198decf15bf16dc931225404d91d06e0eac1e528d93d039ea08fb672a08754b41db5162dbb4f423414022f48a665db022fb009b22c6fa09b0417186e2eb5a8f1ed8ddb25e9ef6d9a6156d6dc16fa660f43fed01938b9221deadd4a085e06adde41c6cbcb6126bf81338709823c7eb2a38d260d0a716a652e1d90b674e845abc71be7b0ff4211b6ee5d0ed9e3b3a78d9ccf75cf1c10e525d69c4e50190326328d367473eb948583ccfdc78e080de280fb75c092592b31bc5d079cde3106b67a17c9996391dc633976ac8f43880ec1f3846033fc66820d56f13e4a0f2ea640f2683370296ec8691833bb0f044a6254075a366964ec78ef94d828b1f1528f479ac56c3a7654891eafa8be9d336c66a645f5ca8b14dafbf50a06767d8e88b30f5555d5d042113c96d618d72f5ad8bdc90faf8f4019094888a157ae821efd78549456c321d23569d614cb36cec1d2cd8b90d2d0079b9cab0da9cec6f97bd9339d726b98e0b46594dbcef5f5f5a73c928cc97fc9c5bea347d02008dc238f3f98b518fdd3dfc2273b0d522e11cdd35474e91dc5ae4df1b857c52877c47ca2f7a8c9f528bdc6e1dca90011d0419ffe0e42d27e8c9cfd02046250cce3a6492878bfd80a7883bcc7eb99c2a874e64522e1059407abbcea06bf62c7bbb8597913fd3a0fc4b8dc19d8fecf38dcaf61f0c80ec2f50ccfe545d0b8a981630713601a40a79c305db3b07d5d3c240ff185424fef189f6fe2e8e50dd8fac516c479fdc81f360b1ddb74b723d130277be4bd8cbc3d40e17dd6b62ddb1748e2ec5019ea2a2f2139ad2bcaab646c5bc3439438f2cc811266c8c83630688486bbb78a619951243fd119c5c78338f089087f1c859f5e6ef807bda8e1e96c0e774abe811c97045f723b844235a4761c0920bbcbeaeaea4a4c436f2d78139c3bc5ebd9b54ff261690ee446a9e834612f0e1f4cc7b745241612f1467090f78a01a5623a6e9e5de3a61849a97dd13c7b3b0a3635c2fd6ac225e0c79a391836b606570a46a0e6f257244440d39e6fa0e5c0c5248b21925e0eacaeb6f6a2653bed1c4bb34fafeac0a9f72a21fa64446ba348edfd3bb418bc01678f86a0aa3cd2da79286c47716a4f6ff0543a8cea478b1848eff721e4c4a9b8541c761d46464e00e1cb7fc3857ffd1e9e7809b64949a6d2ebf8ea0957d1e02a1aab2560db61ed2cadd62e5e99c1996a3da9940f3ab1ba71561eda8ff9376e5eac83aa7078a29597d87d08c59fe692348a604c2060156d7a2ec1636d66e2bb4bb5040ba46524e0f6d70b71f5e3b9f0247849bc889c9a890e530e21ae5937587ad8a150d7fe10ea864ee981b9ecfafec5b1538b2442d9f9853877a51f8e7f0e503f40b08183e1e3ea517e7638caaf1e2260fa8d5a62fbfe85f86c772b945ea707c9b3361d807e83aee1ccba3e089595516828fc5943d06ee436f2963a253de892eb18955437ba680ba28f332b8aa0545d407ccb675067b7c1f6d5844825239058848934593d6df41a5885caebbb605b0a925b0cc1954badb17f07885c8440a756c2c00b630db46e710735173f87144886dcbc2f0cd3746108970b316a7f4b15559921f2a2a51951bee0f0565c2fbf86fe590390fd642e36ae9260ea1c91cb417d1d436f128bcc2e3a8abf4980a103995911dcbf6b62f7e678f8fce43199aa3a183bc9c23dfb04761e2a40bc3f0ea3fa8e4752428aade91a47ddea2b1210fd5962f681802fdef9c38af16cfd9e6df07ba96b58c0fa773621d5330e7b7751c152034b799c4a294fc1e7bb03b8799573d511710920a68770f24b0925e76512101b393d1992db16216f667752401bba093cd331031bdff9d2a286c8fb7cdeb7620d5e51d4f3d5f72ada0e7d33d326af398f24a3a656c1e01e83b1e6cf9fa2f4964924b2d1348da1ecba8a828d41c85e8742f6a066fb3e1f46561f01df5f132890165a3de9c1079be762f1470b91924892695ba80b6bcee677f6b39c8c7e2a2722e361937893c69c6563e67433ce5c29111b044977ab2dfc71d29ff0daf3f938ffe91498a15b48797a32a4275ec5f63522e92f4926e55d5580a1e38044f1636a120bc10411ad072cc391b20a4c9c978786f18046010b784573d792d3428ba6adb7481e715c0cd8b22dd9d29dd263e70a93df5e35d1ae0dd7705dd39fc5a2691b50b239173557ce525ba4f0534e9f9ebc15e5ea68141db4492c38b4ca34d1a9c359147f9845930471cb06a4a08caed32f60c59e1528285c4dc4949cd7c72cc44b03f2a902edf6d4842ebac0a42b24289a966719f60ed2522b1cb9cf354e69c5eacb4ee1d4aa2e103c347e303e360824771e852ee3b7a1ec7639a29a8eb4b4e6b87c6009d5ec6c02a41a24d3430a9e1ab50ea9dd26a2bcec1a4da65e233e902882b317f97cbe3974cb8341808c68028b9ac5325d37de1478d1302c43740bffdc3fba522dde842053c82254db13b7e078bd880f364ea310463166f06c8ceb968b132bbb92bbf4228fdd7b3bbe5a0cb9612bdad8d6a92d4aa49285f1c1f80184e31a7b084c348999130e873fd6b4e85092729393fc5cf87611f7fd97f311ad2f47e3f62320654ec090e96d10524c784480da34b62d3d8856d62d5c3dfc1e955d008f777b1d894f8d706c3d6c52971469c8bf120c063b5073a06480a3653f0476cd058f7da1da5e4983df34f72b2ff90dc678c13454161f0ce2ab93fb306ede60228d0cea34b853a360dee47731f5a5395409219a38248704c2b2b410510f8cfe5d14d28439e407d0586469e1c7c0ae3d02d775ed152ab3553420cb24707485593ce358448bb097dfeec5ce94dc20578086410e5b171e45ebd4743baa470994f18e4df3382f8006bb453453bb3975ed11a86b3f05768d5062e1a0c1d04e21d2cda4f50a7d4e748742afecc7ad3bdf61cbfe15442e05c3fafe1659ed7a51e8eb6211a014a9a2241510e85f69b8bf48fbb8e6ee47d9ff9ffd12f043237f1efc7fa27b92e83fd470d2f4a1a4745d45c193e4f7fa630f46358da95a244425729a06fcbd248705340e5da74baeb97b58b462f7fed8fe0bb74228b164fe5e050000000049454e44ae426082","encryption":"sr25519""#, "Got: {}", print);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn find_westend_verifier() {
        let dbname = "for_tests/find_westend_verifier";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let verifier_key = VerifierKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap());
        let westend_verifier = try_get_valid_current_verifier(&verifier_key, dbname).unwrap();
        assert!(westend_verifier == Some(ValidCurrentVerifier::General));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn not_find_mock_verifier() {
        let dbname = "for_tests/not_find_mock_verifier";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let verifier_key = VerifierKey::from_parts(&hex::decode("62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b").unwrap());
        match try_get_valid_current_verifier(&verifier_key, dbname) {
            Ok(Some(_)) => panic!("Found network key that should not be in database."),
            Ok(None) => (),
            Err(e) => panic!("Error looking for mock verifier: {}", <Signer>::show(&e)),
        }
        fs::remove_dir_all(dbname).unwrap();
    }
}
