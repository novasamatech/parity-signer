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
use definitions::{error::{AddressGenerationCommon, DatabaseSigner, ErrorSigner, ErrorSource, InterfaceSigner, NotFoundSigner, Signer}, helpers::{make_identicon_from_multisigner, multisigner_to_encryption, multisigner_to_public, pic_meta}, keyring::{AddressKey, NetworkSpecsKey, print_multisigner_as_base58, VerifierKey}, network_specs::NetworkSpecs, print::{export_complex_vector, export_plain_vector}, qr_transfers::ContentLoadTypes, users::AddressDetails};
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
            Ok(format!("\"seed_name\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\",\"suggested_derivation\":\"{}\",\"collision\":{{{}}}", seed_name, network_specs.title, network_specs.logo, suggest, collision_display))
        },
        None => Ok(format!("\"seed_name\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\",\"suggested_derivation\":\"{}\"", seed_name, network_specs.title, network_specs.logo, suggest)),
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
    format!("\"derivation_check\":{{{}}}", content)
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
fn guess (word_part: &str) -> Vec<&'static str> {
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

#[cfg(test)]
mod tests {
    
    use sp_core::sr25519::Public;
    use std::fs;
    use std::convert::TryInto;
    
    use constants::test_values::{ALICE_SR_ROOT, ALICE_SR_ALICE, ALICE_SR_KUSAMA, ALICE_SR_POLKADOT, ALICE_SR_WESTEND, ALICE_SR_SECRET_ABRACADABRA, EMPTY_PNG, WESTEND_9000, WESTEND_9010, TYPES_KNOWN, ALICE_WESTEND_ROOT_QR};
    use definitions::{crypto::Encryption, keyring::NetworkSpecsKey, network_specs::Verifier};
    
    use crate::cold_default::populate_cold;
    use crate::identities::try_create_address;
    use crate::manage_history::print_history;
    use crate::remove_types::remove_types_info;
    
    use super::*;
    
    const ALICE_SEED_PHRASE: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

