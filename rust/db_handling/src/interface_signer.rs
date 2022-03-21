use bip39::{Language, Mnemonic};
use blake2_rfc::blake2b::blake2b;
use hex;
use parity_scale_codec::Encode;
use plot_icon::EMPTY_PNG;
use sp_core::{Pair, sr25519};
use sp_runtime::MultiSigner;
use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

use constants::{HISTORY, MAX_WORDS_DISPLAY, TRANSACTION};
use definitions::{error::{AddressGenerationCommon, ErrorSource}, error_signer::{DatabaseSigner, ErrorSigner, InterfaceSigner, NotFoundSigner, Signer}, helpers::{make_identicon_from_multisigner, multisigner_to_encryption, multisigner_to_public, pic_meta, print_multisigner_as_base58}, keyring::{AddressKey, NetworkSpecsKey, VerifierKey}, network_specs::NetworkSpecs, print::{export_complex_vector, export_plain_vector}, qr_transfers::ContentLoadTypes, users::AddressDetails};
use qrcode_static::png_qr_from_string;

use crate::db_transactions::TrDbCold;
use crate::helpers::{get_address_details, get_general_verifier, get_meta_values_by_name, get_meta_values_by_name_version, get_network_specs, get_valid_current_verifier, make_batch_clear_tree, open_db, open_tree, try_get_types};
use crate::identities::{derivation_check, DerivationCheck, get_all_addresses, get_addresses_by_seed_name, generate_random_phrase};
use crate::network_details::get_all_networks;

/// Function to print all seed names with identicons
/// Gets used only on the Signer side, interacts with the user interface.
pub fn print_all_seed_names_with_identicons (database_name: &str, names_phone_knows: &[String]) -> Result<String, ErrorSigner> {
    let mut data_set: HashMap<String, Vec<MultiSigner>> = HashMap::new();
    for (multisigner, address_details) in get_all_addresses(database_name)?.into_iter() {
        if address_details.is_root() {
        // found a root; could be any of the supported encryptions;
            match data_set.get(&address_details.seed_name) {
                Some(root_set) => {
                    for id in root_set.iter() {
                        if multisigner_to_encryption(id) == address_details.encryption {
                            return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys{seed_name: address_details.seed_name.to_string(), encryption: address_details.encryption.to_owned()}))
                        }
                    }
                    let mut new_root_set = root_set.to_vec();
                    new_root_set.push(multisigner);
                    data_set.insert(address_details.seed_name.to_string(), new_root_set);
                },
                None => {data_set.insert(address_details.seed_name.to_string(), vec![multisigner]);},
            }
        }
        else if data_set.get(&address_details.seed_name).is_none() {data_set.insert(address_details.seed_name.to_string(), Vec::new());}
    }
    for x in names_phone_knows.iter() {if data_set.get(x).is_none() {data_set.insert(x.to_string(), Vec::new());}}
    let mut print_set: Vec<(String, String)> = Vec::new();
    for (seed_name, multisigner_set) in data_set.into_iter() {
        let identicon_string = preferred_multisigner_identicon(&multisigner_set);
        print_set.push((identicon_string, seed_name))
    }
    print_set.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(export_complex_vector(&print_set, |(identicon_string, seed_name)| format!("\"identicon\":\"{}\",\"seed_name\":\"{}\"", identicon_string, seed_name)))
}

fn preferred_multisigner_identicon(multisigner_set: &[MultiSigner]) -> String {
    if multisigner_set.is_empty() {hex::encode(EMPTY_PNG)}
    else {
        let mut got_sr25519 = None;
        let mut got_ed25519 = None;
        let mut got_ecdsa = None;
        for x in multisigner_set.iter() {
            match x {
                MultiSigner::Ed25519(_) => got_ed25519 = Some(x.to_owned()),
                MultiSigner::Sr25519(_) => got_sr25519 = Some(x.to_owned()),
                MultiSigner::Ecdsa(_) => got_ecdsa = Some(x.to_owned()),
            }
        }
        if let Some(a) = got_sr25519 {hex::encode(make_identicon_from_multisigner(&a))}
        else if let Some(a) = got_ed25519 {hex::encode(make_identicon_from_multisigner(&a))}
        else if let Some(a) = got_ecdsa {hex::encode(make_identicon_from_multisigner(&a))}
        else {hex::encode(EMPTY_PNG)}
    }
}

/// Function to print all identities (seed names AND derication paths) with identicons
/// Gets used only on the Signer side, interacts with the user interface.
pub fn print_all_identities (database_name: &str) -> Result<String, ErrorSigner> {
    Ok(export_complex_vector(&get_all_addresses(database_name)?, |(multisigner, address_details)| {
        let address_key = AddressKey::from_multisigner(multisigner); // to click
        let public_key = multisigner_to_public(multisigner); // to display
        let hex_identicon = hex::encode(make_identicon_from_multisigner(multisigner));
        format!("\"seed_name\":\"{}\",\"address_key\":\"{}\",\"public_key\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":{},\"path\":\"{}\"", address_details.seed_name, hex::encode(address_key.key()), hex::encode(public_key), hex_identicon, address_details.has_pwd, address_details.path)
    }))
}

/// Function to print separately root identity and derived identities for given seed name and network specs key.
/// Is used only on the Signer side, interacts only with navigation.
pub fn print_identities_for_seed_name_and_network (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey, swiped_key: Option<MultiSigner>, multiselect: Vec<MultiSigner>) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let identities = addresses_set_seed_name_network (database_name, seed_name, network_specs_key)?;
    let mut root_id = None;
    let mut other_id: Vec<(MultiSigner, AddressDetails, Vec<u8>, bool, bool)> = Vec::new();
    for (multisigner, address_details) in identities.into_iter() {
        let identicon = make_identicon_from_multisigner(&multisigner);
        let base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
        let address_key = AddressKey::from_multisigner(&multisigner);
        let swiped = {
            if let Some(ref swiped_multisigner) = swiped_key {swiped_multisigner == &multisigner}
            else {false}
        };
        let is_multiselect = multiselect.contains(&multisigner);
        if address_details.is_root() {
            if root_id.is_some() {return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys{seed_name: seed_name.to_string(), encryption: network_specs.encryption}))}
            root_id = Some(format!("\"seed_name\":\"{}\",\"identicon\":\"{}\",\"address_key\":\"{}\",\"base58\":\"{}\",\"swiped\":{},\"multiselect\":{}", seed_name, hex::encode(identicon), hex::encode(address_key.key()), base58, swiped, is_multiselect));
        }
        else {other_id.push((multisigner, address_details, identicon, swiped, is_multiselect))}
    }
    let root_print = match root_id {
        Some(a) => a,
        None => format!("\"seed_name\":\"{}\",\"identicon\":\"{}\",\"address_key\":\"\",\"base58\":\"\",\"swiped\":false,\"multiselect\":false", seed_name, hex::encode(EMPTY_PNG)),
    };
    let other_print = export_complex_vector(&other_id, |(multisigner, address_details, identicon, swiped, is_multiselect)| format!("\"address_key\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":{},\"path\":\"{}\",\"swiped\":{},\"multiselect\":{}", hex::encode(AddressKey::from_multisigner(multisigner).key()), print_multisigner_as_base58(multisigner, Some(network_specs.base58prefix)), hex::encode(identicon), address_details.has_pwd, address_details.path, swiped, is_multiselect));
    
    Ok(format!("\"root\":{{{}}},\"set\":{},\"network\":{{\"title\":\"{}\",\"logo\":\"{}\"}}", root_print, other_print, network_specs.title, network_specs.logo))
}