    #[test]
    fn print_seed_names() {
        let dbname = "for_tests/print_seed_names";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = print_all_seed_names_with_identicons(dbname, &vec![String::from("Alice")]).unwrap()
            .replace(ALICE_SR_ROOT, r#"<alice_sr25519_root>"#);
        let expected_print = r#"[{"identicon":"<alice_sr25519_root>","seed_name":"Alice"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_seed_names_with_orphan() {
        let dbname = "for_tests/print_seed_names_with_orphan";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = print_all_seed_names_with_identicons(dbname, &vec![String::from("Alice"), String::from("BobGhost")]).unwrap()
            .replace(ALICE_SR_ROOT, r#"<alice_sr25519_root>"#)
            .replace(EMPTY_PNG, r#"<empty>"#);
        let expected_print = r#"[{"identicon":"<alice_sr25519_root>","seed_name":"Alice"},{"identicon":"<empty>","seed_name":"BobGhost"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_all_ids() {
        let dbname = "for_tests/print_all_ids";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = print_all_identities(dbname).unwrap()
            .replace(ALICE_SR_ROOT, r#"<alice_sr25519_root>"#)
            .replace(ALICE_SR_ALICE, r#"<alice_sr25519_//Alice>"#)
            .replace(ALICE_SR_KUSAMA, r#"<alice_sr25519_//kusama>"#)
            .replace(ALICE_SR_POLKADOT, r#"<alice_sr25519_//polkadot>"#)
            .replace(ALICE_SR_WESTEND, r#"<alice_sr25519_//westend>"#);
        let expected_print = r#"[{"seed_name":"Alice","address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend"},{"seed_name":"Alice","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"<alice_sr25519_root>","has_pwd":false,"path":""},{"seed_name":"Alice","address_key":"0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","identicon":"<alice_sr25519_//kusama>","has_pwd":false,"path":"//kusama"},{"seed_name":"Alice","address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice"},{"seed_name":"Alice","address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","identicon":"<alice_sr25519_//polkadot>","has_pwd":false,"path":"//polkadot"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_ids_seed_name_network() {
        let dbname = "for_tests/print_ids_seed_name_network";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = print_identities_for_seed_name_and_network(dbname, "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), None, Vec::new()).unwrap()
            .replace(ALICE_SR_ROOT, r#"<alice_sr25519_root>"#)
            .replace(ALICE_SR_ALICE, r#"<alice_sr25519_//Alice>"#)
            .replace(ALICE_SR_WESTEND, r#"<alice_sr25519_//westend>"#);
        let expected_print = r#""root":{"seed_name":"Alice","identicon":"<alice_sr25519_root>","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","swiped":false,"multiselect":false},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"<alice_sr25519_//westend>","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"<alice_sr25519_//Alice>","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"}"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn show_all_networks_flag_westend() {
        let dbname = "for_tests/show_all_networks_flag_westend";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = show_all_networks_with_flag(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r#""networks":[{"key":"018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3","title":"Polkadot","logo":"polkadot","order":0,"selected":false},{"key":"0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe","title":"Kusama","logo":"kusama","order":1,"selected":false},{"key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","title":"Westend","logo":"westend","order":2,"selected":true}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn show_all_networks_no_flag() {
        let dbname = "for_tests/show_all_networks_no_flag";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = show_all_networks(dbname).unwrap();
        let expected_print = r#""networks":[{"key":"018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3","title":"Polkadot","logo":"polkadot","order":0},{"key":"0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe","title":"Kusama","logo":"kusama","order":1},{"key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","title":"Westend","logo":"westend","order":2}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn first_standard_network() {
        let dbname = "for_tests/first_standard_network";
        populate_cold (dbname, Verifier(None)).unwrap();
        let specs = first_network(dbname).unwrap();
        assert!(specs.name == "polkadot", "\nReceived: \n{:?}", specs);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn export_alice_westend() {
        let dbname = "for_tests/export_alice_westend";
        populate_cold (dbname, Verifier(None)).unwrap();
        let public: [u8;32] = hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap().try_into().unwrap();
        let print = export_key (dbname, &MultiSigner::Sr25519(Public::from_raw(public)), "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap()
            .replace(ALICE_SR_ROOT, r#"<alice_sr25519_root>"#)
            .replace(ALICE_WESTEND_ROOT_QR, r#"<alice_westend_root_qr>"#);
        let expected_print = r#""qr":"<alice_westend_root_qr>","pubkey":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"<alice_sr25519_root>","seed_name":"Alice","path":"","network_title":"Westend","network_logo":"westend""#;
        assert!(print == expected_print, "\nReceived: \n{:?}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn backup_prep_alice() {
        let dbname = "for_tests/backup_prep_alice";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = backup_prep(dbname, "Alice").unwrap();
        let expected_print = r#""seed_name":"Alice","derivations":[{"network_title":"Polkadot","network_logo":"polkadot","network_order":0,"id_set":[{"path":"","has_pwd":false},{"path":"//polkadot","has_pwd":false}]},{"network_title":"Kusama","network_logo":"kusama","network_order":1,"id_set":[{"path":"","has_pwd":false},{"path":"//kusama","has_pwd":false}]},{"network_title":"Westend","network_logo":"westend","network_order":2,"id_set":[{"path":"//westend","has_pwd":false},{"path":"","has_pwd":false},{"path":"//Alice","has_pwd":false}]}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn derive_prep_alice() {
        let dbname = "for_tests/derive_prep_alice";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = derive_prep(dbname, "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), None, "//secret//derive").unwrap();
        let expected_print = r#""seed_name":"Alice","network_title":"Westend","network_logo":"westend","suggested_derivation":"//secret//derive""#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn derive_prep_alice_collided() {
        let dbname = "for_tests/derive_prep_alice_collided";
        populate_cold (dbname, Verifier(None)).unwrap();
        let network_specs_key = NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519);
        let mut collision = None;
        for (multisigner, address_details) in addresses_set_seed_name_network(dbname, "Alice", &network_specs_key).unwrap().into_iter() {
            if address_details.path == "//Alice" {
                collision = Some((multisigner, address_details));
                break;
            }
        }
        let collision = match collision {
            Some(a) => a,
            None => panic!("Did not create address?"),
        };
        let print = derive_prep(dbname, "Alice", &network_specs_key, Some(collision), "//Alice").unwrap()
            .replace(ALICE_SR_ALICE, r#"<alice_sr25519_//Alice>"#);
        let expected_print = r#""seed_name":"Alice","network_title":"Westend","network_logo":"westend","suggested_derivation":"//Alice","collision":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","path":"//Alice","has_pwd":false,"identicon":"<alice_sr25519_//Alice>","seed_name":"Alice"}"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn derive_prep_alice_collided_with_password() {
        let dbname = "for_tests/derive_prep_alice_collided_with_password";
        populate_cold (dbname, Verifier(None)).unwrap();
        let network_specs_key = NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519);
        try_create_address ("Alice", ALICE_SEED_PHRASE, "//secret///abracadabra", &network_specs_key, dbname).unwrap();
        let mut collision = None;
        for (multisigner, address_details) in addresses_set_seed_name_network(dbname, "Alice", &network_specs_key).unwrap().into_iter() {
            if address_details.path == "//secret" {
                collision = Some((multisigner, address_details));
                break;
            }
        }
        let collision = match collision {
            Some(a) => a,
            None => panic!("Did not create address?"),
        };
        let print = derive_prep(dbname, "Alice", &network_specs_key, Some(collision), "//secret///abracadabra").unwrap()
            .replace(ALICE_SR_SECRET_ABRACADABRA, r#"<alice_sr25519_//secret///abracadabra>"#);
        let expected_print = r#""seed_name":"Alice","network_title":"Westend","network_logo":"westend","suggested_derivation":"//secret///abracadabra","collision":{"base58":"5EkMjdgyuHqnWA9oWXUoFRaMwMUgMJ1ik9KtMpPNuTuZTi2t","path":"//secret","has_pwd":true,"identicon":"<alice_sr25519_//secret///abracadabra>","seed_name":"Alice"}"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn westend_network_details() {
        let dbname = "for_tests/westend_network_details";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = network_details_by_key(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap()
            .replace(EMPTY_PNG, r#"<empty>"#)
            .replace(WESTEND_9000, r#"<meta_pic_westend9000>"#)
            .replace(WESTEND_9010, r#"<meta_pic_westend9010>"#);
        let expected_print = r##""base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"public_key":"","identicon":"<empty>","encryption":"none"}},"meta":[{"spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce","meta_id_pic":"<meta_pic_westend9000>"},{"spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf","meta_id_pic":"<meta_pic_westend9010>"}]"##;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn westend_9010_metadata_details() {
        let dbname = "for_tests/westend_9010_metadata_details";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = metadata_details(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), 9010).unwrap()
            .replace(WESTEND_9010, r#"<meta_pic_westend9010>"#);
        let expected_print = r#""name":"westend","version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf","meta_id_pic":"<meta_pic_westend9010>","networks":[{"title":"Westend","logo":"westend","order":2,"current_on_screen":true}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn types_status_and_history() {
        let dbname = "for_tests/types_status_and_history";
        populate_cold (dbname, Verifier(None)).unwrap();
        
        let print = show_types_status(dbname).unwrap()
            .replace(TYPES_KNOWN, r#"<types_known>"#);
        let expected_print = r#""types_on_file":true,"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"<types_known>""#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        
        remove_types_info(dbname).unwrap();
        let print = show_types_status(dbname).unwrap();
        let expected_print = r#""types_on_file":false"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        
        let history_printed = print_history(dbname).unwrap()
            .replace(EMPTY_PNG, r#"<empty>"#)
            .replace(TYPES_KNOWN, r#"<types_known>"#);
        let expected_element = r#"{"event":"types_removed","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"<types_known>","verifier":{"public_key":"","identicon":"<empty>","encryption":"none"}}}"#;
        assert!(history_printed.contains(expected_element), "\nReceived history: \n{}", history_printed);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn path_is_known() {
        let dbname = "for_tests/path_is_known";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = dynamic_path_check (dbname, "Alice", "//Alice", "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
            .replace(ALICE_SR_ALICE, r#"<alice_sr25519_//Alice>"#);
        let expected_print = r#""derivation_check":{"button_good":false,"collision":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","path":"//Alice","has_pwd":false,"identicon":"<alice_sr25519_//Alice>","seed_name":"Alice"}}"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn path_is_unknown() {
        let dbname = "for_tests/path_is_unknown";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = dynamic_path_check (dbname, "Alice", "//secret", "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e");
        let expected_print = r#""derivation_check":{"button_good":true,"where_to":"pin"}"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn path_is_unknown_passworded() {
        let dbname = "for_tests/path_is_unknown_passworded";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = dynamic_path_check (dbname, "Alice", "//secret///abracadabra", "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e");
        let expected_print = r#""derivation_check":{"button_good":true,"where_to":"pwd"}"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn word_search_1() {
        let word_part = "dri";
        let out = guess(word_part);
        let out_expected = vec!["drift".to_string(),"drill".to_string(),"drink".to_string(),"drip".to_string(),"drive".to_string()];
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }

    #[test]
    fn word_search_2() {
        let word_part = "umbra";
        let out = guess(word_part);
        assert!(out.is_empty(), "Found different word set:\n{:?}", out);
    }

    #[test]
    fn word_search_3() {
        let word_part = "котик";
        let out = guess(word_part);
        assert!(out.is_empty(), "Found different word set:\n{:?}", out);
    }

    #[test]
    fn word_search_4() {
        let word_part = "";
        let out = guess(word_part);
        let out_expected = vec!["abandon".to_string(),"ability".to_string(),"able".to_string(),"about".to_string(),"above".to_string(),"absent".to_string(),"absorb".to_string(),"abstract".to_string()];
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }

    #[test]
    fn word_search_5() {
        let word_part = " ";
        let out = guess(word_part);
        assert!(out.is_empty(), "Found different word set:\n{:?}", out);
    }

    #[test]
    fn word_search_6() {
        let word_part = "s";
        let out = guess(word_part);
        let out_expected = vec!["sad".to_string(),"saddle".to_string(),"sadness".to_string(),"safe".to_string(),"sail".to_string(),"salad".to_string(),"salmon".to_string(),"salon".to_string()];
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }

    #[test]
    fn word_search_7() {
        let word_part = "se";
        let out = guess(word_part);
        let out_expected = vec!["sea".to_string(),"search".to_string(),"season".to_string(),"seat".to_string(),"second".to_string(),"secret".to_string(),"section".to_string(),"security".to_string()];
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }

    #[test]
    fn word_search_8() {
        let word_part = "sen";
        let out = guess(word_part);
        let out_expected = vec!["senior".to_string(),"sense".to_string(),"sentence".to_string()];
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }

    #[test]
    fn alice_recalls_seed_phrase_1() {
        let mut seed_draft = SeedDraft::initiate();
        seed_draft.added("bottom", None);
        seed_draft.added("lake", None);
        // oops, wrong place
        seed_draft.added("drive", Some(1));
        seed_draft.added("obey", Some(2));
        let print = seed_draft.print();
        let expected_print = r#"[{"order":0,"content":"bottom"},{"order":1,"content":"drive"},{"order":2,"content":"obey"},{"order":3,"content":"lake"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        // adding invalid word - should be blocked through UI, expect no changes
        seed_draft.added("занавеска", None);
        let print = seed_draft.print();
        let expected_print = r#"[{"order":0,"content":"bottom"},{"order":1,"content":"drive"},{"order":2,"content":"obey"},{"order":3,"content":"lake"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        // removing invalid word - should be blocked through UI, expect no changes
        seed_draft.remove(5);
        let print = seed_draft.print();
        let expected_print = r#"[{"order":0,"content":"bottom"},{"order":1,"content":"drive"},{"order":2,"content":"obey"},{"order":3,"content":"lake"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        // removing word
        seed_draft.remove(1);
        let print = seed_draft.print();
        let expected_print = r#"[{"order":0,"content":"bottom"},{"order":1,"content":"obey"},{"order":2,"content":"lake"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
    }
    
    #[test]
    fn alice_recalls_seed_phrase_2() {
        let mut seed_draft = SeedDraft::initiate();
        seed_draft.added("fit", None);
        let print = seed_draft.print();
        let expected_print = r#"[{"order":0,"content":"fit"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
    }
    
    #[test]
    fn alice_recalls_seed_phrase_3() {
        let mut seed_draft = SeedDraft::initiate();
        seed_draft.added("obe", None);
        let print = seed_draft.print();
        let expected_print = r#"[{"order":0,"content":"obey"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
    }

}