/// Function to get addresses for given seed name and network specs key
pub fn addresses_set_seed_name_network (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<Vec<(MultiSigner, AddressDetails)>, ErrorSigner> {
    Ok(get_addresses_by_seed_name(database_name, seed_name)?
        .into_iter()
        .filter(|(_, address_details)| address_details.network_id.contains(network_specs_key))
        .collect())
}

/// Function to print all networks, with bool indicator which one is currently selected
pub fn show_all_networks_with_flag (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let mut networks = get_all_networks::<Signer>(database_name)?;
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(format!("\"networks\":{}", export_complex_vector(&networks, |a| 
        {
            let network_specs_key_current = NetworkSpecsKey::from_parts(&a.genesis_hash, &a.encryption);
            format!("\"key\":\"{}\",\"title\":\"{}\",\"logo\":\"{}\",\"order\":{},\"selected\":{}", hex::encode(network_specs_key_current.key()), a.title, a.logo, a.order, &network_specs_key_current == network_specs_key)
        }
    )))
}

/// Function to print all networks without any selection
pub fn show_all_networks (database_name: &str) -> Result<String, ErrorSigner> {
    let mut networks = get_all_networks::<Signer>(database_name)?;
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(format!("\"networks\":{}", export_complex_vector(&networks, |a| 
        {
            let network_specs_key_current = NetworkSpecsKey::from_parts(&a.genesis_hash, &a.encryption);
            format!("\"key\":\"{}\",\"title\":\"{}\",\"logo\":\"{}\",\"order\":{}", hex::encode(network_specs_key_current.key()), a.title, a.logo, a.order)
        }
    )))
}

/// Function to sort networks by the order and get the network specs for the first network on the list.
/// If no networks in the system, throws error
pub fn first_network (database_name: &str) -> Result<NetworkSpecs, ErrorSigner> {
    let mut networks = get_all_networks::<Signer>(database_name)?;
    if networks.is_empty() {return Err(ErrorSigner::NoNetworksAvailable)}
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(networks.remove(0))
}

/// Function to prepare the export key screen.
/// Contains among else the QR code with identity information, in format 
/// `substrate:{public_key as as_base58}:0x{network_key}`,
/// this string is transformed into bytes, then into png qr code, then qr code
/// content is hexed so that it could be transferred into app.
pub fn export_key (database_name: &str, multisigner: &MultiSigner, expected_seed_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let address_key = AddressKey::from_multisigner(multisigner);
    let address_details = get_address_details(database_name, &address_key)?;
    if address_details.seed_name != expected_seed_name {return Err(ErrorSigner::Interface(InterfaceSigner::SeedNameNotMatching{address_key, expected_seed_name: expected_seed_name.to_string(), real_seed_name: address_details.seed_name}))}
    let address_base58 = print_multisigner_as_base58(multisigner, Some(network_specs.base58prefix));
    let public_key = multisigner_to_public(multisigner);
    let identicon = make_identicon_from_multisigner(multisigner);
    let qr_prep = {
        if address_details.network_id.contains(network_specs_key) {
            match png_qr_from_string(&format!("substrate:{}:0x{}", address_base58, hex::encode(&network_specs.genesis_hash))) {
                Ok(a) => a,
                Err(e) => return Err(ErrorSigner::Qr(e.to_string())),
            }
        }
        else {return Err(ErrorSigner::NotFound(NotFoundSigner::NetworkSpecsKeyForAddress{network_specs_key: network_specs_key.to_owned(), address_key}))}
    };
    Ok(format!("\"qr\":\"{}\",\"pubkey\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"seed_name\":\"{}\",\"path\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\"", hex::encode(qr_prep), hex::encode(public_key), address_base58, hex::encode(identicon), address_details.seed_name, address_details.path, network_specs.title, network_specs.logo))
}

/// Function to prepare seed backup screen.
/// Gets seed name, outputs all known derivations in all networks.
pub fn backup_prep (database_name: &str, seed_name: &str) -> Result<String, ErrorSigner> {
    let networks = get_all_networks::<Signer>(database_name)?;
    if networks.is_empty() {return Err(ErrorSigner::NoNetworksAvailable)}
    let mut export: Vec<(NetworkSpecs, Vec<AddressDetails>)> = Vec::new();
    for x in networks.into_iter() {
        let id_set = addresses_set_seed_name_network (database_name, seed_name, &NetworkSpecsKey::from_parts(&x.genesis_hash, &x.encryption))?;
        if !id_set.is_empty() {export.push((x, id_set.into_iter().map(|(_, a)| a).collect()))}
    }
    export.sort_by(|(a, _), (b, _)| a.order.cmp(&b.order));
    Ok(format!("\"seed_name\":\"{}\",\"derivations\":{}", seed_name, export_complex_vector(&export, |(specs, id_set)| format!("\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_order\":{},\"id_set\":{}", specs.title, specs.logo, specs.order, export_complex_vector(id_set, |a| format!("\"path\":\"{}\",\"has_pwd\":{}", a.path, a.has_pwd))))))
}

/// Function to prepare key derivation screen.
/// Gets seed name, network specs key and suggested derivation
pub fn derive_prep (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey, collision: Option<(MultiSigner, AddressDetails)>, suggest: &str) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    match collision {
        Some((multisigner, address_details)) => {
            let address_base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
            let hex_identicon = hex::encode(make_identicon_from_multisigner(&multisigner));
            let collision_display = format!("\"base58\":\"{}\",\"path\":\"{}\",\"has_pwd\":{},\"identicon\":\"{}\",\"seed_name\":\"{}\"", address_base58, address_details.path, address_details.has_pwd, hex_identicon, seed_name);
            Ok(format!("\"seed_name\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_specs_key\":\"{}\",\"suggested_derivation\":\"{}\",\"collision\":{{{}}}", seed_name, network_specs.title, network_specs.logo, hex::encode(network_specs_key.key()), suggest, collision_display))
        },
        None => Ok(format!("\"seed_name\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_specs_key\":\"{}\",\"suggested_derivation\":\"{}\"", seed_name, network_specs.title, network_specs.logo, hex::encode(network_specs_key.key()), suggest)),
    }
}

/// Function to show (dynamically) if the derivation with the provided path and no password already exists
/// and if it exists, prints its details
pub fn dynamic_path_check (database_name: &str, seed_name: &str, path: &str, network_specs_key_hex: &str) -> String {
    let content = match NetworkSpecsKey::from_hex(network_specs_key_hex) {
        Ok(network_specs_key) => {
            match get_network_specs(database_name, &network_specs_key) {
                Ok(network_specs) => {
                    match derivation_check (seed_name, path, &network_specs_key, database_name) {
                        Ok(DerivationCheck::BadFormat) => String::from("\"button_good\":false"),
                        Ok(DerivationCheck::Password) => String::from("\"button_good\":true,\"where_to\":\"pwd\""),
                        Ok(DerivationCheck::NoPassword(None)) => String::from("\"button_good\":true,\"where_to\":\"pin\""),
                        Ok(DerivationCheck::NoPassword(Some((multisigner, address_details)))) => {
                            let address_base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
                            let hex_identicon = hex::encode(make_identicon_from_multisigner(&multisigner));
                            let collision_display = format!("\"base58\":\"{}\",\"path\":\"{}\",\"has_pwd\":{},\"identicon\":\"{}\",\"seed_name\":\"{}\"", address_base58, address_details.path, address_details.has_pwd, hex_identicon, seed_name);
                            format!("\"button_good\":false,\"collision\":{{{}}}", collision_display)
                        },
                        Err(e) => format!("\"error\":\"{}\"", <Signer>::show(&e)),
                    }
                },
                Err(e) => format!("\"error\":\"{}\"", <Signer>::show(&e)),
            }
        },
        Err(e) => format!("\"error\":\"{}\"", <Signer>::show(&e)),
    };
    format!("{{\"derivation_check\":{{{}}}}}", content)
}

/// Print network specs and metadata set information for network with given network specs key.
pub fn network_details_by_key (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash);
    let general_verifier = get_general_verifier(database_name)?;
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, database_name)?;
    let relevant_meta = get_meta_values_by_name::<Signer>(database_name, &network_specs.name)?;
    let metadata_print = export_complex_vector(&relevant_meta, |a| {
        let meta_hash = blake2b(32, &[], &a.meta).as_bytes().to_vec();
        let hex_id_pic = hex::encode(pic_meta(&meta_hash));
        format!("\"spec_version\":\"{}\",\"meta_hash\":\"{}\",\"meta_id_pic\":\"{}\"", a.version, hex::encode(meta_hash), hex_id_pic)
    });
    Ok(format!("{},\"meta\":{}", network_specs.show(&valid_current_verifier, &general_verifier), metadata_print))
}

/// Print metadata details for given network specs key and version.
pub fn metadata_details (database_name: &str, network_specs_key: &NetworkSpecsKey, network_version: u32) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_values = get_meta_values_by_name_version::<Signer>(database_name, &network_specs.name, network_version)?;
    let relevant_networks: Vec<NetworkSpecs> = get_all_networks::<Signer>(database_name)?
        .into_iter()
        .filter(|a| a.name == network_specs.name)
        .collect()
    ;
    let relevant_networks_print = export_complex_vector(&relevant_networks, |a| {
        format!("\"title\":\"{}\",\"logo\":\"{}\",\"order\":{},\"current_on_screen\":{}", a.title, a.logo, a.order, &NetworkSpecsKey::from_parts(&a.genesis_hash, &a.encryption) == network_specs_key)
    });
    let meta_hash = blake2b(32, &[], &meta_values.meta).as_bytes().to_vec();
    let hex_id_pic = hex::encode(pic_meta(&meta_hash));
    Ok(format!("\"name\":\"{}\",\"version\":\"{}\",\"meta_hash\":\"{}\",\"meta_id_pic\":\"{}\",\"networks\":{}", network_specs.name, network_version, hex::encode(meta_hash), hex_id_pic, relevant_networks_print))
}

/// Display types status
pub fn show_types_status (database_name: &str) -> Result<String, ErrorSigner> {
    match try_get_types::<Signer>(database_name)? {
        Some(a) => Ok(format!("\"types_on_file\":true,{}", ContentLoadTypes::generate(&a).show())),
        None => Ok(String::from("\"types_on_file\":false")),
    }
}

/// Function to generate new random seed phrase, make identicon for sr25519 public key,
/// and send to Signer screen
pub fn print_new_seed (seed_name: &str) -> Result<String, ErrorSigner> {
    let seed_phrase = generate_random_phrase(24)?;
    let sr25519_pair = match sr25519::Pair::from_string(&seed_phrase, None) {
        Ok(x) => x,
        Err(e) => return Err(<Signer>::address_generation_common(AddressGenerationCommon::SecretString(e))),
    };
    let hex_identicon = hex::encode(make_identicon_from_multisigner(&MultiSigner::Sr25519(sr25519_pair.public())));
    Ok(format!("\"seed\":\"{}\",\"seed_phrase\":\"{}\",\"identicon\":\"{}\"", seed_name, seed_phrase, hex_identicon))
}


/// Function to get database history tree checksum to be displayed in log screen
pub fn history_hex_checksum (database_name: &str) -> Result<String, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let history = open_tree::<Signer>(&database, HISTORY)?;
    let checksum = history.checksum().map_err(|e| ErrorSigner::Database(DatabaseSigner::Internal(e)))?;
    Ok(format!("\"checksum\":\"{}\"", hex::encode(checksum.encode()).to_uppercase()))
}


/// Function to clear transaction tree of the database, for cases when transaction is declined by the user
/// (e.g. user has scanned something, read it, clicked `back`)
pub fn purge_transactions (database_name: &str) -> Result<(), ErrorSigner> {
    TrDbCold::new()
        .set_transaction(make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?) // clear transaction
        .apply::<Signer>(database_name)
}


/// Function to display possible options of English code words from allowed words list
/// that start with already entered piece
pub(crate) fn guess (word_part: &str) -> Vec<&'static str> {
    let dictionary = Language::English.wordlist();
    let words = dictionary.get_words_by_prefix(word_part);
    if words.len() > MAX_WORDS_DISPLAY {words[..MAX_WORDS_DISPLAY].to_vec()}
    else {words.to_vec()}
}


/// Function to dynamically update suggested seed phrase words,
/// based on user entered word part.
/// Produces json-like export with maximum 8 fitting words, sorted by alphabetical order.
pub fn print_guess (user_entry: &str) -> String {
    export_plain_vector(&guess(user_entry))
}


pub const BIP_CAP: usize = 24; // maximum word count in bip39 standard, see https://docs.rs/tiny-bip39/0.8.2/src/bip39/mnemonic_type.rs.html#60
pub const WORD_LENGTH: usize = 8; // maximum word length in bip39 standard
pub const SAFE_RESERVE: usize = 1000; // string length to reserve for json output of numbered draft; each element is {"order":**,"content":"********"}, at most 33+1(comma) symbols for each max BIP_CAP elements, two extras for [ and ], and some extra space just in case


/// Struct to store seed draft, as entered by user
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SeedDraft {
    user_input: String,
    saved: Vec<SeedElement>,
}

/// Struct to store seed element, as entered by user
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
struct SeedElement(String);

impl SeedElement {
    fn from_checked_str(word: &str) -> Self {
        let mut new = String::with_capacity(WORD_LENGTH);
        new.push_str(word);
        SeedElement(new)
    }
    fn word(&self) -> &str {
        &self.0
    }
}


impl SeedDraft {
    pub fn initiate() -> Self {
        Self {
            user_input: String::with_capacity(WORD_LENGTH), // capacity corresponds to maximum word length in bip39 standard;
            saved: Vec::with_capacity(BIP_CAP), // capacity corresponds to maximum word count in bip39 standard; set here to avoid reallocation;
        }
    }
    pub fn text_field_update(&mut self, user_text: &str) {
        if self.saved.len() < BIP_CAP {
            if user_text.is_empty() {
                // user has removed all text, including the first default symbol
                // if there are words in draft, remove the last one
                self.remove_last();
                // restore the user input to emtpy one
                self.user_input.clear();
            }
            else {
                let user_text = user_text.trim_start();
                if user_text.ends_with(' ') {
                    let word = user_text.trim();
                    if self.added(word, None) {self.user_input.clear()}
                    else if !guess(word).is_empty() {self.user_input = String::from(word)}
                }
                else if !guess(user_text).is_empty() {self.user_input = String::from(user_text)}
            }
        }
        else {self.user_input.clear()}
    }
    pub fn added(&mut self, word: &str, position: Option<u32>) -> bool {
        if self.saved.len() < BIP_CAP {
            let guesses = guess(word);
            let definitive_guess = {
                if guesses.len() == 1 {Some(guesses[0])}
                else if guesses.contains(&word) {Some(word)}
                else {None}
            };
            if let Some(guess) = definitive_guess {
                let new = SeedElement::from_checked_str(guess);
                match position {
                    Some(p) => {
                        let p = p as usize;
                        if p <= self.saved.len() {self.saved.insert(p, new)}
                        else {self.saved.push(new)}
                    },
                    None => self.saved.push(new),
                }
                self.user_input.clear();
                true
            }
            else {false}
        }
        else {false}
    }
    pub fn remove(&mut self, position: u32) {
        let position = position as usize;
        if position < self.saved.len() {
            self.saved.remove(position);
        }
    }
    pub fn remove_last(&mut self) {
        if !self.saved.is_empty() {
            self.saved.remove(self.saved.len()-1);
        }
    }
    pub fn print(&self) -> String {
        let mut out = String::with_capacity(SAFE_RESERVE);
        out.push('[');
        for (i,x) in self.saved.iter().enumerate() {
            if i>0 {out.push(',')}
            out.push_str(&format!("{{\"order\":{},\"content\":\"", i));
            out.push_str(x.word());
            out.push_str("\"}");
        }
        out.push(']');
        out
    }
    /// Combines all draft elements into seed phrase proposal,
    /// and checks its validity.
    /// If valid, outputs secret seed phrase.
    pub fn try_finalize(&self) -> Option<String> {
        let mut seed_phrase_proposal = String::with_capacity((WORD_LENGTH+1)*BIP_CAP);
        for (i, x) in self.saved.iter().enumerate() {
            if i>0 {seed_phrase_proposal.push(' ');}
            seed_phrase_proposal.push_str(x.word());
        }
        if Mnemonic::validate(&seed_phrase_proposal, Language::English).is_ok() {Some(seed_phrase_proposal)}
        else {
            seed_phrase_proposal.zeroize();
            None
        }
    }
    /// Function to output the user input back into user interface
    pub fn user_input(&self) -> &str {
        &self.user_input
    }
}
