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
pub fn print_all_seed_names_with_identicons (database_name: &str, names_phone_knows: &Vec<String>) -> Result<String, ErrorSigner> {
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
        else {if let None = data_set.get(&address_details.seed_name) {data_set.insert(address_details.seed_name.to_string(), Vec::new());}}
    }
    for x in names_phone_knows.iter() {if let None = data_set.get(x) {data_set.insert(x.to_string(), Vec::new());}}
    let mut print_set: Vec<(String, String)> = Vec::new();
    for (seed_name, multisigner_set) in data_set.into_iter() {
        let identicon_string = preferred_multisigner_identicon(&multisigner_set);
        print_set.push((identicon_string, seed_name))
    }
    print_set.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(export_complex_vector(&print_set, |(identicon_string, seed_name)| format!("\"identicon\":\"{}\",\"seed_name\":\"{}\"", identicon_string, seed_name)))
}

fn preferred_multisigner_identicon(multisigner_set: &Vec<MultiSigner>) -> String {
    if multisigner_set.len() == 0 {hex::encode(EMPTY_PNG)}
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
        else {
            if let Some(a) = got_ed25519 {hex::encode(make_identicon_from_multisigner(&a))}
            else {
                if let Some(a) = got_ecdsa {hex::encode(make_identicon_from_multisigner(&a))}
                else {hex::encode(EMPTY_PNG)}
            }
        }
    }
}

/// Function to print all identities (seed names AND derication paths) with identicons
/// Gets used only on the Signer side, interacts with the user interface.
pub fn print_all_identities (database_name: &str) -> Result<String, ErrorSigner> {
    Ok(export_complex_vector(&get_all_addresses(database_name)?, |(multisigner, address_details)| {
        let address_key = AddressKey::from_multisigner(&multisigner); // to click
        let public_key = multisigner_to_public(&multisigner); // to display
        let hex_identicon = hex::encode(make_identicon_from_multisigner(&multisigner));
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
            if let Some(ref swiped_multisigner) = swiped_key {
                if swiped_multisigner == &multisigner {true}
                else {false}
            }
            else {false}
        };
        let multiselect = {
            if multiselect.contains(&multisigner) {true}
            else {false}
        };
        if address_details.is_root() {
            if let Some(_) = root_id {return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys{seed_name: seed_name.to_string(), encryption: network_specs.encryption.to_owned()}))}
            root_id = Some(format!("\"seed_name\":\"{}\",\"identicon\":\"{}\",\"address_key\":\"{}\",\"base58\":\"{}\",\"swiped\":{},\"multiselect\":{}", seed_name, hex::encode(identicon), hex::encode(address_key.key()), base58, swiped, multiselect));
        }
        else {other_id.push((multisigner, address_details, identicon, swiped, multiselect))}
    }
    let root_print = match root_id {
        Some(a) => a,
        None => format!("\"seed_name\":\"{}\",\"identicon\":\"{}\",\"address_key\":\"\",\"base58\":\"\",\"swiped\":false,\"multiselect\":false", hex::encode(EMPTY_PNG), seed_name),
    };
    let other_print = export_complex_vector(&other_id, |(multisigner, address_details, identicon, swiped, multiselect)| format!("\"address_key\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":{},\"path\":\"{}\",\"swiped\":{},\"multiselect\":{}", hex::encode(AddressKey::from_multisigner(&multisigner).key()), print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix)), hex::encode(identicon), address_details.has_pwd, address_details.path, swiped, multiselect));
    
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
            let network_specs_key_current = NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption);
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
            let network_specs_key_current = NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption);
            format!("\"key\":\"{}\",\"title\":\"{}\",\"logo\":\"{}\",\"order\":{}", hex::encode(network_specs_key_current.key()), a.title, a.logo, a.order)
        }
    )))
}

/// Function to sort networks by the order and get the network specs for the first network on the list.
/// If no networks in the system, throws error
pub fn first_network (database_name: &str) -> Result<NetworkSpecs, ErrorSigner> {
    let mut networks = get_all_networks::<Signer>(database_name)?;
    if networks.len() == 0 {return Err(ErrorSigner::NoNetworksAvailable)}
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(networks.remove(0))
}

/// Function to prepare the export key screen.
/// Contains among else the QR code with identity information, in format 
/// `substrate:{public_key as as_base58}:0x{network_key}`,
/// this string is transformed into bytes, then into png qr code, then qr code
/// content is hexed so that it could be transferred into app.
pub fn export_key (database_name: &str, multisigner: &MultiSigner, expected_seed_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, &network_specs_key)?;
    let address_key = AddressKey::from_multisigner(&multisigner);
    let address_details = get_address_details(database_name, &address_key)?;
    if address_details.seed_name != expected_seed_name {return Err(ErrorSigner::Interface(InterfaceSigner::SeedNameNotMatching{address_key, expected_seed_name: expected_seed_name.to_string(), real_seed_name: address_details.seed_name}))}
    let address_base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
    let public_key = multisigner_to_public(&multisigner);
    let identicon = make_identicon_from_multisigner(&multisigner);
    let qr_prep = {
        if address_details.network_id.contains(&network_specs_key) {
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
    if networks.len() == 0 {return Err(ErrorSigner::NoNetworksAvailable)}
    let mut export: Vec<(NetworkSpecs, Vec<AddressDetails>)> = Vec::new();
    for x in networks.into_iter() {
        let id_set = addresses_set_seed_name_network (database_name, seed_name, &NetworkSpecsKey::from_parts(&x.genesis_hash.to_vec(), &x.encryption))?;
        if id_set.len() != 0 {export.push((x, id_set.into_iter().map(|(_, a)| a).collect()))}
    }
    export.sort_by(|(a, _), (b, _)| a.order.cmp(&b.order));
    Ok(format!("\"seed_name\":\"{}\",\"derivations\":{}", seed_name, export_complex_vector(&export, |(specs, id_set)| format!("\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_order\":{},\"id_set\":{}", specs.title, specs.logo, specs.order, export_complex_vector(&id_set, |a| format!("\"path\":\"{}\",\"has_pwd\":{}", a.path, a.has_pwd))))))
}

/// Function to prepare key derivation screen.
/// Gets seed name, network specs key and suggested derivation
pub fn derive_prep (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey, collision: Option<(MultiSigner, AddressDetails)>, suggest: &str) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    match collision {
        Some((multisigner, address_details)) => {
            let address_base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
            let hex_identicon = match make_identicon_from_multisigner(&multisigner) {
                Ok(a) => hex::encode(a),
                Err(_) => String::new(),
            };
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
                            let hex_identicon = match make_identicon_from_multisigner(&multisigner) {
                                Ok(a) => hex::encode(a),
                                Err(_) => String::new(),
                            };
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
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash.to_vec());
    let general_verifier = get_general_verifier(&database_name)?;
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, &database_name)?;
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
        format!("\"title\":\"{}\",\"logo\":\"{}\",\"order\":{},\"current_on_screen\":{}", a.title, a.logo, a.order, &NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption) == network_specs_key)
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
        .set_transaction(make_batch_clear_tree::<Signer>(&database_name, TRANSACTION)?) // clear transaction
        .apply::<Signer>(&database_name)
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
    pub fn new() -> Self {
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
                    else {
                        if !guess(word).is_empty() {self.user_input = String::from(word)}
                    }
                }
                else {
                    if !guess(user_text).is_empty() {self.user_input = String::from(user_text)}
                }
            }
        }
        else {self.user_input.clear()}
    }
    pub fn added(&mut self, word: &str, position: Option<u32>) -> bool {
        if self.saved.len() < BIP_CAP {
            let guesses = guess(word);
            let definitive_guess = {
                if guesses.len() == 1 {Some(guesses[0])}
                else {
                    if guesses.contains(&word) {Some(word)}
                    else {None}
                }
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
        if self.saved.len() > 0 {
            self.saved.remove(self.saved.len()-1);
        }
    }
    pub fn print(&self) -> String {
        let mut out = String::with_capacity(SAFE_RESERVE);
        out.push_str("[");
        for (i,x) in self.saved.iter().enumerate() {
            if i>0 {out.push_str(",")}
            out.push_str(&format!("{{\"order\":{},\"content\":\"", i));
            out.push_str(x.word());
            out.push_str("\"}");
        }
        out.push_str("]");
        out
    }
    /// Combines all draft elements into seed phrase proposal,
    /// and checks its validity.
    /// If valid, outputs secret seed phrase.
    pub fn try_finalize(&self) -> Option<String> {
        let mut seed_phrase_proposal = String::with_capacity((WORD_LENGTH+1)*BIP_CAP);
        for (i, x) in self.saved.iter().enumerate() {
            if i>0 {seed_phrase_proposal.push_str(" ");}
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
        let horrible_print = print_all_seed_names_with_identicons(dbname, &vec![String::from("Alice")]).unwrap();
        let expected_print = r#"[{"identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a9b49444154789c75570b54556516feceebbeef85ab20be125f9924f980184d535b6a2e534bcb64d4091341b324a734731ad668594bc7c4345f4c91e84c91955966d358a3948491a60e844ae10b1015b8bceffbdef39afd1fd335d363b3f6e272f8cffffd7bffdffef6be9caeebf80d13c85572c354554d9565791af9785dd77bdbaca2ca814328a2089aa6b74a925446fe2f51144bd872726622b942fe0bfb2de05b2f44a3d1b991486499a228a30800a2c84396559457791095350cbd230eee180b2251153ccf4110841f4d265381d96cdec9715c98b6600168e4ff07f46bc006a8a2aa77070381dd1461b2466b5874b4914a9b736fecade44e5636e91c07f4e9e9e29e5d98a2bb1c265d51349edea5a7ec80a2c76ab52ea5437c78e39961b7c07e0e6c805284d9c160b08045689204c56a35b375822872283fdb888d05a7e1b049e008a6bd238cf98fdc8907eeeb87404866516bf49eaa2a9a447b81c05f235f411f99b10318806c43fa6598011a0a855e21cfa5cf90245169eff08b070e9d84a7b913e3462761f0ed7d919b57ca00282a1e9dbe089ec94cc1e03e6ed45ce88428f148e86d87dd25699a4abbd38129ea4f1c0ec74cda9299017e13582457c214a9dfef2fe0799eb2ab4312057ee54b6fe3eb6f7f80d562027b96ff6a36ae366bf8e8f3f3f43a70f7b0043c363d0915651e4442373865b58b1836ba1b0489d7698da2e99a64b55877d8edb61cfa37bb7395011b1f883c7707fdfe93224fe45135cd6133f3e72e5ec392956fc26292c00b3cdadafd983e39056bff94819aba66229e8a3b6e8f43f599465c38d30e93856d05c8111583867745e20017226183740ae55f94ccd68516b379372d1119301d0a08f9fd671a7cdee4fd17aa94f670581c9ed01d137bf4c5c2e5f9b872b5054e8715cd2d9d7875cd02c4c56958bf632b3150c69c19b3307ddc349415d712a379232b2c9ba326f44479b5075f1dbf4a3ce1b5e913faf34903ba86cd167b5f5ad764004758c90483ef6eabf84e39dd745d74984cf08623589a360a31ad2a36bef9193abd414c1a7b171e993e0253e6cf4183c7431c90a0a92a0ebcb5077de39270f9875623337d0739d1a9cad8907f1282c051f969e81e6f975f78324d72c73af22c16eb4a039852fc6d6bc03feae513a5aaaa6b024bb73f1ac5f0b804ac1c3f098d4d1e789a5a317468128e96956266f6027475bb5979a1b1d98375cfe76259d6625caea9232001897d7ae2fd4f2bf1d1171711eb341b590804657d79562a3764505cab64b20de6a84e53bd3edf2989c0b6969fc0c9c66b70881471348c65a3c6c256e3c10bcfbd8c7030847113efc5d2554b30635106ce5fbe04b3c94cf7c7a378ef7e7c59760c057bdf06dd2572739661e488fbb17af3513a08718622eed1cd8edca523558b5910ac56db628e4a677520187cc924086a4b30281caabf8836021de28ec73deeeef8fdd4f9a8bd74054e97130dd71bb06dd726248f4ec69a8d1ba86e4378862275c7c462e2dc5970daed0630a502478ade832f188b92efea60318b9834ba370626bab57044e1cd66d3679cd7eb2d566479022dd75c760bdfdcd889bafa160c1fd61fb575759839612e6c362b483dd0e269c5dcccd958fffa5aba9e00a84ce0703a5150f40f3cbb7635dd633ca51546fa77e76d41fa433371e2d43974713bd0bf4f02d57c88d4ceb016aeb3b3b33a1a9507399d56fdd0a1726ef3e67f22e00f23b15f3cb66ccec2ababff8abd7b3e04d5204c6613fe7ea000c352ef42301024e5e221d29d76783b31f5f179b85073d98878e48814eccb2fc4866d9fa2f47815cc6609f31eb9178b322622188cd2f590fc76747454e9ba962447552d3bfb6f7c43431b5c2e1b1a1b3b9091311e394f4fc6f64dbbd0d6da8629d3efc7c87bd3100e85299b4c80284d14a2d56cc6f99acbf8e4c8e7208dc792b97fc0c9f27afc795d1112e263a9de15e3406fbeb604fd13bb211291750398242829108868590b77f23e5f08569b192dcd5e4c9d4662f1ca5c041a283a45071f2b21aac9b49c107f32c6586a0890a3328e977c47ed53c3b48727a2e8e36fb071db0103981d26188c60c7866c240fbe0dc1509401775613b38d5417167ec9e5e77f41db01b13136bcf67a267af13634545c37d223d94d482092885656bfc40a3256b7a47a583427076525c78de733d2a7e1c54d6bb02cb710d517af13d980699352b17ac52c237a962d46ae3202be87fed0cc24313f9ebb8e5a2aa1a123fa2221ce81dac3970d508e84400d2b700e70c39d4c24a2cec5d217436cdf5ff409563cf102e213e2580ad0ec69c5ae0f7660e49851387ce434e2baba909232c0b816ea5c6c3f1f47ed2f2f180aad2092a85159168aabfe83abed2d48eb7f0746f41888fa921ae88a069e3a91426dcf3d241e8ebe31f05e6883425add23b9178e961ec3e2f41c7489771b07f279fd282212f64a198c6fea6b60934c48ebd2032e935953349527c5fb9a09c8046276b1dd62d5d77ffc2eb7f7d857b0996ea8cd86058b91e6e887fad3572050f315634ce896da13ad151e849be9de298502a53d614c6ffc65d52bf8f7c1c39015158b9e9a8f8ce71761cdd1c3f045a3203544727c37ac481dcd1aa560b3d956b1ae2dd0a471b6c5db39382b3f4f8b2a0a6f22b2b407fc98306438362e7c0afef600e450148e0417daaeb5a0f9db6be025ea4404ccd2df65680245de0395a7ced0a573189e320cfbbe2fc7bef367e1b6588d140764597f2e753497d4352e24d96c77717402a27764792010d89453b85daeaaaf95626c0e3476b461f943e9782029196bb7bc8886a66b983d631e664d9a8ddae20b34413066d1bd530bbc6d5c1f9cb9d2807def9f806412f158fa3d507a9ab181743dc6624194586d952425376dacd82336f61d8bd59a610053fbb5d0145077ea5275b78d073fd0bcc1209fd26f2056cd9c8385397348044ae072c6501904b07be77b183d600c1abfaf074f3fce44177c76e089456fc0ef0f5139e9888f77a170d79328edb88692ba5aa6d7facc018331b6571f8ea21d228962150336a60f8afa512512dd172225f18582fc6df1ddb9b33f56e2e1c7a79264da8889025a5a9b913e731e5e5f9f8feba4db6cda4c4cec8da2778a919777d00064d6d2e2c39a176763c68369a8f7d0802009b25d9024de6c5a67b35a7369c98d41808c2e0c2a4b7794d24ead8d0d2192a22a989d391d95e72ae074bad0d1d98e3d5b8be0ec33105b0ebe6fa470ce98fbf0bbee77222b7ba7512acc6d2440056f3d815ebdbb4293b5a80eddc48be21197d3793fe130e36e02134d0cd369e63a40d1cfa0cf8acd6ae32bab2af8ed6f6d86a7a50993c74f41faec4c646c5d077f380c3391d0471d6ad7d32bd05c1dc0de0f8e19b3d9ac474761ecd8243d1c8e92deb1d14d38ef72b986915684695f9a4da1dd0466c6c08d3fa8b6b787c3e1a5ec7f042ef38220d261381789c5b1ca0afc71cf0ec4120169c446b3b703cf4c9b850593a7c2ebf3b2a6a19340a824912c8b1cd5ec119a301ffc09943d3326c2ff0566760b9c8032a957ef2439b4d0fe941b9ebeb2800b2b32b7ac70075775b51602dd7bacdd81ed594fa35f7c772a4599dee5046a3a442801168b651dcdd4ec4e9909e40628b39f0333a3fd8d74d0356b097480e7c833e97357b6d642e2d2d0de8a8f4e94222ccb98323c0d43136998a7d45354cc432693693f81ae27706a4086b1fd6e88fb4ff66bc037cd603b3953b138fa0ef530a9dc0cf29194ce385237f6bd061159e6e8003e2a91724aeb670c94c6a14bf41a33b6874afe0b90ff0298033ab9521b07fe0000000049454e44ae426082","seed_name":"Alice"}]"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_seed_names_with_orphan() {
        let dbname = "for_tests/print_seed_names_with_orphan";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_all_seed_names_with_identicons(dbname, &vec![String::from("Alice"), String::from("BobGhost")]).unwrap();
        let expected_print = r#"[{"identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a9b49444154789c75570b54556516feceebbeef85ab20be125f9924f980184d535b6a2e534bcb64d4091341b324a734731ad668594bc7c4345f4c91e84c91955966d358a3948491a60e844ae10b1015b8bceffbdef39afd1fd335d363b3f6e272f8cffffd7bffdffef6be9caeebf80d13c85572c354554d9565791af9785dd77bdbaca2ca814328a2089aa6b74a925446fe2f51144bd872726622b942fe0bfb2de05b2f44a3d1b991486499a228a30800a2c84396559457791095350cbd230eee180b2251153ccf4110841f4d265381d96cdec9715c98b6600168e4ff07f46bc006a8a2aa77070381dd1461b2466b5874b4914a9b736fecade44e5636e91c07f4e9e9e29e5d98a2bb1c265d51349edea5a7ec80a2c76ab52ea5437c78e39961b7c07e0e6c805284d9c160b08045689204c56a35b375822872283fdb888d05a7e1b049e008a6bd238cf98fdc8907eeeb87404866516bf49eaa2a9a447b81c05f235f411f99b10318806c43fa6598011a0a855e21cfa5cf90245169eff08b070e9d84a7b913e3462761f0ed7d919b57ca00282a1e9dbe089ec94cc1e03e6ed45ce88428f148e86d87dd25699a4abbd38129ea4f1c0ec74cda9299017e13582457c214a9dfef2fe0799eb2ab4312057ee54b6fe3eb6f7f80d562027b96ff6a36ae366bf8e8f3f3f43a70f7b0043c363d0915651e4442373865b58b1836ba1b0489d7698da2e99a64b55877d8edb61cfa37bb7395011b1f883c7707fdfe93224fe45135cd6133f3e72e5ec392956fc26292c00b3cdadafd983e39056bff94819aba66229e8a3b6e8f43f599465c38d30e93856d05c8111583867745e20017226183740ae55f94ccd68516b379372d1119301d0a08f9fd671a7cdee4fd17aa94f670581c9ed01d137bf4c5c2e5f9b872b5054e8715cd2d9d7875cd02c4c56958bf632b3150c69c19b3307ddc349415d712a379232b2c9ba326f44479b5075f1dbf4a3ce1b5e913faf34903ba86cd167b5f5ad764004758c90483ef6eabf84e39dd745d74984cf08623589a360a31ad2a36bef9193abd414c1a7b171e993e0253e6cf4183c7431c90a0a92a0ebcb5077de39270f9875623337d0739d1a9cad8907f1282c051f969e81e6f975f78324d72c73af22c16eb4a039852fc6d6bc03feae513a5aaaa6b024bb73f1ac5f0b804ac1c3f098d4d1e789a5a317468128e96956266f6027475bb5979a1b1d98375cfe76259d6625caea9232001897d7ae2fd4f2bf1d1171711eb341b590804657d79562a3764505cab64b20de6a84e53bd3edf2989c0b6969fc0c9c66b70881471348c65a3c6c256e3c10bcfbd8c7030847113efc5d2554b30635106ce5fbe04b3c94cf7c7a378ef7e7c59760c057bdf06dd2572739661e488fbb17af3513a08718622eed1cd8edca523558b5910ac56db628e4a677520187cc924086a4b30281caabf8836021de28ec73deeeef8fdd4f9a8bd74054e97130dd71bb06dd726248f4ec69a8d1ba86e4378862275c7c462e2dc5970daed0630a502478ade832f188b92efea60318b9834ba370626bab57044e1cd66d3679cd7eb2d566479022dd75c760bdfdcd889bafa160c1fd61fb575759839612e6c362b483dd0e269c5dcccd958fffa5aba9e00a84ce0703a5150f40f3cbb7635dd633ca51546fa77e76d41fa433371e2d43974713bd0bf4f02d57c88d4ceb016aeb3b3b33a1a9507399d56fdd0a1726ef3e67f22e00f23b15f3cb66ccec2ababff8abd7b3e04d5204c6613fe7ea000c352ef42301024e5e221d29d76783b31f5f179b85073d98878e48814eccb2fc4866d9fa2f47815cc6609f31eb9178b322622188cd2f590fc76747454e9ba962447552d3bfb6f7c43431b5c2e1b1a1b3b9091311e394f4fc6f64dbbd0d6da8629d3efc7c87bd3100e85299b4c80284d14a2d56cc6f99acbf8e4c8e7208dc792b97fc0c9f27afc795d1112e263a9de15e3406fbeb604fd13bb211291750398242829108868590b77f23e5f08569b192dcd5e4c9d4662f1ca5c041a283a45071f2b21aac9b49c107f32c6586a0890a3328e977c47ed53c3b48727a2e8e36fb071db0103981d26188c60c7866c240fbe0dc1509401775613b38d5417167ec9e5e77f41db01b13136bcf67a267af13634545c37d223d94d482092885656bfc40a3256b7a47a583427076525c78de733d2a7e1c54d6bb02cb710d517af13d980699352b17ac52c237a962d46ae3202be87fed0cc24313f9ebb8e5a2aa1a123fa2221ce81dac3970d508e84400d2b700e70c39d4c24a2cec5d217436cdf5ff409563cf102e213e2580ad0ec69c5ae0f7660e49851387ce434e2baba909232c0b816ea5c6c3f1f47ed2f2f180aad2092a85159168aabfe83abed2d48eb7f0746f41888fa921ae88a069e3a91426dcf3d241e8ebe31f05e6883425add23b9178e961ec3e2f41c7489771b07f279fd282212f64a198c6fea6b60934c48ebd2032e935953349527c5fb9a09c8046276b1dd62d5d77ffc2eb7f7d857b0996ea8cd86058b91e6e887fad3572050f315634ce896da13ad151e849be9de298502a53d614c6ffc65d52bf8f7c1c39015158b9e9a8f8ce71761cdd1c3f045a3203544727c37ac481dcd1aa560b3d956b1ae2dd0a471b6c5db39382b3f4f8b2a0a6f22b2b407fc98306438362e7c0afef600e450148e0417daaeb5a0f9db6be025ea4404ccd2df65680245de0395a7ced0a573189e320cfbbe2fc7bef367e1b6588d140764597f2e753497d4352e24d96c77717402a27764792010d89453b85daeaaaf95626c0e3476b461f943e9782029196bb7bc8886a66b983d631e664d9a8ddae20b34413066d1bd530bbc6d5c1f9cb9d2807def9f806412f158fa3d507a9ab181743dc6624194586d952425376dacd82336f61d8bd59a610053fbb5d0145077ea5275b78d073fd0bcc1209fd26f2056cd9c8385397348044ae072c6501904b07be77b183d600c1abfaf074f3fce44177c76e089456fc0ef0f5139e9888f77a170d79328edb88692ba5aa6d7facc018331b6571f8ea21d228962150336a60f8afa512512dd172225f18582fc6df1ddb9b33f56e2e1c7a79264da8889025a5a9b913e731e5e5f9f8feba4db6cda4c4cec8da2778a919777d00064d6d2e2c39a176763c68369a8f7d0802009b25d9024de6c5a67b35a7369c98d41808c2e0c2a4b7794d24ead8d0d2192a22a989d391d95e72ae074bad0d1d98e3d5b8be0ec33105b0ebe6fa470ce98fbf0bbee77222b7ba7512acc6d2440056f3d815ebdbb4293b5a80eddc48be21197d3793fe130e36e02134d0cd369e63a40d1cfa0cf8acd6ae32bab2af8ed6f6d86a7a50993c74f41faec4c646c5d077f380c3391d0471d6ad7d32bd05c1dc0de0f8e19b3d9ac474761ecd8243d1c8e92deb1d14d38ef72b986915684695f9a4da1dd0466c6c08d3fa8b6b787c3e1a5ec7f042ef38220d261381789c5b1ca0afc71cf0ec4120169c446b3b703cf4c9b850593a7c2ebf3b2a6a19340a824912c8b1cd5ec119a301ffc09943d3326c2ff0566760b9c8032a957ef2439b4d0fe941b9ebeb2800b2b32b7ac70075775b51602dd7bacdd81ed594fa35f7c772a4599dee5046a3a442801168b651dcdd4ec4e9909e40628b39f0333a3fd8d74d0356b097480e7c833e97357b6d642e2d2d0de8a8f4e94222ccb98323c0d43136998a7d45354cc432693693f81ae27706a4086b1fd6e88fb4ff66bc037cd603b3953b138fa0ef530a9dc0cf29194ce385237f6bd061159e6e8003e2a91724aeb670c94c6a14bf41a33b6874afe0b90ff0298033ab9521b07fe0000000049454e44ae426082","seed_name":"Alice"},{"identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea20000002e49444154789cedcd410100200c0021ed1f7ab6381f8302dc99393f8833e28c3823ce8833e28c3823ce8833fbe20724cf59c50a861d5c0000000049454e44ae426082","seed_name":"BobGhost"}]"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_all_ids() {
        let dbname = "for_tests/print_all_ids";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = print_all_identities(dbname).unwrap();
        let expected_print = r#"[{"seed_name":"Alice","address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a7149444154789c7557097415d519feee9de5ed592481200261070544402522229155100a226a018fa0d50a1605a4ad4bb5b41a8f47a02a05ec091ea46244d10222b56c22d4a29168544021248201819085bcf766e6cd7efbcf43ad5afde7dc7366eef27fff7eff614208fc0c49343c1a59f23c6f90e338e31dc71d2e847751583ebf667b907cc19b1545dea728ca3f6559de43d3d9352299864be3ffe8e780bf3b60dbf6ad9665cd735d7748b0576280e3331c6d51e0fa40d77c1739aa0f9bde39639024e9b0aaaae5a150682563cc24168102b48a1f00fd14701694341cacebfa1ad771fa0667183d60cca3c136d744d9e12655309a6f17f3d8b44b7411937de193ea7496051cb92c9f8d4622734988d7e9930e672958cad28f81b3a0a4e19d866194076b4c925ca1a801332944b8271a7d541c8a2322d31acda62c8e71dd0c5cded987ee2ba4357ce6ba1e5c47a133884422cb682ca4d780020182e91f0067413399cce3341e0e7640965d66e8b27af013302d0d74e98ad3175e8c973e0a81ac0a89f4d32d604a7f07fd9c63f00e1e025314d817f783dfa6d087e304dc25d27a733c1eff05710c884e427c0b9c05b54ceb4e5dd3cb19677ed62b0ae7b12daf43f9b2068218065bed2937e35dd11b1f1e27d5c0d08bf88f6f7f06f1d7d79d178e36f9f91740bb7126841aa113be4b2e5022e1f08a683c7a2f710d7cee05c0d917d7f106a7ade47ea6d2970d9f9cc6f9578d886f5847021028a9c80d9db4190067c2f5686ab6e0b80ceddaa9502aab107e771bfc58820e8bec3e63d4443883fb82e91982e204eecb319e981d0aab6b68931c00077b91b65307aca37e5fe3b1b0eb9f66b23ac643f43e1df18abf83b59e830885c0351d9909e361e65c01790ff9d8637006925c055f22bebe02be2c83f9642a0a3b7dd60c18ffea0073358565047e7491cd95e19e99c3f38ab9cc1ab2c0b665dfaa39e90aed96986b6d95659e4be64a02f1953ea2371c45ecad7720db3af41e7d60f51f01f1521c224dae225b319f049869433e5789c4fe4ac294a05f53824cf340a4c6ca2415eda3a492bafb4ece0e4d89b68d2c8984228b98a0836933f5be7dca1b92bc36ee09875295cced3733283738285aef20d9e0c1302cb42f8ec13c4c795c11058b92c158602a0e363283c4081f67ebc9ac8ca3b063084d4f30188bc3e045e72de0b73091b3d160a1117e7382e5f666aee30e4aa552552c0ca4a645606e95c0e23e3c4dc285ab4c548e75319f80d21ec798ee169eb8d6815c1187d744c01201738ec82c0dcb6a385eac229b7a020f8ecb60862e7062748cd291804d06b99740de4ec343c29762a1d85d8c52e751433716f330f39c7a21b97f49809fa07c1c61c09e6d62d4ea38ea48fbdcb040634ac2f2a906eee9e4a1756f1e9803c406a7f15ec8c3b8f23862217211e19057b17dae869e7b2598af50c0c53cf03969a857508dd10457c3ea5646daee725ca794aaab1f2d08f35327cfe0e4170de83fb418275af3306cb984688824a69c6dd080bb2ff7f0dc5417c78f57c3774c74e93610abdf8f63ee668e22c208e8740a58778b8b9b0667b0efdd5ae4b78da3dbc5c5e4778b429e4a00634d2c994c1eb16da767222f26de5ef30e5b76cfdfc8ac3aba74ee8c67b73d84873eea84759514ff2120c405de9eed23bf6e01be787f15e905b4ef3112ed4b2b30666d1e6a9b6982341edc4960d32d693c357d09f6eefe1021a898be700a7e55361d463a43de21f4d6d6d6cf29b2fbd896e3df71d97c7eeaf419e4447370c6388bdbee988279cfff064bb7d868ca4898344041ffe83eec59730d645526064a96d1955356422bba072fed33c8d40cf75fafa0eae52df8fd9ca5280a17c032a930d0b3ba7219baf6eb0cd3b044169844efa325757f56fffb79ba5543341e41a3d68c095347a36cc3221c31eb61ca2e8ac54548d57e80ca574742094528806598e934fa8e5e86e292fb700c47b3c07d43c558fbc43ff0f4232be91229a0b24d59611b58b5fb29f42de90d43cb64818fd03ddb33911f172ffca182ad7af24592432087e762f9f6c7d1589ac076aac31299275f84f04ba33bea37cfc4c9233bc859406edb620c98ba0d9b0a757cc95aa8e40197f2420c3f968ffb4a1f41cd895ab23fc3f889a3f0e82b0b6193f6e46304c1b58f804be8d557a22affe8c323a83b548f2b87f54361af3658854f099451e6704a590b57399d705daa10276a37527065d0a1db041c6ea362a3720871f226252d342a0633fc3ec86be0d8b16d3fdab4cbc3d011fd4185131e853df938cde8fa5b9231320b992279c274a4dc8d0474ac09cef09e38736d31d6290761311f2a01a7241bd7191d31c4c9c7f19c1aaaf40e3aa67be02481bd1aaf4154c8f446a55eb898e65f825ed529a85b3fa3121842ebd4cbe0b74df8cc72b9ac2a7b19695b9a6e4dee424e5884ee5dcfe4f2dd10d279c9f1f25da8bca90376da743b513e15b9514c36bba22efa069aa45a2a5c1c61e463a03e133b94661c569bc9c7028379078cf9448174fd3340639a0c4db93aac37329be650791152341afd1d954c5f4a66f4837e43b2777cf8521fa6cb459822b6518335790055a95f43b353681516bab0029ccad4a03af10a1441a58e4aa1cd35f4d047a38f3c1cf55223640aae428a64b36c13228f6d81df3e979c48f5fc9c218c37e7306f588f4c2e53fb652f09d3b21618babe34367e8523551d57441bba041a5a20fd792a0edc3e087f5ab81a5f7ddd885933c7603a7dff472aa7782166744bd8c2c0103603556f1b78f2990aea4c143cb07806469f75e04d5c0ed626874aa603b2a8abedba5f56bb15ad8ba9e1995960f278b8d5b7be627b6ada46e6bfe6e39cc1bdabbbc37aee660c9ff6477cb6a71a48c4a8ddc8e0b5d7ca5072630807acdd944e4027de17f6819e18346a3efcb411f042ace80254fdfb59742bff00feda7d60aa2cec07c7c2bebd84e542bd4452e4cf0360babbe0da963535edd91bb86e7b2269f06897b6acfad33a5c55ba10917898229b433bdb8a5b6f1b898ae71fc4a9967a0a2e131df37b62c98a0d58b46005e2eddb102b40236b95bfb008b3678c434bddd79023aae3e547140abeb24834f2306df9a611a07b8686972173676c6b01f55a0e773d25e8d9ae18f35b1cab3e4aa6228d5349bcb0a60ca5b737e1bdcc63e46213fd94bb61544ec4d0b1f3487ce2451ab344041fbfb30c3dbb5c888cf06c025155483be3b939a3082720ea13cf03331a01094dd33691f69304981b8b86f8feea5abef8e9f5387db60593460fc5038baec69bf25570588aa40dc314494c9676e1adb51e96bfb801b17004f3ee9a8029134a84a69b2e1523859afc9a444ecea5ec7c9f4d0ea24ef41be08002f0ec07e5f65f4dd39c1b2c11b823495cb61c9bc5955cd49adbb13b32894a4501ed66c8f0d31898790a97ab0ba07b49bac514c139f3d21a1577e2a928ca4eea306ff8063498cbfe657c1f38a0efc0a9b79e4577f54ac775c354d5a8da304ff88cf992c1764727b266e9e3601661518052632b124e6fdfe7a680e0123576e42db247385c463df5c3c42ea0ef4003fa3170400178600eaa6e7e3b12e0011ab3e89d22871822068d1fc751753571c9a0d8bd19856e091ca4e8085d778c65a88f7e83409fa4df99cf6932a0801f5d98ffa39f02fe96b2d14e835a655140ff5093a9ca4d725de74a26940259c4b3073dea6b3c6ea46549ada61fb7ad0128d5e23a5a0a28e0e1d1c8eefd3efd17159411b5cfaad80c0000000049454e44ae426082","has_pwd":false,"path":"//westend"},{"seed_name":"Alice","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a9b49444154789c75570b54556516feceebbeef85ab20be125f9924f980184d535b6a2e534bcb64d4091341b324a734731ad668594bc7c4345f4c91e84c91955966d358a3948491a60e844ae10b1015b8bceffbdef39afd1fd335d363b3f6e272f8cffffd7bffdffef6be9caeebf80d13c85572c354554d9565791af9785dd77bdbaca2ca814328a2089aa6b74a925446fe2f51144bd872726622b942fe0bfb2de05b2f44a3d1b991486499a228a30800a2c84396559457791095350cbd230eee180b2251153ccf4110841f4d265381d96cdec9715c98b6600168e4ff07f46bc006a8a2aa77070381dd1461b2466b5874b4914a9b736fecade44e5636e91c07f4e9e9e29e5d98a2bb1c265d51349edea5a7ec80a2c76ab52ea5437c78e39961b7c07e0e6c805284d9c160b08045689204c56a35b375822872283fdb888d05a7e1b049e008a6bd238cf98fdc8907eeeb87404866516bf49eaa2a9a447b81c05f235f411f99b10318806c43fa6598011a0a855e21cfa5cf90245169eff08b070e9d84a7b913e3462761f0ed7d919b57ca00282a1e9dbe089ec94cc1e03e6ed45ce88428f148e86d87dd25699a4abbd38129ea4f1c0ec74cda9299017e13582457c214a9dfef2fe0799eb2ab4312057ee54b6fe3eb6f7f80d562027b96ff6a36ae366bf8e8f3f3f43a70f7b0043c363d0915651e4442373865b58b1836ba1b0489d7698da2e99a64b55877d8edb61cfa37bb7395011b1f883c7707fdfe93224fe45135cd6133f3e72e5ec392956fc26292c00b3cdadafd983e39056bff94819aba66229e8a3b6e8f43f599465c38d30e93856d05c8111583867745e20017226183740ae55f94ccd68516b379372d1119301d0a08f9fd671a7cdee4fd17aa94f670581c9ed01d137bf4c5c2e5f9b872b5054e8715cd2d9d7875cd02c4c56958bf632b3150c69c19b3307ddc349415d712a379232b2c9ba326f44479b5075f1dbf4a3ce1b5e913faf34903ba86cd167b5f5ad764004758c90483ef6eabf84e39dd745d74984cf08623589a360a31ad2a36bef9193abd414c1a7b171e993e0253e6cf4183c7431c90a0a92a0ebcb5077de39270f9875623337d0739d1a9cad8907f1282c051f969e81e6f975f78324d72c73af22c16eb4a039852fc6d6bc03feae513a5aaaa6b024bb73f1ac5f0b804ac1c3f098d4d1e789a5a317468128e96956266f6027475bb5979a1b1d98375cfe76259d6625caea9232001897d7ae2fd4f2bf1d1171711eb341b590804657d79562a3764505cab64b20de6a84e53bd3edf2989c0b6969fc0c9c66b70881471348c65a3c6c256e3c10bcfbd8c7030847113efc5d2554b30635106ce5fbe04b3c94cf7c7a378ef7e7c59760c057bdf06dd2572739661e488fbb17af3513a08718622eed1cd8edca523558b5910ac56db628e4a677520187cc924086a4b30281caabf8836021de28ec73deeeef8fdd4f9a8bd74054e97130dd71bb06dd726248f4ec69a8d1ba86e4378862275c7c462e2dc5970daed0630a502478ade832f188b92efea60318b9834ba370626bab57044e1cd66d3679cd7eb2d566479022dd75c760bdfdcd889bafa160c1fd61fb575759839612e6c362b483dd0e269c5dcccd958fffa5aba9e00a84ce0703a5150f40f3cbb7635dd633ca51546fa77e76d41fa433371e2d43974713bd0bf4f02d57c88d4ceb016aeb3b3b33a1a9507399d56fdd0a1726ef3e67f22e00f23b15f3cb66ccec2ababff8abd7b3e04d5204c6613fe7ea000c352ef42301024e5e221d29d76783b31f5f179b85073d98878e48814eccb2fc4866d9fa2f47815cc6609f31eb9178b322622188cd2f590fc76747454e9ba962447552d3bfb6f7c43431b5c2e1b1a1b3b9091311e394f4fc6f64dbbd0d6da8629d3efc7c87bd3100e85299b4c80284d14a2d56cc6f99acbf8e4c8e7208dc792b97fc0c9f27afc795d1112e263a9de15e3406fbeb604fd13bb211291750398242829108868590b77f23e5f08569b192dcd5e4c9d4662f1ca5c041a283a45071f2b21aac9b49c107f32c6586a0890a3328e977c47ed53c3b48727a2e8e36fb071db0103981d26188c60c7866c240fbe0dc1509401775613b38d5417167ec9e5e77f41db01b13136bcf67a267af13634545c37d223d94d482092885656bfc40a3256b7a47a583427076525c78de733d2a7e1c54d6bb02cb710d517af13d980699352b17ac52c237a962d46ae3202be87fed0cc24313f9ebb8e5a2aa1a123fa2221ce81dac3970d508e84400d2b700e70c39d4c24a2cec5d217436cdf5ff409563cf102e213e2580ad0ec69c5ae0f7660e49851387ce434e2baba909232c0b816ea5c6c3f1f47ed2f2f180aad2092a85159168aabfe83abed2d48eb7f0746f41888fa921ae88a069e3a91426dcf3d241e8ebe31f05e6883425add23b9178e961ec3e2f41c7489771b07f279fd282212f64a198c6fea6b60934c48ebd2032e935953349527c5fb9a09c8046276b1dd62d5d77ffc2eb7f7d857b0996ea8cd86058b91e6e887fad3572050f315634ce896da13ad151e849be9de298502a53d614c6ffc65d52bf8f7c1c39015158b9e9a8f8ce71761cdd1c3f045a3203544727c37ac481dcd1aa560b3d956b1ae2dd0a471b6c5db39382b3f4f8b2a0a6f22b2b407fc98306438362e7c0afef600e450148e0417daaeb5a0f9db6be025ea4404ccd2df65680245de0395a7ced0a573189e320cfbbe2fc7bef367e1b6588d140764597f2e753497d4352e24d96c77717402a27764792010d89453b85daeaaaf95626c0e3476b461f943e9782029196bb7bc8886a66b983d631e664d9a8ddae20b34413066d1bd530bbc6d5c1f9cb9d2807def9f806412f158fa3d507a9ab181743dc6624194586d952425376dacd82336f61d8bd59a610053fbb5d0145077ea5275b78d073fd0bcc1209fd26f2056cd9c8385397348044ae072c6501904b07be77b183d600c1abfaf074f3fce44177c76e089456fc0ef0f5139e9888f77a170d79328edb88692ba5aa6d7facc018331b6571f8ea21d228962150336a60f8afa512512dd172225f18582fc6df1ddb9b33f56e2e1c7a79264da8889025a5a9b913e731e5e5f9f8feba4db6cda4c4cec8da2778a919777d00064d6d2e2c39a176763c68369a8f7d0802009b25d9024de6c5a67b35a7369c98d41808c2e0c2a4b7794d24ead8d0d2192a22a989d391d95e72ae074bad0d1d98e3d5b8be0ec33105b0ebe6fa470ce98fbf0bbee77222b7ba7512acc6d2440056f3d815ebdbb4293b5a80eddc48be21197d3793fe130e36e02134d0cd369e63a40d1cfa0cf8acd6ae32bab2af8ed6f6d86a7a50993c74f41faec4c646c5d077f380c3391d0471d6ad7d32bd05c1dc0de0f8e19b3d9ac474761ecd8243d1c8e92deb1d14d38ef72b986915684695f9a4da1dd0466c6c08d3fa8b6b787c3e1a5ec7f042ef38220d261381789c5b1ca0afc71cf0ec4120169c446b3b703cf4c9b850593a7c2ebf3b2a6a19340a824912c8b1cd5ec119a301ffc09943d3326c2ff0566760b9c8032a957ef2439b4d0fe941b9ebeb2800b2b32b7ac70075775b51602dd7bacdd81ed594fa35f7c772a4599dee5046a3a442801168b651dcdd4ec4e9909e40628b39f0333a3fd8d74d0356b097480e7c833e97357b6d642e2d2d0de8a8f4e94222ccb98323c0d43136998a7d45354cc432693693f81ae27706a4086b1fd6e88fb4ff66bc037cd603b3953b138fa0ef530a9dc0cf29194ce385237f6bd061159e6e8003e2a91724aeb670c94c6a14bf41a33b6874afe0b90ff0298033ab9521b07fe0000000049454e44ae426082","has_pwd":false,"path":""},{"seed_name":"Alice","address_key":"0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a9049444154789c7557097415e515fefe99796bde4b8440d9820d152cb1609045c21e928a620228521615105caa65892c021501058b5401110415045c40b0842a1528b2a8a887200464af040252812c24e16d79cbacfde659286df5e6dc7326ffdcb9dfddfe7bef139665e16748261be4241986d149d3b402721fcbb4325c900dc17315866c4aa875288e7d0e8763bba2287b797ced3b85ac93ff8f7e0ef8fa07aaaa8e48241213755dcfb165650868c242b91ca0808956461a7c96031a9f252120cbf2774ea77395cbe55a2184885385ed8049fe2fa09f024e82ea86de395a1f5d4b0fdb19a6092ab1d99025496c779d176572c012146c6c7ac4e0441bcb6d48966119128f782c40efab3d1ecf381a51fce35992ae83fd2f7012941e3e168d4657990474399cbacf93421f210bddc43f129528f69c85c7b27d17080b15f9f10c74575ac170397802534dc48c783ceaa0a120f862f214eab589af7f04bf1138091a8bc55e24cfb44fdd04ad0ad4286f6e5f8f1f6a2a30b0733ef2fbe46195f52dad605821216c25305cce867ee424b6ed5a03b72b05fdee1983cc56edcd44224a3542a6d75b7c3edf7d54695312fc1a7012341e8f3f168e4456319c7496de3a5dd2bdb31fc1d7fb3e053c5e8a007fffd306f8bade82cfb4730c3d90253541c78b02932674434d4d08b6ba8c8c2658bcb4043e7f038b45a913c3e1f6b897fb527ce3a9c2ceb96103271f98cbce6a227ed0a928886b9ae9f7a74a074e1c46cee4c1f0badc506419c1ab3578e89e61583775292e86ab5950065af99be12fc58bb06cc954a4374aa32a0b7575214c7f76350a068c45381486a238753aa2c88a18eb76bbd75248b1816192b544fc78d9f717db4d7b75b57eb1aa5619989b83492307a2c3f8425cba580ea73f0d6a6d15564d7b13d903eec4bcf046e8c2c0284f3fb43f6ee1a9a77b31424cb05dbfa4956f1fc391f24358f7f11286df6b3e3962b694d3213f2e3b45262da84a02b39846e89afa41e184e7f52f767eada0412a501bc08a85b3d0be7d3a46bff2475c0905302ab71093c63e85426b012a10809319d22d1d5b5ccfa1f6af3bf1dea617a0480e3cf2e802a4b7b805c327f66175030915c8cc68a6ad5fb4dfd1b8519385bc6acf08d3b4c00a2cb9547925a7e3f08986a6ebb28bd2c1ab2114e6e7e09315f3515d5389ba5825da36ea802fd463186abd8286f0c3aeea4a1a30cf1a86f1be813857798131947173460696ac9985d7de79118d1ba631ef2602a1b0b56afe76d1bde35db59282b682b9ed140a854add8c53c1b839f862e757487a1c0860f1bcd918f9703a0e448a2039e268a0e5a2b99881416231caac4b70b1713884826dd234d49767e270950c93572e378b7552b70b0f4d2d80ccb24d24805619cdb071c901c3ebf1cb6e8feb09c1ab333b525fff82d7e5322e5456cb73576c40f9f79751909783a2d1fdb0df2840bd758e414d455c54a0b3f5066aac3cbce1f9085116d748ad2bdad575c4bba5325c92c5d212490fc7f451b0ffc07b28deba063e5f2ac60e9f81ecb639667d2c22b95dee6d82deee613bcc332dd3f43bbc52dc9dc015bd16372b99a88b9c4489e84f502fd52948a01a2dc518749117a2f46c29a2aa8ede59ed71b04260fb51093e37abca0222cce980f62a3adee2c139b50629921b0d0d3f4289a025093656216a4430183cad19daad6e06eea474567ce6d80f5568144cc560bd1017f00c7ec0fbbc7c04b75ce8e1d88a79ab3fc5f28f971144e0b739795853b4141f1df6a326c2239ad8e22613833bc7b0d3b51fe572154d96d059bb153d8c76507909edda108160e014f39265b0cc36b8b74a4111018d602b8c50381b7dd56c944bef42356b90e91e803365049fd41f0e770a38911065475bf9ecebf85def875072260c4996d0b19984f3be726c554a59821e26c4a0411686c7fb22dd4ae570d1ad24b0a55b59aaa499eb5d9f4809a18225838888a29dde1a0589bb70597c8e84558d4ccfdd28397116f9cf0c82d7df80574746e8ca65bc3ae965140d7a1487ce07082c232b5d46a9a30c7b1c47e1b73c1c4d263dd53134d11b4dcd86f499c03786ba443922be944b591c80573831541b8c7af32d7c27163058326dff253a599b30eaa597b1ebab62765d19ad5ab7c38eb9efa3e47c06ce56dbdd43e0b6a626fa7608a158f90a55e22acf8076662bf4d7ba10dea404a5585cfb345debc67c994e499182a941d4490134533360862b5022df4341897f2ec4adcb68a34cc6afe2b3f0c9becd88c53514f6ec856ab509361f34595cb64aa09ec535f87615ad3239b7950052f86d93888fb3962fd8e0595c61c1f1b7903cc5a1284642d3e47736efc1e9f21f50d8b73bf2725b626f3c9f1dbd9e059282182ee236eb05dc249ec2a694dd88d0fe215a0798979ae1c32332529c4465b4629a89913d1d085fd98f2d3b3e809fedf6fe82c7d1a8415333a1c6254eab2f85aa69790cf79eb414af3566ceab62fdfb1f01291e8e0d9d177e21baf53b8f6f2293b95958cc57366e172b311aebf0b975048aa5a0a5948ebf89e9387ebc314e55d9bdda44b7360eb4f01cc583937be36a28014d07badf710756cddfcd2a13b2d7eb99ce5a36e558347aa2a2a6b66dc761133943555e70278275010cecd7135b5e7f11816819a2e61534577a606fec24868a3f238d11b0e7b1dd325fb286e349cf009c0bc6205902994ddd58b6760e16ad998b5fa4a7d11803c170c45ab36087b8333b2fc696d9feda90981cadaf5fd46bec34ede4b1d30ef74dec5215d5786ec61ff0f8b0be98b06c0117812a3c79f703e873772ff4559fa7e1567248d458217ce89e8e16076ab076d32c386517468f7d199702957874fa6034bc898d878dc697e2d5372e39a86466b459e774394626810dc3749bba7ae1b36f8efce28979cbccba6058caed723bde9e538482e71fc1a1437b01b63dc4a3d8327723a23d52302fba11bcfb18eeccc5c87f6661dc84ce88d673e53381f4743f96bf7514ebb62ec7e61d2bed796c8d7f782e86f47f5c48b2f51b87c379ca06565812f69e35c4d4b54d9158dcb81a8a48ad5bb61087cf9c4097a2fbe0e3f6c1ad8413ab060f721158cf45e07cf0223b9c8e5fa7667211588ca5af4d412386d5a6baba2066cc5c8d7b0bc7e2fb0b67e17279b4347f3a173273bed7eb9d49911f1701924c36382c1669aa3a5951644e47c3a1b1c07a4c790067cb8eb1e0fc6cc241ac9db9125dfbe76157e20cc30d74505aa0e1c94b78baa82798caa4c71eaf82d7961f44b3e6ad01cb54b9683869f7eed4d4b4bb886393b8064c1549b22291c8c7f47e109ff514b7572a3d734c9ab761192aae566360977c148df83dd6cac7901006332cf16e6b182577c2e96dc5d8b26d85bd6d60c07d13d0bdc7fd563c1ee1bec5ceaa2865a9a9a9d9bcbff69e2d91cd6bc036d9e0c97f78af5fe7e237ceeeaf292eafc6302b095d133e8787eb6d058a5d67e0655bb529c281929b6881de8ed68898716e9f922549b2118d86ed280aeed7bbb9610ef837a87dc6b8f0c50dc0365d07a7d76338ab5730e66ec15349920cfe74118604b1d97d465448f5d422b85f2b1892b815e9bad3d4a98d5fcbf63c96b90170b19bcf9ddacea94dd7416da26812e746224c321cfc016136a10153c963f89c6e4b3af82a24a938aa5c814e942ca3219a1b299cd546f2437a166367da4cd09764593ec5239b6c7dccfe7fe8a780af914266cf61082cab117f43ddcf3569109786aebcc28d9c165b194917a62087b9397ecbb06eb341199d72beb2c9d6619093b237d2bf00eca90db4873fe5580000000049454e44ae426082","has_pwd":false,"path":"//kusama"},{"seed_name":"Alice","address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a9e49444154789c75570b58555516fef779dd7b2e6f44051fa8e01b351109336ac2322d47884629ca6a2cb29cd4926ae62ba399cce9259af5a5d3c494d998658a65a5d40c9a53815f5969be46507ca0c845b8c0e5bece3dcf59e730fa394d2ddd7ce79ebdf7faf77aec7fadc32ccbc22f084fc3a0e1886118399aa6cda2f12bcbb206894c72e6744be32d66f94441ac174571a72008ffa2d7ce1c894043a7f17ff24bc09736a8aa5a1a8d4697e8ba3ec55ecb310e8665a0553f47da75f4170642661efb00ce1ccff3c72449aa72b95ceb18630aa9b00d3069fc0fd0cf013ba0866e4c0e8543eb754d1f67d136c6ecc10c7060fb94af598bde4c1b1912b92436552eb05ccc65d18138dacb6c0841102fc8b2fc101d62abf3ae5768a6577e0aec80928565e170b8ca32c94291d3799963162c9e3704b4055bf1b5520b092ed2c610b1c298e8cac52857367466d2b938d330a3866e2a22a90781afa6f128e9b5c53e80037839b0031a894456d058662b6522d3b52e5df0eee846b45d434a5e02a47c13bb3b6b6ceb6d1028049cef9909b716c489ee4f2170328626cc40826b986958515b3d4f566f8f8d8dbd85f4dbe2805f047640a38a52160c06ab388e331d6b258e3bbaac05beba00c86ad86b27be948196ac461cf31f011987414206c688c3f1c5992508ebeda406889306a160c82b1058bc0546c69b96e876bbd7c6c6c42ca2693be6860dec3c502c2787a2917dbcc8c3500d538c93b8eec37e1c5c721a9c8b024babb42e03fd66c623bb62147c011f342818280fc6feb3efe387b63590c514b205508c4e5c99f624c6a6cca2c3f8c1335e372d4be05dae7bdd2ed77ac2136c60c792705439d479aa655cdd4befeb216f9730f4fa899834bf10fb1f6c42e49c0a219687d2a1227bc548b4f43d80b75f5c09558ba0f0f60598326b12761ebd1fa499704dd2c7307bc45ff0cfa628361dfa1e2e413417e5e672570d1eacf0b23c54e0f936079892a954d1a29b3e7d60957ea2e65b414e8e43a4a30737beba1043275c8b8695a7a0f798e837ad0f620b352c985b808eb65688a244f75bc7dabfd7226174270e9cdd008e133129751e25e144dc5ebd11222740d1750c4b4ad2b6cc9923f64d48a874b9dd8f333b962125bcd7dfda31e583e2a7488fc1f3a28048570099d37371ebdf1e4797b71b119f86f4acfea8ffaa164be615223129198ce3e800e7f1484525ee79b01c2dde46029690963814abeb76a3b27e2ffac7c4900f806e45b1de2e2a62f9e9e93ee6768f66c444393d3d3ddf092e099fdcbf12276abe813b892cf60570f36b0b9134fe5ad4ac38094db390999b80c977cb58585a8053c74f407289745f79546dfd0a9efe4770cefb1181981899760f4e0527a174eb46221586a86e203339195b4b4a8c5851e45d1ecf024657e7e95028f48ce8968cc0791fffed6bd5e83ae345e6b41c8c2b9d89771f3a892e8ab12b8647cf0515c57fca842bb3091b5ead84120da0a86431265e3d14df372c80c07b28c6f63f2037632d6acf18d87ee2183c8280f9e3b3909d36c00ca92a4709b68391b5bb345d9b460735851891632a0fad5385275d466b831feffdee340437dd58caea50a781ec5b1251f8fb1138f2631be58682099386a1d9bb0d8de7564212fb38591dd53b30367d3906254dc77787f6203921052332c6d24df05be4015b3a98dfef6fa0ab3492ae8ca536a92cb28fe85523f288071266c4a2664d2b0e7dd2ed800b12c3bcb519d8b67d1f36bdbe9fa8c0c2d4824c2c5b39150dad4b2957ce10b089f898f11839e005942f5f88dd753be09264dc7fe7c3282fab40281ca43ca0ebe9eff61f354c638c6d71f0b33067064d3017a31f16e4f112f809120e6fef42a84bc7d88224f4580194156d86ecb1e3cba1bd2d80e5af14627a713c4e367f4e87313162f0edd8f2c9c758bafc3ea4f5ed87a8aac0304d6c7b630f4692e59168d8ea05b68c31545ccc504d98cc26b2211e332316a44c11b1d7c968fe36446ed6312a3f118d8d5e2c9c5b8df84417783a799b3780279ebb11737e9b85e673f5e46913190367e2afefbe8c3f569623ad7f1a34ba4ea17000efaffd07b2c75d49cf4102beccd5cac128537e508800e8e06475f2ec38d46df3a1fecd7687b912d324ccad4cc7aa176af1f9b66354028151e35251b97e362e449e85cfff1d011b484d9e813ef242dcbdf4d738dc7884520d983bab14ab2baac8faa8c3f37672d5d395ba8a1e4d2692f74322cc6e0b621a83af3d884d0f52dca8d8d9f10db4eb987a570aae7920155fee6821e6d290973f022a578bc3279f854b4c26084a2ecd872b325781d747624f5d3552925391977b3d8837619a5459392ec0a8fc5546c291473991378ca8ce1ffdf00b7436b560f80d39e83b7e0236dcdb00d5763b1509bf57c50d8b07616ca185ed1bd72312f163c6ec0588edebc3c1a672caea24f29605cd082167d86ab428435073aa11b1c4708599439122cb54300d4e12c52f6d0299d6e3f7ef9262656bd713556cff5b3b21c5c8307513c5ef3c8a88311a9f3d7f8a5cc8903a52c6aca7d3f0e423c5d8bba78e2813481d3800af6fde8d207b0f5e5f2dad3331acdf6d50f912dcb665237ca130344aacfc2143b0a1b8d8a093f11e8fe70fcc344d3e180e1d0eb6778dde7ccb53a611d5389e1829e2ebc1f09ba6a0f88d729c6feacdeaccc9fdb0bffe4b2cba6316e212126d97a1e3422b1e7be615cc2b5b8c96f33f5258040c4ccac29abd7bf0625d5d2f655a16fc4499ef1417b32983064538591eef1409251a2d0f8742ab3ebc7385e63d7042b42993580cd754dc81ecd2dfe0e4eba7a1f90ca4cdec87f0702fee9b7d0d2933c96209dd9ddd585df521b2ae4bc70f67dea64c772327b504dfb47870dfc75b902cc710a1e8889524bdbaa44418d6a7cf464996ef7280a95771470cf5ccd9bd87fb7d51f1a6a97487b881796330edcf653856e145e7be0004a24c5d3190bb6634be3af701de7ae9054a160d37df7a37ee5c5486cf8f3f40bf438eabdd420a6666acc3ba7dc7b1f5df87a842f1d6c37979b82d2b8b5181c81205e1a80d2c5022ea6a343a4731b42d5a4831d440848b1b9cc482c7151c20aeb6bb0fd07fa711b8291e394f8d41cbd966a86604190346e1c0f9cdf8bef5e5de468044d13b9137e0498c4abe192d012f245ed4125c143f517cce23cbcb68496f234042371286ed6e4555cb7981d74c5d172d1d38b8b419c106852ce6a0050d8c7b7a0894820e1ceb3902933730848d427fcb855dcd8b296f2877c86291f360da90b5f08803c8f5864aad8fc404a136212e6e3ae1d8c22e02331ab658c160f0236a0c8aa82ce89c9b71c146853bfbae0f2a31579fbc78a4dc2163b77f27344b252790fba1e2ba984284833fa2b1fb2332c58de14945181077b565980af55b20cff28df1f1f15710715021a06d448c17816db1c19d1f74b75f5314e521fb15efe634ead305533319758b680d9e475d7837a88fa695cce932c7bb7230d63d190ac5980e6331c6518717e64935a3af8b5aea3067ff17d4f12c0daa2f347b995c0257a3eafc7024bc8e3a12376d22df30f223fd611afb5ad9c53a0d1f81303a801bf9f20d88e7e28993889aa8ffb6bb1a8eea287596cf514f6dc7d4964ba0b6fc14d8161bdc76071514b33fb9fd311af3e9998a2deda60a12b68238a9353a9f32e9e23024737de91346a55946802c42d7ac9a409fa7cf99a3f4d2165b9fdd015d929f03be284eb6d3a0a4b152e81baa9858ae88bea1f298c55204263a1b0d4b272f18018117f6935b775038aa89589a68ca165b8741c3597bb9fc071e6834b2076c49ae0000000049454e44ae426082","has_pwd":false,"path":"//Alice"},{"seed_name":"Alice","address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000aaf49444154789c7557097854d515feefdb66e6cd64932568302a2a42855059048a408850c18018aaa2b82048554050d9945245502255b01411fb459116431465111094b2f3b109585430024a3024280984cc6496b7dfdbf3e6536cad9ef9eeacf7deffacff39c38410f815916979b4d2e2795e17c7718a1ddbe9cb99688d0c857e6340ca95254734289aba5755d58d8aa2ecf4b7d3f245a1e5d2fa3ff935e08b076cdbbec7b2ac89aeebf6109cf66a1298e141de589f7e7507b480c80b02490f4c669065f998a6696f040281c58c3193aef00de0b4e8f04ff24bc06950d773bba692a9a56465078fce31ff21314f86c4020f7dce9435670524bab12093992bbb0aafa52ab8edd13782d14ea88a5a1f0a85c693122be93e46cb978b603f074e8392856352a9d41b5c700424cd8d2821064fc84e840cdb590b7dc841881c2d6d0b3b63c158d01eda63ed118ad2698571c3b3bc9467aabe0204fe0aadc9f48b2fbe0269c0ff064e831a86f102ad1982b60425d5adb31a95bfd7ac45ad518f5b2fef85e1b16ef00ab791f5641cb95dd4995057f4c2f6a2735856b91e612d845179b7a263a40d4f714b300199ac5e1b89446ea7fb7da19bc92d3f00a7412dcb1c934c24dea04b39c85a55d2a4419f4ec3eefa5d645d88fc2ab0aef0551497e5c07ce54b481e79bb241fdfbcdc1c3df78f451329477722379c8f7d3d5e477325221cc15d08a10683c1d7f470e431da407e82e703a7df78aed33596b40e927e109ec5752d43fae2c211fc6effa308c941f2a084987501f7e50dc1dbbd6622faf9698824474ecf7cccff6a05a61c791959c1e6040b34d1beb70a9ec5c82b87a2c18a4321cf71ce958ca0345a0b0497d216c507a65781a6a47dc4bbf04d87c081e75ca4be53903f10d18271e8be67346a9335942c1970acf378adeb3318eedd897d077d43247468079c6efd290af7903112c59d3c454fd8d76739ba7cb71ff69765909410b76f982af1bc3e66b62e5dc924a52e0d6cdbd63d71c3a9d0ff35c295abd62b2298036e342270731936b4ba11538ecec63937863b73fbe3f9bc71d859aec36c221c895c452003467094f31578b9aa82c22e61dad5e3305ec986bdb63fedd1c03c133ceb1a2775db263594d5725e2818984ac01cf1a4b9cf6dfaae87bea6882ac7917d7733b301d615839131ec1dd4d79d476d9381ced73643cd318e1d15410475328c32c78c4be8d8dfc06f0b657cf56d140149a04dfe6568da558ae0c1d9107a2b728000b32e08e3d6d5443d850d5921d68eb98ed3a5a9a9e91014baece3bb2155ad070f661223352150bc109b9ac6e1a9a74d18a9000a0b0dcc9eeae2931511349d6390fcec20abfb8f4ee2cdd540f9721d1ef7f0e493321e29dc0efbdda1944a44389e0964b78579fb168f2b1139ac071e66543acfa692c9594c0d7a227e5ace38bc006abc16e6157d71becd2318382c8093df3264660ad47f2fe1f50526860fe4d8bf3902cf65e8d43385aa0b1e7eff071de13007e7a410b9ffa30f186e505740fb6215b8a623563016c8bd910b3b2151826d6064ed56c7718b285a3cac35974ed9b5a86a3a8e1b2fed86bae301dc54c2110e4a50a8e0eacf01631ef0b070ae8dd8c946c0f190d5ee1294bd1dc0c4690a5ab524af9213beaf03962ff2507287822dd59fa045a8393a44da23653690cf256252769ec562b1e3b66bb7cd52b3c5f233ff604f548e47ccb5d15ebf1a1ff65a8b594f5f8765151e5422af605060fd7b0efa50649c5dc740dc08b4cdc3995ebd70cbdd617cfd0da3a416e8d605585d1ec7d813c3f1e1d96d085048265d350533af9d83841b27d225f74763d14ae189f616b778cf7d5da553462d9aa9196830e21877cd4398dfe64dccab88a2f182824103187ae7d5435ab2094c23174812156d0ae2ce9b7024a32d56ada2a6c1389eb82b0b1bbd451873700272f4300ccf804fbf7b7a1ec0f5910e487929c1a2d16825e3ac7dcc8bf1ee7b6f901a9d46ea78193867c6706febe128eff60e0e6d6b44b25e41bbfe1222661d024b37033ad52c99cc08d81adc0366413b1cdfe812b087de832ec1bceab9987a643a9a8732e1700771d7c0f6ee3bd03dbb07599d20e058f4b8e33a6db3c9d5cf7ffd1c9b7d6c4e3a50ba1ac4d69b37a0ea95422c2bb5897d24b4bc5260c63a03f95fec8277b8da0f1770e925480e2bc2dc913938ba9d511f13e83b4cc2dd65d518b8ef169c88569182c01d9797e09f9dca6172933ed283926b2fb5be9ef49607d58074c0de8a2f1347d1af4511b24e75c1e47e0e558404d2038d6780e2291ec694d2e17f7f4fec4efdbe532e36ad8ee0f5071564b7229d49e9e83952f01d19570d3d853535ebd02a948b7ea121f01c099cb97e8ce38cdadf3c23654c9655e6d996907757e4e0ec0986ce832de4172430bd470446923a55183857cb30bad4c14d139398bbf92c71b287270a73611fc8c4ac612ab25b1030a7d61965786e6b12a188861d4bc3d073803ef74791d39273db6292a629bb18595b148bc6b7ea99107f9da0b1956532652fd52369f7d23a1b4e5460f1388a27d5679bce1e26be6563d89a833874f4bc4ff5c8ce0961ef841ef8f8d90cec5e2d9155c0d0c9a4dc3d02138a02686ce0e97aefd21798bbdaa47e26645dd79f629c0b3965c48e36d4f17613fae9dc3689eac9ad5162a6de25365e2807aa0f85108dd9e8d43d806db5a771f3c283888455c8945cb19885f977b5c3a47ed7a3720f8585cbb8aeaf8ba52f3958f2e7209a5d268854807823137ff9c0609d7a734361591d89ab05a80f4f4a2653f3a70dd19daf0e496a56339fa504c63d2363c0cce358e0cd45bd568d127324aead198682455b896f880d65b2d0b0b0f2c13ec8ef5c89c5da4b088a10c6e07124de2fc4b47b5d643763206310ce84fbb72d29a5751bad3ca085ef4f03730f419b47ab3fdbc55a2e7c32c0e3514805bda81e17d8783cfb76ec133b9121b2906209bc855538b6b323a6ef3a425c4731ef7839c60e9131421d80a488933e1c2d582e2af8666c987d053e7a9713eb3171ffd30e063de0d03c9875bdaaca953eb042fc43d3877d87e325de3793cc4bc48594971b62c7700423e541d0994efc2ba381d5e336eb3ebc985186933535300c1b1dda5e8db7138b30479d8c96b894ae02cea30e739c3294c823505d4fed55559c8c1c4f65422fd5f5d00cda42aa90c524446af0924963be6d1b9328c31dcf662ad101c6e80351291d46446422c6a35818780ffa920896cc2aa501cdc6e08747a26066773c8022ca3f8f661ae27611c132632b5a7b6da841583665ba2641db929915194038beb01f81a9c4d3221289c40764fd5048c20d41972aa5cfa425ea3cb2e22c0ae5620c3d751fc6761a82a41d8786001268c2abeb56a1a6b81acb538b684c0a63b8f347147ac5c240d2159ca934e49fc8cccce844cd81a24dcef36be407605f7cf0f407aaed45a6698ea75110044e85252bb667b2ccac4c6a873bf0a7e2d1c8a2e2f499ab215187474a6760c4f447a947c741f3340d4492974442a6db98aaa95b22e1c8901f40d39ea54533c44fc0be5c04b76c6b1411cb6297bb4146df32c13ca648cc8a1b6c7a9f51ec78d5e750e891a9e460eeee654436d7702765d1d8c9647faa9199427c102ca599da8fa92f17417df939b02f049376874793612e0df753688da2f7cdfc96170c8770b6aa161b1655c04a99281a791b7ed3bb0b52b1387980da9dc40c4dd55611e88bf477a692eef1c5bf8faaf927f925e01f259dedb4887f4573fa0f55422c3794feb47557026af35058a7837e8d9acc4c197172e96172f3061ade57499274928ef9e2dfe1d1a2bdff2bff015e411eb6f81c41390000000049454e44ae426082","has_pwd":false,"path":"//polkadot"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_ids_seed_name_network() {
        let dbname = "for_tests/print_ids_seed_name_network";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_identities_for_seed_name_and_network(dbname, "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), None, Vec::new()).unwrap();
        let expected_print = r#""root":{"seed_name":"Alice","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a9b49444154789c75570b54556516feceebbeef85ab20be125f9924f980184d535b6a2e534bcb64d4091341b324a734731ad668594bc7c4345f4c91e84c91955966d358a3948491a60e844ae10b1015b8bceffbdef39afd1fd335d363b3f6e272f8cffffd7bffdffef6be9caeebf80d13c85572c354554d9565791af9785dd77bdbaca2ca814328a2089aa6b74a925446fe2f51144bd872726622b942fe0bfb2de05b2f44a3d1b991486499a228a30800a2c84396559457791095350cbd230eee180b2251153ccf4110841f4d265381d96cdec9715c98b6600168e4ff07f46bc006a8a2aa77070381dd1461b2466b5874b4914a9b736fecade44e5636e91c07f4e9e9e29e5d98a2bb1c265d51349edea5a7ec80a2c76ab52ea5437c78e39961b7c07e0e6c805284d9c160b08045689204c56a35b375822872283fdb888d05a7e1b049e008a6bd238cf98fdc8907eeeb87404866516bf49eaa2a9a447b81c05f235f411f99b10318806c43fa6598011a0a855e21cfa5cf90245169eff08b070e9d84a7b913e3462761f0ed7d919b57ca00282a1e9dbe089ec94cc1e03e6ed45ce88428f148e86d87dd25699a4abbd38129ea4f1c0ec74cda9299017e13582457c214a9dfef2fe0799eb2ab4312057ee54b6fe3eb6f7f80d562027b96ff6a36ae366bf8e8f3f3f43a70f7b0043c363d0915651e4442373865b58b1836ba1b0489d7698da2e99a64b55877d8edb61cfa37bb7395011b1f883c7707fdfe93224fe45135cd6133f3e72e5ec392956fc26292c00b3cdadafd983e39056bff94819aba66229e8a3b6e8f43f599465c38d30e93856d05c8111583867745e20017226183740ae55f94ccd68516b379372d1119301d0a08f9fd671a7cdee4fd17aa94f670581c9ed01d137bf4c5c2e5f9b872b5054e8715cd2d9d7875cd02c4c56958bf632b3150c69c19b3307ddc349415d712a379232b2c9ba326f44479b5075f1dbf4a3ce1b5e913faf34903ba86cd167b5f5ad764004758c90483ef6eabf84e39dd745d74984cf08623589a360a31ad2a36bef9193abd414c1a7b171e993e0253e6cf4183c7431c90a0a92a0ebcb5077de39270f9875623337d0739d1a9cad8907f1282c051f969e81e6f975f78324d72c73af22c16eb4a039852fc6d6bc03feae513a5aaaa6b024bb73f1ac5f0b804ac1c3f098d4d1e789a5a317468128e96956266f6027475bb5979a1b1d98375cfe76259d6625caea9232001897d7ae2fd4f2bf1d1171711eb341b590804657d79562a3764505cab64b20de6a84e53bd3edf2989c0b6969fc0c9c66b70881471348c65a3c6c256e3c10bcfbd8c7030847113efc5d2554b30635106ce5fbe04b3c94cf7c7a378ef7e7c59760c057bdf06dd2572739661e488fbb17af3513a08718622eed1cd8edca523558b5910ac56db628e4a677520187cc924086a4b30281caabf8836021de28ec73deeeef8fdd4f9a8bd74054e97130dd71bb06dd726248f4ec69a8d1ba86e4378862275c7c462e2dc5970daed0630a502478ade832f188b92efea60318b9834ba370626bab57044e1cd66d3679cd7eb2d566479022dd75c760bdfdcd889bafa160c1fd61fb575759839612e6c362b483dd0e269c5dcccd958fffa5aba9e00a84ce0703a5150f40f3cbb7635dd633ca51546fa77e76d41fa433371e2d43974713bd0bf4f02d57c88d4ceb016aeb3b3b33a1a9507399d56fdd0a1726ef3e67f22e00f23b15f3cb66ccec2ababff8abd7b3e04d5204c6613fe7ea000c352ef42301024e5e221d29d76783b31f5f179b85073d98878e48814eccb2fc4866d9fa2f47815cc6609f31eb9178b322622188cd2f590fc76747454e9ba962447552d3bfb6f7c43431b5c2e1b1a1b3b9091311e394f4fc6f64dbbd0d6da8629d3efc7c87bd3100e85299b4c80284d14a2d56cc6f99acbf8e4c8e7208dc792b97fc0c9f27afc795d1112e263a9de15e3406fbeb604fd13bb211291750398242829108868590b77f23e5f08569b192dcd5e4c9d4662f1ca5c041a283a45071f2b21aac9b49c107f32c6586a0890a3328e977c47ed53c3b48727a2e8e36fb071db0103981d26188c60c7866c240fbe0dc1509401775613b38d5417167ec9e5e77f41db01b13136bcf67a267af13634545c37d223d94d482092885656bfc40a3256b7a47a583427076525c78de733d2a7e1c54d6bb02cb710d517af13d980699352b17ac52c237a962d46ae3202be87fed0cc24313f9ebb8e5a2aa1a123fa2221ce81dac3970d508e84400d2b700e70c39d4c24a2cec5d217436cdf5ff409563cf102e213e2580ad0ec69c5ae0f7660e49851387ce434e2baba909232c0b816ea5c6c3f1f47ed2f2f180aad2092a85159168aabfe83abed2d48eb7f0746f41888fa921ae88a069e3a91426dcf3d241e8ebe31f05e6883425add23b9178e961ec3e2f41c7489771b07f279fd282212f64a198c6fea6b60934c48ebd2032e935953349527c5fb9a09c8046276b1dd62d5d77ffc2eb7f7d857b0996ea8cd86058b91e6e887fad3572050f315634ce896da13ad151e849be9de298502a53d614c6ffc65d52bf8f7c1c39015158b9e9a8f8ce71761cdd1c3f045a3203544727c37ac481dcd1aa560b3d956b1ae2dd0a471b6c5db39382b3f4f8b2a0a6f22b2b407fc98306438362e7c0afef600e450148e0417daaeb5a0f9db6be025ea4404ccd2df65680245de0395a7ced0a573189e320cfbbe2fc7bef367e1b6588d140764597f2e753497d4352e24d96c77717402a27764792010d89453b85daeaaaf95626c0e3476b461f943e9782029196bb7bc8886a66b983d631e664d9a8ddae20b34413066d1bd530bbc6d5c1f9cb9d2807def9f806412f158fa3d507a9ab181743dc6624194586d952425376dacd82336f61d8bd59a610053fbb5d0145077ea5275b78d073fd0bcc1209fd26f2056cd9c8385397348044ae072c6501904b07be77b183d600c1abfaf074f3fce44177c76e089456fc0ef0f5139e9888f77a170d79328edb88692ba5aa6d7facc018331b6571f8ea21d228962150336a60f8afa512512dd172225f18582fc6df1ddb9b33f56e2e1c7a79264da8889025a5a9b913e731e5e5f9f8feba4db6cda4c4cec8da2778a919777d00064d6d2e2c39a176763c68369a8f7d0802009b25d9024de6c5a67b35a7369c98d41808c2e0c2a4b7794d24ead8d0d2192a22a989d391d95e72ae074bad0d1d98e3d5b8be0ec33105b0ebe6fa470ce98fbf0bbee77222b7ba7512acc6d2440056f3d815ebdbb4293b5a80eddc48be21197d3793fe130e36e02134d0cd369e63a40d1cfa0cf8acd6ae32bab2af8ed6f6d86a7a50993c74f41faec4c646c5d077f380c3391d0471d6ad7d32bd05c1dc0de0f8e19b3d9ac474761ecd8243d1c8e92deb1d14d38ef72b986915684695f9a4da1dd0466c6c08d3fa8b6b787c3e1a5ec7f042ef38220d261381789c5b1ca0afc71cf0ec4120169c446b3b703cf4c9b850593a7c2ebf3b2a6a19340a824912c8b1cd5ec119a301ffc09943d3326c2ff0566760b9c8032a957ef2439b4d0fe941b9ebeb2800b2b32b7ac70075775b51602dd7bacdd81ed594fa35f7c772a4599dee5046a3a442801168b651dcdd4ec4e9909e40628b39f0333a3fd8d74d0356b097480e7c833e97357b6d642e2d2d0de8a8f4e94222ccb98323c0d43136998a7d45354cc432693693f81ae27706a4086b1fd6e88fb4ff66bc037cd603b3953b138fa0ef530a9dc0cf29194ce385237f6bd061159e6e8003e2a91724aeb670c94c6a14bf41a33b6874afe0b90ff0298033ab9521b07fe0000000049454e44ae426082","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","swiped":false,"multiselect":false},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a7149444154789c7557097415d519feee9de5ed592481200261070544402522229155100a226a018fa0d50a1605a4ad4bb5b41a8f47a02a05ec091ea46244d10222b56c22d4a29168544021248201819085bcf766e6cd7efbcf43ad5afde7dc7366eef27fff7eff614208fc0c49343c1a59f23c6f90e338e31dc71d2e847751583ebf667b907cc19b1545dea728ca3f6559de43d3d9352299864be3ffe8e780bf3b60dbf6ad9665cd735d7748b0576280e3331c6d51e0fa40d77c1739aa0f9bde39639024e9b0aaaae5a150682563cc24168102b48a1f00fd14701694341cacebfa1ad771fa0667183d60cca3c136d744d9e12655309a6f17f3d8b44b7411937de193ea7496051cb92c9f8d4622734988d7e9930e672958cad28f81b3a0a4e19d866194076b4c925ca1a801332944b8271a7d541c8a2322d31acda62c8e71dd0c5cded987ee2ba4357ce6ba1e5c47a133884422cb682ca4d780020182e91f0067413399cce3341e0e7640965d66e8b27af013302d0d74e98ad3175e8c973e0a81ac0a89f4d32d604a7f07fd9c63f00e1e025314d817f783dfa6d087e304dc25d27a733c1eff05710c884e427c0b9c05b54ceb4e5dd3cb19677ed62b0ae7b12daf43f9b2068218065bed2937e35dd11b1f1e27d5c0d08bf88f6f7f06f1d7d79d178e36f9f91740bb7126841aa113be4b2e5022e1f08a683c7a2f710d7cee05c0d917d7f106a7ade47ea6d2970d9f9cc6f9578d886f5847021028a9c80d9db4190067c2f5686ab6e0b80ceddaa9502aab107e771bfc58820e8bec3e63d4443883fb82e91982e204eecb319e981d0aab6b68931c00077b91b65307aca37e5fe3b1b0eb9f66b23ac643f43e1df18abf83b59e830885c0351d9909e361e65c01790ff9d8637006925c055f22bebe02be2c83f9642a0a3b7dd60c18ffea0073358565047e7491cd95e19e99c3f38ab9cc1ab2c0b665dfaa39e90aed96986b6d95659e4be64a02f1953ea2371c45ecad7720db3af41e7d60f51f01f1521c224dae225b319f049869433e5789c4fe4ac294a05f53824cf340a4c6ca2415eda3a492bafb4ece0e4d89b68d2c8984228b98a0836933f5be7dca1b92bc36ee09875295cced3733283738285aef20d9e0c1302cb42f8ec13c4c795c11058b92c158602a0e363283c4081f67ebc9ac8ca3b063084d4f30188bc3e045e72de0b73091b3d160a1117e7382e5f666aee30e4aa552552c0ca4a645606e95c0e23e3c4dc285ab4c548e75319f80d21ec798ee169eb8d6815c1187d744c01201738ec82c0dcb6a385eac229b7a020f8ecb60862e7062748cd291804d06b99740de4ec343c29762a1d85d8c52e751433716f330f39c7a21b97f49809fa07c1c61c09e6d62d4ea38ea48fbdcb040634ac2f2a906eee9e4a1756f1e9803c406a7f15ec8c3b8f23862217211e19057b17dae869e7b2598af50c0c53cf03969a857508dd10457c3ea5646daee725ca794aaab1f2d08f35327cfe0e4170de83fb418275af3306cb984688824a69c6dd080bb2ff7f0dc5417c78f57c3774c74e93610abdf8f63ee668e22c208e8740a58778b8b9b0667b0efdd5ae4b78da3dbc5c5e4778b429e4a00634d2c994c1eb16da767222f26de5ef30e5b76cfdfc8ac3aba74ee8c67b73d84873eea84759514ff2120c405de9eed23bf6e01be787f15e905b4ef3112ed4b2b30666d1e6a9b6982341edc4960d32d693c357d09f6eefe1021a898be700a7e55361d463a43de21f4d6d6d6cf29b2fbd896e3df71d97c7eeaf419e4447370c6388bdbee988279cfff064bb7d868ca4898344041ffe83eec59730d645526064a96d1955356422bba072fed33c8d40cf75fafa0eae52df8fd9ca5280a17c032a930d0b3ba7219baf6eb0cd3b044169844efa325757f56fffb79ba5543341e41a3d68c095347a36cc3221c31eb61ca2e8ac54548d57e80ca574742094528806598e934fa8e5e86e292fb700c47b3c07d43c558fbc43ff0f4232be91229a0b24d59611b58b5fb29f42de90d43cb64818fd03ddb33911f172ffca182ad7af24592432087e762f9f6c7d1589ac076aac31299275f84f04ba33bea37cfc4c9233bc859406edb620c98ba0d9b0a757cc95aa8e40197f2420c3f968ffb4a1f41cd895ab23fc3f889a3f0e82b0b6193f6e46304c1b58f804be8d557a22affe8c323a83b548f2b87f54361af3658854f099451e6704a590b57399d705daa10276a37527065d0a1db041c6ea362a3720871f226252d342a0633fc3ec86be0d8b16d3fdab4cbc3d011fd4185131e853df938cde8fa5b9231320b992279c274a4dc8d0474ac09cef09e38736d31d6290761311f2a01a7241bd7191d31c4c9c7f19c1aaaf40e3aa67be02481bd1aaf4154c8f446a55eb898e65f825ed529a85b3fa3121842ebd4cbe0b74df8cc72b9ac2a7b19695b9a6e4dee424e5884ee5dcfe4f2dd10d279c9f1f25da8bca90376da743b513e15b9514c36bba22efa069aa45a2a5c1c61e463a03e133b94661c569bc9c7028379078cf9448174fd3340639a0c4db93aac37329be650791152341afd1d954c5f4a66f4837e43b2777cf8521fa6cb459822b6518335790055a95f43b353681516bab0029ccad4a03af10a1441a58e4aa1cd35f4d047a38f3c1cf55223640aae428a64b36c13228f6d81df3e979c48f5fc9c218c37e7306f588f4c2e53fb652f09d3b21618babe34367e8523551d57441bba041a5a20fd792a0edc3e087f5ab81a5f7ddd885933c7603a7dff472aa7782166744bd8c2c0103603556f1b78f2990aea4c143cb07806469f75e04d5c0ed626874aa603b2a8abedba5f56bb15ad8ba9e1995960f278b8d5b7be627b6ada46e6bfe6e39cc1bdabbbc37aee660c9ff6477cb6a71a48c4a8ddc8e0b5d7ca5072630807acdd944e4027de17f6819e18346a3efcb411f042ace80254fdfb59742bff00feda7d60aa2cec07c7c2bebd84e542bd4452e4cf0360babbe0da963535edd91bb86e7b2269f06897b6acfad33a5c55ba10917898229b433bdb8a5b6f1b898ae71fc4a9967a0a2e131df37b62c98a0d58b46005e2eddb102b40236b95bfb008b3678c434bddd79023aae3e547140abeb24834f2306df9a611a07b8686972173676c6b01f55a0e773d25e8d9ae18f35b1cab3e4aa6228d5349bcb0a60ca5b737e1bdcc63e46213fd94bb61544ec4d0b1f3487ce2451ab344041fbfb30c3dbb5c888cf06c025155483be3b939a3082720ea13cf03331a01094dd33691f69304981b8b86f8feea5abef8e9f5387db60593460fc5038baec69bf25570588aa40dc314494c9676e1adb51e96bfb801b17004f3ee9a8029134a84a69b2e1523859afc9a444ecea5ec7c9f4d0ea24ef41be08002f0ec07e5f65f4dd39c1b2c11b823495cb61c9bc5955cd49adbb13b32894a4501ed66c8f0d31898790a97ab0ba07b49bac514c139f3d21a1577e2a928ca4eea306ff8063498cbfe657c1f38a0efc0a9b79e4577f54ac775c354d5a8da304ff88cf992c1764727b266e9e3601661518052632b124e6fdfe7a680e0123576e42db247385c463df5c3c42ea0ef4003fa3170400178600eaa6e7e3b12e0011ab3e89d22871822068d1fc751753571c9a0d8bd19856e091ca4e8085d778c65a88f7e83409fa4df99cf6932a0801f5d98ffa39f02fe96b2d14e835a655140ff5093a9ca4d725de74a26940259c4b3073dea6b3c6ea46549ada61fb7ad0128d5e23a5a0a28e0e1d1c8eefd3efd17159411b5cfaad80c0000000049454e44ae426082","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a9e49444154789c75570b58555516fef779dd7b2e6f44051fa8e01b351109336ac2322d47884629ca6a2cb29cd4926ae62ba399cce9259af5a5d3c494d998658a65a5d40c9a53815f5969be46507ca0c845b8c0e5bece3dcf59e730fa394d2ddd7ce79ebdf7faf77aec7fadc32ccbc22f084fc3a0e1886118399aa6cda2f12bcbb206894c72e6744be32d66f94441ac174571a72008ffa2d7ce1c894043a7f17ff24bc09736a8aa5a1a8d4697e8ba3ec55ecb310e8665a0553f47da75f4170642661efb00ce1ccff3c72449aa72b95ceb18630aa9b00d3069fc0fd0cf013ba0866e4c0e8543eb754d1f67d136c6ecc10c7060fb94af598bde4c1b1912b92436552eb05ccc65d18138dacb6c0841102fc8b2fc101d62abf3ae5768a6577e0aec80928565e170b8ca32c94291d3799963162c9e3704b4055bf1b5520b092ed2c610b1c298e8cac52857367466d2b938d330a3866e2a22a90781afa6f128e9b5c53e80037839b0031a894456d058662b6522d3b52e5df0eee846b45d434a5e02a47c13bb3b6b6ceb6d1028049cef9909b716c489ee4f2170328626cc40826b986958515b3d4f566f8f8d8dbd85f4dbe2805f047640a38a52160c06ab388e331d6b258e3bbaac05beba00c86ad86b27be948196ac461cf31f011987414206c688c3f1c5992508ebeda406889306a160c82b1058bc0546c69b96e876bbd7c6c6c42ca2693be6860dec3c502c2787a2917dbcc8c3500d538c93b8eec37e1c5c721a9c8b024babb42e03fd66c623bb62147c011f342818280fc6feb3efe387b63590c514b205508c4e5c99f624c6a6cca2c3f8c1335e372d4be05dae7bdd2ed77ac2136c60c792705439d479aa655cdd4befeb216f9730f4fa899834bf10fb1f6c42e49c0a219687d2a1227bc548b4f43d80b75f5c09558ba0f0f60598326b12761ebd1fa499704dd2c7307bc45ff0cfa628361dfa1e2e413417e5e672570d1eacf0b23c54e0f936079892a954d1a29b3e7d60957ea2e65b414e8e43a4a30737beba1043275c8b8695a7a0f798e837ad0f620b352c985b808eb65688a244f75bc7dabfd7226174270e9cdd008e133129751e25e144dc5ebd11222740d1750c4b4ad2b6cc9923f64d48a874b9dd8f333b962125bcd7dfda31e583e2a7488fc1f3a28048570099d37371ebdf1e4797b71b119f86f4acfea8ffaa164be615223129198ce3e800e7f1484525ee79b01c2dde46029690963814abeb76a3b27e2ffac7c4900f806e45b1de2e2a62f9e9e93ee6768f66c444393d3d3ddf092e099fdcbf12276abe813b892cf60570f36b0b9134fe5ad4ac38094db390999b80c977cb58585a8053c74f407289745f79546dfd0a9efe4770cefb1181981899760f4e0527a174eb46221586a86e203339195b4b4a8c5851e45d1ecf024657e7e95028f48ce8968cc0791fffed6bd5e83ae345e6b41c8c2b9d89771f3a892e8ab12b8647cf0515c57fca842bb3091b5ead84120da0a86431265e3d14df372c80c07b28c6f63f2037632d6acf18d87ee2183c8280f9e3b3909d36c00ca92a4709b68391b5bb345d9b460735851891632a0fad5385275d466b831feffdee340437dd58caea50a781ec5b1251f8fb1138f2631be58682099386a1d9bb0d8de7564212fb38591dd53b30367d3906254dc77787f6203921052332c6d24df05be4015b3a98dfef6fa0ab3492ae8ca536a92cb28fe85523f288071266c4a2664d2b0e7dd2ed800b12c3bcb519d8b67d1f36bdbe9fa8c0c2d4824c2c5b39150dad4b2957ce10b089f898f11839e005942f5f88dd753be09264dc7fe7c3282fab40281ca43ca0ebe9eff61f354c638c6d71f0b33067064d3017a31f16e4f112f809120e6fef42a84bc7d88224f4580194156d86ecb1e3cba1bd2d80e5af14627a713c4e367f4e87313162f0edd8f2c9c758bafc3ea4f5ed87a8aac0304d6c7b630f4692e59168d8ea05b68c31545ccc504d98cc26b2211e332316a44c11b1d7c968fe36446ed6312a3f118d8d5e2c9c5b8df84417783a799b3780279ebb11737e9b85e673f5e46913190367e2afefbe8c3f569623ad7f1a34ba4ea17000efaffd07b2c75d49cf4102beccd5cac128537e508800e8e06475f2ec38d46df3a1fecd7687b912d324ccad4cc7aa176af1f9b66354028151e35251b97e362e449e85cfff1d011b484d9e813ef242dcbdf4d738dc7884520d983bab14ab2baac8faa8c3f37672d5d395ba8a1e4d2692f74322cc6e0b621a83af3d884d0f52dca8d8d9f10db4eb987a570aae7920155fee6821e6d290973f022a578bc3279f854b4c26084a2ecd872b325781d747624f5d3552925391977b3d8837619a5459392ec0a8fc5546c291473991378ca8ce1ffdf00b7436b560f80d39e83b7e0236dcdb00d5763b1509bf57c50d8b07616ca185ed1bd72312f163c6ec0588edebc3c1a672caea24f29605cd082167d86ab428435073aa11b1c4708599439122cb54300d4e12c52f6d0299d6e3f7ef9262656bd713556cff5b3b21c5c8307513c5ef3c8a88311a9f3d7f8a5cc8903a52c6aca7d3f0e423c5d8bba78e2813481d3800af6fde8d207b0f5e5f2dad3331acdf6d50f912dcb665237ca130344aacfc2143b0a1b8d8a093f11e8fe70fcc344d3e180e1d0eb6778dde7ccb53a611d5389e1829e2ebc1f09ba6a0f88d729c6feacdeaccc9fdb0bffe4b2cba6316e212126d97a1e3422b1e7be615cc2b5b8c96f33f5258040c4ccac29abd7bf0625d5d2f655a16fc4499ef1417b32983064538591eef1409251a2d0f8742ab3ebc7385e63d7042b42993580cd754dc81ecd2dfe0e4eba7a1f90ca4cdec87f0702fee9b7d0d2933c96209dd9ddd585df521b2ae4bc70f67dea64c772327b504dfb47870dfc75b902cc710a1e8889524bdbaa44418d6a7cf464996ef7280a95771470cf5ccd9bd87fb7d51f1a6a97487b881796330edcf653856e145e7be0004a24c5d3190bb6634be3af701de7ae9054a160d37df7a37ee5c5486cf8f3f40bf438eabdd420a6666acc3ba7dc7b1f5df87a842f1d6c37979b82d2b8b5181c81205e1a80d2c5022ea6a343a4731b42d5a4831d440848b1b9cc482c7151c20aeb6bb0fd07fa711b8291e394f8d41cbd966a86604190346e1c0f9cdf8bef5e5de468044d13b9137e0498c4abe192d012f245ed4125c143f517cce23cbcb68496f234042371286ed6e4555cb7981d74c5d172d1d38b8b419c106852ce6a0050d8c7b7a0894820e1ceb3902933730848d427fcb855dcd8b296f2877c86291f360da90b5f08803c8f5864aad8fc404a136212e6e3ae1d8c22e02331ab658c160f0236a0c8aa82ce89c9b71c146853bfbae0f2a31579fbc78a4dc2163b77f27344b252790fba1e2ba984284833fa2b1fb2332c58de14945181077b565980af55b20cff28df1f1f15710715021a06d448c17816db1c19d1f74b75f5314e521fb15efe634ead305533319758b680d9e475d7837a88fa695cce932c7bb7230d63d190ac5980e6331c6518717e64935a3af8b5aea3067ff17d4f12c0daa2f347b995c0257a3eafc7024bc8e3a12376d22df30f223fd611afb5ad9c53a0d1f81303a801bf9f20d88e7e28993889aa8ffb6bb1a8eea287596cf514f6dc7d4964ba0b6fc14d8161bdc76071514b33fb9fd311af3e9998a2deda60a12b68238a9353a9f32e9e23024737de91346a55946802c42d7ac9a409fa7cf99a3f4d2165b9fdd015d929f03be284eb6d3a0a4b152e81baa9858ae88bea1f298c55204263a1b0d4b272f18018117f6935b775038aa89589a68ca165b8741c3597bb9fc071e6834b2076c49ae0000000049454e44ae426082","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"}"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
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
        let print = export_key (dbname, &MultiSigner::Sr25519(Public::from_raw(public)), "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r#""qr":"89504e470d0a1a0a0000000d49484452000000c4000000c40800000000e5cdd1b7000006ac49444154789cedcf416e1cdb1504517bff8bb6790601245e574be4c046ff6205406466bc6a09f7dffff9d73f9fe7884fe139e253788ef8149e233e85e7884fe139e253788ef814ee79c4bfbffedee1db7db7176f391dbbb7c3ae0787d32f7d13cf11f6e22da763f776d8f5e070faa56fe2be479c0e7979c29f9cdfed37bd5d39f0b63cb9f2cf11c19f9cdfed37bd5d39f0b63cb9f2f73e42467b336c9cce3e13f5123a6cfdcc68cbe53902364e679f897a091db67e66b4e5f27b8f083bfee4db7519f6e28ddb8cb65c9e23ecf8936fd765d88b376e33da72f99d474087ad6fe2ecb0f532ecc51bb7196db93c47d8fa26ce0e5b2fc35ebc719bd196cbbd8f38c97f37a1a31d7c4e877d767972e59f23fe94d0d10e3ea7c33ebb3cb9f2f73de21dbef5feffca77785f9e23fed7f90eefcb3d8ff80efe03bf93d061eb12575d4287ad97d07fc27384840e5b97b8ea123a6cbd84fe13ee79c4f90fb5b14e97a86fa22e51976183db0e5b970bb73c4748d4375197a8cbb0c16d87adcb855bee7d449cefe89bdeecb3974b4e2e39091d767de197e788deecb3974b4e2e39091d767de197df73c4e9c09ff4ddbeadfb6e2fe36a2fcf11277db76febbedbcbb8dacb3d8f083ff256868d9cad6f62fbc29ff45d6fb62e63b7be3c47c0d637b17de14ffaae375b97b15b5fee7f04bcbfeb6807bfae2d9775f5f2a73c47a01dfcbab65cd6d5cb9f72cf23fc438b774ec6bb2da10767cbe5bb6ee95d2ecf11d196d083b3e5f25db7f42e977b1eb1f8c1e25b6e7379e7c05f75b465d8e0ce2e97e708b9bc73e0af3ada326c706797cb3d8ff011f2bbcfbe1936387d73e1c0d761631dd6d7e33962336c70fae6c281afc3c63aacafc73d8f800fd1db6e5d627b708b774ec66e1db6fe9d3c798ed81edce29d93b15b87ad7f274fee79c4d587ef5cf4966b83b34be8b0f512fa925f5ebe79115f7fdf75d15bae0dce2ea1c3d64be84b7e79f9e6457cfd7dd7456fb93638bb840e5b2fa12ff9e5e59b17f1f5f75d17bde5dae0ec123a6cbd84bee497976f4e013fe437a1a30d6e77f0cbd537f05d6ff54de84b3e9e23c0ed0e7eb9fa06beebadbe097dc9c73d8ff0034ea22e51df848e76f0ebeca5b7f5dcd596d097e788123adac1afb397ded673575b425fee7d0474d8f5e0c25b5b8fd3d9fa996183d3cbe5d2bd88afbf9c0ebb1e5c786beb713a5b3f336c707ab95cba17f1f597d361d7830b6f6d3d4e67eb67860d4e2f974bf722befe723aec7a70e1adadc7e96cfdccb0c1e9e572e95ec4d71f27176ef1ce6d425fd6eb57b9e4364ff8e53982db84beacd7af72c96d9ef0cb3d8f801fe6f5c8456ff9dd7ab970e16df7b26fef7a3c47f496dfad970b17de762ffbf6aec7bd8f887dcf73db97d35f6d5da25e2e1cf8fa15cf11d097d35f6d5da25e2e1cf8fa15f73cc20f4e7c93bfea3276ebb07589bff532ce7df21c5197b15b87ad4bfcad9771ee93fb1ec16d62fbb2be5e42bfa277f8c69661077feee53902ebeb25f42b7a876f6c1976f0e75eee7904fc8097273cf66d5d3d72326cbc73ba841e570ecf11b1ae1e391936de395d428f2b877b1ee1436e133a6cbd0cfb4ff8b66ff425bfec37de779f3c47c8b0ff846ffb465ff2cb7ee37df7c9ef38027ae4e27c6bd74be80bcf9df9539e2370beb5eb25f485e7cefc29f73d02bcbeb9acab97a897d061d79777fec477cb7344ae5ea25e42875d5fdef913df2df73c22fc63de36a15fd1fbe2db2b8fdee49293b8ea72798e587a5f7c7be5d19b5c7212575d2ebfeb88b091b37589b3c3ae0777e29b7c5d2e97ee14d1c765d8c8d9bac4d961d7833bf14dbe2e974b778ae8e3326ce46c5de2ecb0ebc19df8265f97cba53b45f471193672b62e7176d8f5e04e7c93afcbe5d2bd88af3ff07a89bac45597efe85d623bec2bfaa6f7763c475c75f98ede25b6c3bea26f7a6fc73d8ff81bfd434bff466fedc55b5e87ad9761c7fad8773c4744ff466fedc55b5e87ad9761c7fad877dcf388ab1f45dffa46df0c1b5c3d38e4cf0dcebecab097e788cdb0c1d58343fedce0ecab0c7bb9ef11a743be841e9cbd09fdc4db950fefd1779c2e4f9e23f4e0ec4de827deae7c788fbee3747972ef2364b4379775fac2e7f4250f6fbb4fbcbfe33922d6e90b9fd3973cbced3ef1fe8edf7904cebee4a377fe6f7dc94be8918be708e84b3e7ae7ffd697bc841eb9f8dd472cebf532ec13ef7c89b307d7de1ecf11b15e2fc33ef1ce97387b70eded71ef234ef225aeba8cdd75091df6bb7ef2eeed39e2aacbd85d97d061bfeb27efdeee7bc43b7ceb5d9e5c796ed9776fb68cb68cdddb97e788e5ca73cbbe7bb365b465ecdebedcf3887f22cf119fc273c4a7f01cf1293c477c0acf119fc273c4a7f01cf129dce288ff0239d9b86ac67989a70000000049454e44ae426082","pubkey":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a9b49444154789c75570b54556516feceebbeef85ab20be125f9924f980184d535b6a2e534bcb64d4091341b324a734731ad668594bc7c4345f4c91e84c91955966d358a3948491a60e844ae10b1015b8bceffbdef39afd1fd335d363b3f6e272f8cffffd7bffdffef6be9caeebf80d13c85572c354554d9565791af9785dd77bdbaca2ca814328a2089aa6b74a925446fe2f51144bd872726622b942fe0bfb2de05b2f44a3d1b991486499a228a30800a2c84396559457791095350cbd230eee180b2251153ccf4110841f4d265381d96cdec9715c98b6600168e4ff07f46bc006a8a2aa77070381dd1461b2466b5874b4914a9b736fecade44e5636e91c07f4e9e9e29e5d98a2bb1c265d51349edea5a7ec80a2c76ab52ea5437c78e39961b7c07e0e6c805284d9c160b08045689204c56a35b375822872283fdb888d05a7e1b049e008a6bd238cf98fdc8907eeeb87404866516bf49eaa2a9a447b81c05f235f411f99b10318806c43fa6598011a0a855e21cfa5cf90245169eff08b070e9d84a7b913e3462761f0ed7d919b57ca00282a1e9dbe089ec94cc1e03e6ed45ce88428f148e86d87dd25699a4abbd38129ea4f1c0ec74cda9299017e13582457c214a9dfef2fe0799eb2ab4312057ee54b6fe3eb6f7f80d562027b96ff6a36ae366bf8e8f3f3f43a70f7b0043c363d0915651e4442373865b58b1836ba1b0489d7698da2e99a64b55877d8edb61cfa37bb7395011b1f883c7707fdfe93224fe45135cd6133f3e72e5ec392956fc26292c00b3cdadafd983e39056bff94819aba66229e8a3b6e8f43f599465c38d30e93856d05c8111583867745e20017226183740ae55f94ccd68516b379372d1119301d0a08f9fd671a7cdee4fd17aa94f670581c9ed01d137bf4c5c2e5f9b872b5054e8715cd2d9d7875cd02c4c56958bf632b3150c69c19b3307ddc349415d712a379232b2c9ba326f44479b5075f1dbf4a3ce1b5e913faf34903ba86cd167b5f5ad764004758c90483ef6eabf84e39dd745d74984cf08623589a360a31ad2a36bef9193abd414c1a7b171e993e0253e6cf4183c7431c90a0a92a0ebcb5077de39270f9875623337d0739d1a9cad8907f1282c051f969e81e6f975f78324d72c73af22c16eb4a039852fc6d6bc03feae513a5aaaa6b024bb73f1ac5f0b804ac1c3f098d4d1e789a5a317468128e96956266f6027475bb5979a1b1d98375cfe76259d6625caea9232001897d7ae2fd4f2bf1d1171711eb341b590804657d79562a3764505cab64b20de6a84e53bd3edf2989c0b6969fc0c9c66b70881471348c65a3c6c256e3c10bcfbd8c7030847113efc5d2554b30635106ce5fbe04b3c94cf7c7a378ef7e7c59760c057bdf06dd2572739661e488fbb17af3513a08718622eed1cd8edca523558b5910ac56db628e4a677520187cc924086a4b30281caabf8836021de28ec73deeeef8fdd4f9a8bd74054e97130dd71bb06dd726248f4ec69a8d1ba86e4378862275c7c462e2dc5970daed0630a502478ade832f188b92efea60318b9834ba370626bab57044e1cd66d3679cd7eb2d566479022dd75c760bdfdcd889bafa160c1fd61fb575759839612e6c362b483dd0e269c5dcccd958fffa5aba9e00a84ce0703a5150f40f3cbb7635dd633ca51546fa77e76d41fa433371e2d43974713bd0bf4f02d57c88d4ceb016aeb3b3b33a1a9507399d56fdd0a1726ef3e67f22e00f23b15f3cb66ccec2ababff8abd7b3e04d5204c6613fe7ea000c352ef42301024e5e221d29d76783b31f5f179b85073d98878e48814eccb2fc4866d9fa2f47815cc6609f31eb9178b322622188cd2f590fc76747454e9ba962447552d3bfb6f7c43431b5c2e1b1a1b3b9091311e394f4fc6f64dbbd0d6da8629d3efc7c87bd3100e85299b4c80284d14a2d56cc6f99acbf8e4c8e7208dc792b97fc0c9f27afc795d1112e263a9de15e3406fbeb604fd13bb211291750398242829108868590b77f23e5f08569b192dcd5e4c9d4662f1ca5c041a283a45071f2b21aac9b49c107f32c6586a0890a3328e977c47ed53c3b48727a2e8e36fb071db0103981d26188c60c7866c240fbe0dc1509401775613b38d5417167ec9e5e77f41db01b13136bcf67a267af13634545c37d223d94d482092885656bfc40a3256b7a47a583427076525c78de733d2a7e1c54d6bb02cb710d517af13d980699352b17ac52c237a962d46ae3202be87fed0cc24313f9ebb8e5a2aa1a123fa2221ce81dac3970d508e84400d2b700e70c39d4c24a2cec5d217436cdf5ff409563cf102e213e2580ad0ec69c5ae0f7660e49851387ce434e2baba909232c0b816ea5c6c3f1f47ed2f2f180aad2092a85159168aabfe83abed2d48eb7f0746f41888fa921ae88a069e3a91426dcf3d241e8ebe31f05e6883425add23b9178e961ec3e2f41c7489771b07f279fd282212f64a198c6fea6b60934c48ebd2032e935953349527c5fb9a09c8046276b1dd62d5d77ffc2eb7f7d857b0996ea8cd86058b91e6e887fad3572050f315634ce896da13ad151e849be9de298502a53d614c6ffc65d52bf8f7c1c39015158b9e9a8f8ce71761cdd1c3f045a3203544727c37ac481dcd1aa560b3d956b1ae2dd0a471b6c5db39382b3f4f8b2a0a6f22b2b407fc98306438362e7c0afef600e450148e0417daaeb5a0f9db6be025ea4404ccd2df65680245de0395a7ced0a573189e320cfbbe2fc7bef367e1b6588d140764597f2e753497d4352e24d96c77717402a27764792010d89453b85daeaaaf95626c0e3476b461f943e9782029196bb7bc8886a66b983d631e664d9a8ddae20b34413066d1bd530bbc6d5c1f9cb9d2807def9f806412f158fa3d507a9ab181743dc6624194586d952425376dacd82336f61d8bd59a610053fbb5d0145077ea5275b78d073fd0bcc1209fd26f2056cd9c8385397348044ae072c6501904b07be77b183d600c1abfaf074f3fce44177c76e089456fc0ef0f5139e9888f77a170d79328edb88692ba5aa6d7facc018331b6571f8ea21d228962150336a60f8afa512512dd172225f18582fc6df1ddb9b33f56e2e1c7a79264da8889025a5a9b913e731e5e5f9f8feba4db6cda4c4cec8da2778a919777d00064d6d2e2c39a176763c68369a8f7d0802009b25d9024de6c5a67b35a7369c98d41808c2e0c2a4b7794d24ead8d0d2192a22a989d391d95e72ae074bad0d1d98e3d5b8be0ec33105b0ebe6fa470ce98fbf0bbee77222b7ba7512acc6d2440056f3d815ebdbb4293b5a80eddc48be21197d3793fe130e36e02134d0cd369e63a40d1cfa0cf8acd6ae32bab2af8ed6f6d86a7a50993c74f41faec4c646c5d077f380c3391d0471d6ad7d32bd05c1dc0de0f8e19b3d9ac474761ecd8243d1c8e92deb1d14d38ef72b986915684695f9a4da1dd0466c6c08d3fa8b6b787c3e1a5ec7f042ef38220d261381789c5b1ca0afc71cf0ec4120169c446b3b703cf4c9b850593a7c2ebf3b2a6a19340a824912c8b1cd5ec119a301ffc09943d3326c2ff0566760b9c8032a957ef2439b4d0fe941b9ebeb2800b2b32b7ac70075775b51602dd7bacdd81ed594fa35f7c772a4599dee5046a3a442801168b651dcdd4ec4e9909e40628b39f0333a3fd8d74d0356b097480e7c833e97357b6d642e2d2d0de8a8f4e94222ccb98323c0d43136998a7d45354cc432693693f81ae27706a4086b1fd6e88fb4ff66bc037cd603b3953b138fa0ef530a9dc0cf29194ce385237f6bd061159e6e8003e2a91724aeb670c94c6a14bf41a33b6874afe0b90ff0298033ab9521b07fe0000000049454e44ae426082","seed_name":"Alice","path":"","network_title":"Westend","network_logo":"westend""#;
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
        let print = derive_prep(dbname, "Alice", &network_specs_key, Some(collision), "//Alice").unwrap();
        let expected_print = r#""seed_name":"Alice","network_title":"Westend","network_logo":"westend","suggested_derivation":"//Alice","collision":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","path":"//Alice","has_pwd":false,"identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed_name":"Alice"}"#;
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
        let print = derive_prep(dbname, "Alice", &network_specs_key, Some(collision), "//secret///abracadabra").unwrap();
        let expected_print = r#""seed_name":"Alice","network_title":"Westend","network_logo":"westend","suggested_derivation":"//secret///abracadabra","collision":{"base58":"5EkMjdgyuHqnWA9oWXUoFRaMwMUgMJ1ik9KtMpPNuTuZTi2t","path":"//secret","has_pwd":true,"identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000038b49444154789ced99bd6ddc4010462d400d9c1bb02356e0d8210b70a60a95b900868e5d0123bb015f03079c777018e1bbd5707e76678903c41750c12e66debe48909eaed7eba78fce2e11cee773f392d3e9f4547e0c6548849e475b8c88921a61e4e36b3263a4446879fcd71f5fcaf79e3f3fff966f8c8c185d115a1e4f4801989610444f8ce6082302307b87688ad01a801819816809118ad0f3786674042612c31d212300b15704c21bc2152112e0f5f2bb7cef7979fe56be37a211ac79169e106684de000c8a6b21ac000cceb3b042a445d08419149742780330384fa32b82370091294d64cfd3426c46880420b2a5b3e7115b21c408d10044b674f63c460ae18e70f9f75abef73c7f7e29df1b51e9e5f27edefcdc3ecff2635c11bc01185ca489a3b01480f186c0795e3fa60e6146d01630b8481247612d006385c079513f428d5007205a9668442358b4fa61882342e12d821480685db2c5a3442038447a845578e4048f5a84f39a19eeafc2fd09cea37e883b02a12dc205923083e28b726f867bab726f827b5ebf9a5004425a840b346106c517e1fe0ce7ab705e33c17dcb4fe22e8215c04354da227bde1614e28870440846b07e738b4aff12ee7f87f3e83ccb6f0b77046901838b347114960230de1038cfeb27e18aa02d607091248ec25a00c60a81f3a27e35432258442358f4fa1d110a4784c290088bf0c8191e158d60cd8bfad5b82210da225c20093328ae85b0023038cfeb27e18e40488b708126cca0b814c21b80c17996df16a1081651698bec795b1c110a4784c25b04c20a61fdef302abd0af727385f84f39a19ee5b7e1214a0fcb8fd3d81d022480b185ca489a3f0aadc9be0dea2dc9be19ed7afc61d415bc0e022491c8557e1bc6682fb8b707f86f3a81f322c82c52a3caa6682475af4f8bd8b4048217a96483c4a040e401c110a7711883a447489f59b5b3482352fea476000c28c40688b708124cca0b816c20ac0e03caf1f634620bc21708126cca0b814c21b80c179961f530720dc112ca2d216d9f3187704221a225b3a7b1e2105203623109110d9d2d9f3b602106a04c21b222a6dfdef303a4f430b40a44520347114960230de1038cfa23b02d11b0285b5008c1502e7595801085704221242231aa1074f00c21d81c808b157046f00221481e989313a42e4f14c5304a235c4c8082d0188e608c488107b0720ba22302d31a4102d017a1ecfa444605a62b492f178263502333246e6e39921116a7aa28c7874cd2e111e9dff30b1bfacbc80bf400000000049454e44ae426082","seed_name":"Alice"}"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn westend_network_details() {
        let dbname = "for_tests/westend_network_details";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = network_details_by_key(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r##""base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"public_key":"","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea20000002e49444154789cedcd410100200c0021ed1f7ab6381f8302dc99393f8833e28c3823ce8833e28c3823ce8833fbe20724cf59c50a861d5c0000000049454e44ae426082","encryption":"none"}},"meta":[{"spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce","meta_id_pic":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a7b49444154789c7557797454e515ff7d6f9b7933938490b0c4844d88249ab21920090812a0543d08882d52e45841ed1f504e59542a15b5156dcf11bb0922682d88d6a341b11e721009a8c510031494352cb22621404226b3bc99795b7f6f222db57a73be33f3de7cdffdddfbbbcb77235cd7c5f788cc6573a5c5b6ed5b4dd3bc2b659a6385eb1464f8f99b00e249c8a623b56aaa52abaa6ab5a2289f7adbb93c51b82caeff93ef03fecf81542a3533994c2eb02cabcc715c68fcc53005aa1be4f4e7c4420bf9992e62295a2a09c8b27c4cd3b4753e9f6fb5102241159e030ed7ff007d173055c3b26cbb341e8bbd4e2f4b6c6e11fca35e5b928498bbc927de3fa2b87cc6a09e8ea89a95707375d74d5ade1610cf85aa4a97743d308f4654511f5fa7859a3ae5dbc069507af8503c1e5fe779e853252be45704bfca92e2a0e6a88bbbdfd091adf305d5357608bc7867120bc69b3049b022c1310c611b0654cf005dd75fe45a4cbd9ef04427f8f5c06950c3309ee55ae6728f9fa02ded4965cdb6f338772581e923bb6150611eca5669b402d048e245027f30378182a88b575e5790110466ceb2507cb3e3c4e370092ed3eb0f42a1d054eaf7240d7e0d380dea791a8b45d709213126a44b96a43b56ecc3ae2f2e017ea270ebf615a5d87b350fcfd608b88ec0bd432cfc6a98893b27e9686af2de01030a1d6cd99a4056b6e3dab6b05cd751fd7e7d552818984f1c2a82ed01a7bfd8b6551aee88edf1debbaeed047c9af4d59936542cab834ed714462f1c4e62d6f8026c5c3204074f5b882505ca8a1cac592d61f1421ff2f25ccf365c6a1158b32e8159f70391ab5e420acb711c45f8f5397e9fef756e513c607ebae8e8881f74ecd6125df9d4e276c5457fb4c54760c4d2cfd1d814831654916a4be0d54787616049084f7c7e142603fff321052809f7c6b8892a74bfe701d591cc5d9f98a893cfe14f7b2f20a0cacef2d21ba5f105b909a107fb2ab2d492064ea5923323d1c45b21f53d4b911a14d70dd0eb38146d32aaffd5170bd77f89e6b0859f8dee8905f7f4c1f0aa3a84a3ac142fd004df71df709cdcd2032bff20339b5d2c7dcc46c1844bb87d2309546885e5202f3b68d64d1ba1f6c8ca78c1e7d71f4d03774462bb6d2b5c96a16ea0bd8eec42625d18485a8510be9948c4dad01636d0bf5f1eb69e68c21d9bbf40a6aed131d21f4be0f9d1c5583aba10874f47a1a8c0c082107ebdf318567c710259411f8bd8452461bad5934bc5c45edd5aa1078a846599b7767474eca56908289b5822c7e884ce48c76148d36036e522f4e587501c0bc96e37e272f1188cdc5c8fe6d68e4e8f65197b7e5c81eaf62b78e9c2593ae7e2f7850351a4043166532d20714fa7c7a89f5e6667a8b2ec0b041e112c9de5f178ec192154f689b09ca5d6535f0409bb0f628912647eb21122da0657f5b13f4620954fc6671903b072ff19842d174b8a7b203b4bc6d8da3a1ac1e22083240207468fc2a1e608d61e3d830c55c5e221fd51deb38b134d5992dfa76d11f4b6c634ad4a08c709c921e954fc0a4ec71b3128bb18dde361a0e635488ac6c223fd892892034aa18d9a82338777219a4a62484905569fb9807907f621d3af33b3808e44021b860ec6ecde7db0abc54277bfc04d19365a138ed7ed04e58a0887c30d29cbbca98b1a743734ee10f38fae8561c57153a817de2f5d8ea2afeae09cda0be1814b32cc0973b0e6bd95f8c7bb2fa59d2b2f1b8f398bd7e3b67dfbd11289d063c638bb0bb60f1f8105fb74545f94d32d6041610acb8a2d4459330487680fb71f716db738e9984ee9ee25d205e322a9c940c4b88cb937de83578b1e41f8c42e2809034abf4138dada8425f347c1e797a030932e5f4ee2a927d7a0efd8d95877fc109951b0884958d39c8907ea15e407c0b0316569d0f6db1328ca7060d8dcd6dede7e4438280edb716768ed22e9aa194586a2239c68c58c8209786be86308371e869c8843eb5b82830df558ba703c42193ee68d42e018162f7e11774c5b884da78e921505b3f273f097e31a161ed0501070bddc42849e56df964069570731cb030eb7379869aa43eed327df162b8eaf6794d821e9f596912b30eefc455807b71384dd3f988df898fbb0e28f0fe1b31d1f9333d25ad8074fadd88619a71ab1b7a59927057e94df136b8a8761ca2e0d5fb6731365561f0baf94a690a41182cf5e72d5f2ea2b67c01d9fa44935ad0d3814398771dd06a1540ec0d9b69631619058368877c02e198bf8cd1538f4cfbf73083030b67c2ade8983b4d622f44d72459329bc3f7c28c677cdc7e666811e3ea0222749cf5dd634ed15222278fdbdc0925acc8b81e564c95dd513046947caee8b48b42bb276fe1530537015360c023b83c7e17cff726c387611572d07f30b7371d4b88aa9f5fbc032612cd9094d0b3b2bca90edc8f85bc33974f1a9b87f606ff4d07d4ed2b6254d553f13f4b63212e9a8612b777579ab50a47a1e56c9a28b843413ee0903fe03d5a4876675c943a27c0a2a3f3a8c03679a69ba84accc20eaa697e3c9af4fa2aaa98959ede297fdfb636ecf1b30ac6a374cc364663918da2b07dbef1eeedd4ab21e083cce4f47e6ad74c8b1234519ea7a876d5ee2154f3ae26c994570d41908188d48c4e208dcd00bdb4eb761d2e63a84e8852c44ba65ae1c538245157d51db72c5b30565ddbae1e94f4ee099ba636c991cce684c3469ba5bef1e2e2af3730cb6cc1f10989e25128b48f9ca90f6b629a3896d9eb1723b58bb95387ca114cbab8ee1ebcb29fca2320fa38665e196b73e67a41811a2d8f114aa7e320cddcfe663e50b0acbcc6596db38dff302ee7d673f94a0e6cd510868aab567fa48a530a7cb4655d767a781797bfbdbc3c6590967bbebca36f67443b2d19bf53709954fedc7feaf2e03bc16f9021f3e3d021743261edfe5f574170f96e4e3e1dc81183fc1c7cb867cb166f37abad8b923853f9f6bc0ba834d9c5484fb9be103f0f0cd05c2f5076ed114e58807cc060b8bd3e4bdbc1adf958449669252c09729f69feec0a865b508fa954e5a3908fcb4b2006f3e3a04271a63a0b318dc4fc5cbab142c59a4c11b043c69e120f0ca5aee9dede064638a838430737c8aeaa8bee702babe8c5bae0d020c2a8d358cf84ac3482c82904d4938aa653b287fa21e274fb26707691fbbc09bcb0643cfe98327b74acc7c090f552431216463e2449dde0a2f8f100ab9f8a8c640df7e24cf94532c228d6d6e7b5646c644e278c251ac13d8ab694fdc6834ba39954a4ee19d6c057db2b4f75458fa6dd52934b525316d4437ccf9e10054bc4c5a3907f8684bd810d839cfc0c5dd02abd6aa08b245ce7d9833d85db61b8b713617aeaac8f2f1ccccccc1a273ce66fac1b906ec89079e7e60a2bdc4849be73d047d8ac96b57314c576486043e3e024c7dc3875c02907d3447047e37298985134c74440538f0bb5ece45a31e8bae50556d7b28149cfc0d28df75fe97713db02754d5099e4ca61e24f5ab4dcbf6f3104104897545dc1262ea1bbad87b81d6504d6ec0c5870fb0f9e7380e73cfa502d99bc7158e3c7ebfff39ced45e4c3de1ee4e504fbe0dec09cfa6e9b03919f6e0c8bb84eb417ecfa13eb03a7086fdf7b53d2aff8501660cb250d6db4e539fbeee8464689aba89a0cfcbb24c7ed2e2e963f4ff2bdf057c4d1841c688c23db9ccfa69ec725338348c54653737a4751e4c98eca49614d11479bfa2aa5b38bc6fe285728a3f79e2e9b0b9d27baf977f03ce7bfeaa1a7a59e70000000049454e44ae426082"},{"spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf","meta_id_pic":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a5c49444154789c7557097094e5197efe6b77ffddcd6e0e12926042141209b7805c2d62b0161404416c45eb50aa838822d3227514eb206391ce20d54e2b531c04045b6ba1141c4530448e1a8d342a060202f180841c24d9237bfe679fffa7d86aedbbfbcdfec7f7bdcff73eeff71e2bd8b68dff231287c9e18a699a63755d9fc931d5b6edab0445e13b01b6a14b54d2232b4abda2286fc9b27cd899cee188cc6170fc8f70cd77027fbd40d3b405d96cf611c330263a730551846d9ad03a2ebabf9efec5107d2aaf0d08820849924e7b3c9e97bc5eef8b822064a8c231c0e2f806d07701bba0b4705c3299dc62e8fa70678e2070b220984416e28d0d42f662abfb500ee70ab913be6f0b5eaf0dcb14a98d3400a22c77f955f5216e6227f571b52b7c7359be0dec82d2c2fb53a9d44bb66d41941443f2f8380f1231916e3f8f58fd51081e0f710598e93472465e077fe510d83aad1645cbb64cd3320d85baa0aaea068e15bc74c4d9800b4885eeaf232e683a9d7e86639533439015434f46e5aee3ef408b7723aff27a844a87a3a7ee2d828a344b8495c92077d2148821959b3a074152e0eb3710b23fec6c80ea6d8956ef090683b753a5238e6afb0ab00bea589a48265f1205c1729e8b922c7eb66b2d7acf3640527c9c02542f781a725641a2f938ef00ef803204aa8722da7c08a696e61352e3cb41ded01be95d0f95db8665590aadfe43c0ef7f187cca613ac0ee050fcfb8645fdf3182825bb5648f4f8c777e8993afae82a838b48ad05331148dbc098367af40a6a395478687abb004c9b66624bef808a247a52a3ed633c819341eb9fdaf86a16b244620b82dcbaafa339fd7bb8553640798bf40a2afaf2996c90e3f194d1969c3948bfd3e540544346d7f1ce9de8b907d0168895e54deb60279c3c6201efdd8254df55d0d590f22d27c90f78e0d363f024ab8c123310defb4b6c343bfcfaf18208ec80b673cc16085248a9d2eb01332a964e24fef5fea33da925999139165788c2f2d4251cf199c7b771bcc4c1cf9832763c00df31189d431aed3544f3f13262fbf065a7784e31cef44848b2bd1e22dc4eac64f411068a68552bfaaff7a4cb552100caef7aaea4a1798d6be9fc86a13df6d8f9a74aee4d0ad599cec533065501992915ee8b15ee4545c8b54ec4b447a0ed1ff5e82d2472669cd1985a2e231e83cff052cbaa4ac7c20b67fda8c573f6f45be570121d0a71bf6af460f1146e5e7f62881c01041378cb17df1f83f1db0face185a931978193659ee724259110abf68c1994dafc0ce6a088f1e8ef28577229a380a2d1ba5df250e11c5c533b0e3779bb17bdb566ec4c4b2c71f47e5dc3bf0e89106289274d908bf8a7563879b3e499454bf7fb1c0d0798a31fb34293153ba2e9deed391b40414fb445c23db38f1c433c8b47742e242ad378241cb96a060fa3868b166c6848940b00a273e6cc1e25b67c01f088027987403af1eac453b0fde9b2dad087864cc2bef8fca508e95314dd1ebf1bc29c4e3f183ccbfd338d752bcaaa8c7ba908e74205456855447044d2b9f644af4f2dc70e791284a6ffd21aa962d46d3b10f90c9a631e97b35d8bd650b9e5af2000a4b4ae8721bdd1d1d58bb651be6dcb3003d5d2df0282a5369081953b7791e9977846e21168b7dc6b458c5536b5f3a7948f8b27633cc6c0a6a411986fc78152e6cde89ce03b504f731a1c818bd6e35361f78053bb6bec0b0b3515333138f3ef22c96cc9c85cfcf9c71ad1d39761c36beb11749a309a9741b9124e4f8ab100e0ce5be9c1220408845a3cd3c51d5b6a95b27b63f266669b1ac06918d5d42e9c4b91878e322b4eddc836c248eab6e9a820b521af7de3315aa3f005956d0d979111b9edb81b14327e16f2fbfcc981531ebde85c829d4d17ee92814394030931bb251945bc3fb30ef0dfb32b08d6a5a699d7865a5686493cc52f42763b670c43454ce66b2d0cec3b4b3c80d56e0a38606dcbfe416844279a44f4247672bd63cb511f3efbc0f7b6a77317c24fc60d22db4f42cba638d90253f81e8775a5a983b95b4e73bd7048e4549b5e152dd5affba70e1f00e52c1d4a2e6a0faced540288b78a491ec88b43088bcbca978e2c987f1f681dda410183674145ef8ed5ff0c8d34b51f75e1de907e6ce988b3faedd848eee43c8ea11875804fc1528085f4f50a742521d0f573d0fd7243adc126545ecbb781aa9ee0b08978d80279c8beeae7d9cc5a5f49369a610ca194e9a87a1be613fad4a62fab4dbb1fff0db58b4f26e14f3143b79e1526f17b6ad7f1db367dc829ec43986941fa2990f7a93baf815843e81a1b49e21b582e5cc646197749e482b95825c500829cf8fde9e5a5a61d077320c23498bc7424bf7c7be3fef4022de873b7eba0827db4ee3470fce46617e11e75a88f7c5b077eb3e94fbaec5e1d78f21105631f18e11c8e917b08cacc1d4af1c1168ed345a7d505414bbef934621d5f219434726193672274c8595a791ea63bc0714fa27141a8f65f3eec207470ebb25ada4bc1c2f1f3880df6c5d87ddfbfe4a564c3cb4683916cf5c8ef5f76c422a9a66a1b05035f16a2cd978178f932df9fdfec7041622299e489cd053a921d1a37516ad169d98b5b259784b0710fc06e8995e5a9b466efe40341e398aa5736621949b47164474b5b7e389e79fc74f962ec57b0dff208d12268f9f843736d662ef863a848b72400ca6da8cbd64d302a1f2fa81698fe81de1e6ea4c26f30b52fe5cecfda3ba11e951048f17563a85e0b051f05d53c192d7c4d84ed3ef9568efcd62e18d6482ca647621b19e6e6c786d27a6cc9a8c68e404816504c38371aaae1b9b97bf86406e8085de802fe831966f5f28f71f58b8c3e7f5ddeb0273f8e2c9e457d9aecea244d3c796ad69a2421fe78c1a83d8d97aa6c70ed2ef8165682819331d6fec39802deb9e758a3866dc75371e58f573b475ee87696924df7643a8b4e866ec7fb101c7f6b242c9a23dfdc12998386fb4406b87c98adcec003bae32581ae727d369d74996ae89923f2818c928a227eb18499cc2936de9a49f6d4df97535387fe102e35b4355c5205cbc741cbdf146c6f5e546c0a95805e1f108d3f268572f08a4ab61af224359abfad5559cf275232071980edd0eeda224e9f48b5bcfa2a70e414f44d8352a3c241a8a874cc607ba1f3bbf6a85cef7d38afa61f65501b4f5becb48217bfc8882c2645103590c709dc0e2647998586a43a1d0cdc47144b802cce872c54e24127f67ef35c7695768a9a82723628ab16d6a1904f24b902eb806bffcf05324d9512a2c9f699ee235634662b03f8aeec459b6a23202ea20a8de52aa36d9f2580a9bfc33041dc5f875fa6c91c3ba02ec8803eeded0f2dfd3f2879c070c2d9dfb932dcb14fcecbd8e77f760cd27a71052143e067a58a7efabacc0bc8a72c448bd445b986e98fe0d89ba058f47a90d0482b7fd1bd46596e31bedad235475199c3e5fc40dbc4897fbb8886f0493ab84b46909ab3f39259c8d271c106e40c69aeb86a23ca85ae494a07000395d84cfe75bcbeed2f1a9235c7e19d4916f033bc2b52e1d2669ea4fda1fe558c4eb0267a69731de95c9e2edd64e7616266e282e4475388894c13f197ccfd84ef33fd42e823ecb22d2cc478e38fa982fff23df057c45640ea778f28cd9fdc8c05c66b9396c9526f0453f9f2cb90bd9c80959cbeaa3233f26e09b6cde7711bc85af1ce154d74a77ee7fcbbf00a20127bd2177af5c0000000049454e44ae426082"}]"##;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn westend_9010_metadata_details() {
        let dbname = "for_tests/westend_9010_metadata_details";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = metadata_details(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), 9010).unwrap();
        let expected_print = r#""name":"westend","version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf","meta_id_pic":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a5c49444154789c7557097094e5197efe6b77ffddcd6e0e12926042141209b7805c2d62b0161404416c45eb50aa838822d3227514eb206391ce20d54e2b531c04045b6ba1141c4530448e1a8d342a060202f180841c24d9237bfe679fffa7d86aedbbfbcdfec7f7bdcff73eeff71e2bd8b68dff231287c9e18a699a63755d9fc931d5b6edab0445e13b01b6a14b54d2232b4abda2286fc9b27cd899cee188cc6170fc8f70cd77027fbd40d3b405d96cf611c330263a730551846d9ad03a2ebabf9efec5107d2aaf0d08820849924e7b3c9e97bc5eef8b822064a8c231c0e2f806d07701bba0b4705c3299dc62e8fa70678e2070b220984416e28d0d42f662abfb500ee70ab913be6f0b5eaf0dcb14a98d3400a22c77f955f5216e6227f571b52b7c7359be0dec82d2c2fb53a9d44bb66d41941443f2f8380f1231916e3f8f58fd51081e0f710598e93472465e077fe510d83aad1645cbb64cd3320d85baa0aaea068e15bc74c4d9800b4885eeaf232e683a9d7e86639533439015434f46e5aee3ef408b7723aff27a844a87a3a7ee2d828a344b8495c92077d2148821959b3a074152e0eb3710b23fec6c80ea6d8956ef090683b753a5238e6afb0ab00bea589a48265f1205c1729e8b922c7eb66b2d7acf3640527c9c02542f781a725641a2f938ef00ef803204aa8722da7c08a696e61352e3cb41ded01be95d0f95db8665590aadfe43c0ef7f187cca613ac0ee050fcfb8645fdf3182825bb5648f4f8c777e8993afae82a838b48ad05331148dbc098367af40a6a395478687abb004c9b66624bef808a247a52a3ed633c819341eb9fdaf86a16b244620b82dcbaafa339fd7bb8553640798bf40a2afaf2996c90e3f194d1969c3948bfd3e540544346d7f1ce9de8b907d0168895e54deb60279c3c6201efdd8254df55d0d590f22d27c90f78e0d363f024ab8c123310defb4b6c343bfcfaf18208ec80b673cc16085248a9d2eb01332a964e24fef5fea33da925999139165788c2f2d4251cf199c7b771bcc4c1cf9832763c00df31189d431aed3544f3f13262fbf065a7784e31cef44848b2bd1e22dc4eac64f411068a68552bfaaff7a4cb552100caef7aaea4a1798d6be9fc86a13df6d8f9a74aee4d0ad599cec533065501992915ee8b15ee4545c8b54ec4b447a0ed1ff5e82d2472669cd1985a2e231e83cff052cbaa4ac7c20b67fda8c573f6f45be570121d0a71bf6af460f1146e5e7f62881c01041378cb17df1f83f1db0face185a931978193659ee724259110abf68c1994dafc0ce6a088f1e8ef28577229a380a2d1ba5df250e11c5c533b0e3779bb17bdb566ec4c4b2c71f47e5dc3bf0e89106289274d908bf8a7563879b3e499454bf7fb1c0d0798a31fb34293153ba2e9deed391b40414fb445c23db38f1c433c8b47742e242ad378241cb96a060fa3868b166c6848940b00a273e6cc1e25b67c01f088027987403af1eac453b0fde9b2dad087864cc2bef8fca508e95314dd1ebf1bc29c4e3f183ccbfd338d752bcaaa8c7ba908e74205456855447044d2b9f644af4f2dc70e791284a6ffd21aa962d46d3b10f90c9a631e97b35d8bd650b9e5af2000a4b4ae8721bdd1d1d58bb651be6dcb3003d5d2df0282a5369081953b7791e9977846e21168b7dc6b458c5536b5f3a7948f8b27633cc6c0a6a411986fc78152e6cde89ce03b504f731a1c818bd6e35361f78053bb6bec0b0b3515333138f3ef22c96cc9c85cfcf9c71ad1d39761c36beb11749a309a9741b9124e4f8ab100e0ce5be9c1220408845a3cd3c51d5b6a95b27b63f266669b1ac06918d5d42e9c4b91878e322b4eddc836c248eab6e9a820b521af7de3315aa3f005956d0d979111b9edb81b14327e16f2fbfcc981531ebde85c829d4d17ee92814394030931bb251945bc3fb30ef0dfb32b08d6a5a699d7865a5686493cc52f42763b670c43454ce66b2d0cec3b4b3c80d56e0a38606dcbfe416844279a44f4247672bd63cb511f3efbc0f7b6a77317c24fc60d22db4f42cba638d90253f81e8775a5a983b95b4e73bd7048e4549b5e152dd5affba70e1f00e52c1d4a2e6a0faced540288b78a491ec88b43088bcbca978e2c987f1f681dda410183674145ef8ed5ff0c8d34b51f75e1de907e6ce988b3faedd848eee43c8ea11875804fc1528085f4f50a742521d0f573d0fd7243adc126545ecbb781aa9ee0b08978d80279c8beeae7d9cc5a5f49369a610ca194e9a87a1be613fad4a62fab4dbb1fff0db58b4f26e14f3143b79e1526f17b6ad7f1db367dc829ec43986941fa2990f7a93baf815843e81a1b49e21b582e5cc646197749e482b95825c500829cf8fde9e5a5a61d077320c23498bc7424bf7c7be3fef4022de873b7eba0827db4ee3470fce46617e11e75a88f7c5b077eb3e94fbaec5e1d78f21105631f18e11c8e917b08cacc1d4af1c1168ed345a7d505414bbef934621d5f219434726193672274c8595a791ea63bc0714fa27141a8f65f3eec207470ebb25ada4bc1c2f1f3880df6c5d87ddfbfe4a564c3cb4683916cf5c8ef5f76c422a9a66a1b05035f16a2cd978178f932df9fdfec7041622299e489cd053a921d1a37516ad169d98b5b259784b0710fc06e8995e5a9b466efe40341e398aa5736621949b47164474b5b7e389e79fc74f962ec57b0dff208d12268f9f843736d662ef863a848b72400ca6da8cbd64d302a1f2fa81698fe81de1e6ea4c26f30b52fe5cecfda3ba11e951048f17563a85e0b051f05d53c192d7c4d84ed3ef9568efcd62e18d6482ca647621b19e6e6c786d27a6cc9a8c68e404816504c38371aaae1b9b97bf86406e8085de802fe831966f5f28f71f58b8c3e7f5ddeb0273f8e2c9e457d9aecea244d3c796ad69a2421fe78c1a83d8d97aa6c70ed2ef8165682819331d6fec39802deb9e758a3866dc75371e58f573b475ee87696924df7643a8b4e866ec7fb101c7f6b242c9a23dfdc12998386fb4406b87c98adcec003bae32581ae727d369d74996ae89923f2818c928a227eb18499cc2936de9a49f6d4df97535387fe102e35b4355c5205cbc741cbdf146c6f5e546c0a95805e1f108d3f268572f08a4ab61af224359abfad5559cf275232071980edd0eeda224e9f48b5bcfa2a70e414f44d8352a3c241a8a874cc607ba1f3bbf6a85cef7d38afa61f65501b4f5becb48217bfc8882c2645103590c709dc0e2647998586a43a1d0cdc47144b802cce872c54e24127f67ef35c7695768a9a82723628ab16d6a1904f24b902eb806bffcf05324d9512a2c9f699ee235634662b03f8aeec459b6a23202ea20a8de52aa36d9f2580a9bfc33041dc5f875fa6c91c3ba02ec8803eeded0f2dfd3f2879c070c2d9dfb932dcb14fcecbd8e77f760cd27a71052143e067a58a7efabacc0bc8a72c448bd445b986e98fe0d89ba058f47a90d0482b7fd1bd46596e31bedad235475199c3e5fc40dbc4897fbb8886f0493ab84b46909ab3f39259c8d271c106e40c69aeb86a23ca85ae494a07000395d84cfe75bcbeed2f1a9235c7e19d4916f033bc2b52e1d2669ea4fda1fe558c4eb0267a69731de95c9e2edd64e7616266e282e4475388894c13f197ccfd84ef33fd42e823ecb22d2cc478e38fa982fff23df057c45640ea778f28cd9fdc8c05c66b9396c9526f0453f9f2cb90bd9c80959cbeaa3233f26e09b6cde7711bc85af1ce154d74a77ee7fcbbf00a20127bd2177af5c0000000049454e44ae426082","networks":[{"title":"Westend","logo":"westend","order":2,"current_on_screen":true}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn types_status_and_history() {
        let dbname = "for_tests/types_status_and_history";
        populate_cold (dbname, Verifier(None)).unwrap();
        
        let print = show_types_status(dbname).unwrap();
        let expected_print = r#""types_on_file":true,"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a4849444154789c7557097454e515fefeff2df3de643209061216c105a42c611110a1142d540564aba2455a01414a5daac70aca394504ab60f5803dada82c4510c58a15ab28e00212ad6815152b1221a0a0b24842969979b3bcbddf1b856aabf7e49e99fcefbefbddfdbf23c230c40f9042f6c945f27dbfbfebbaa31dc7bd5088f0f4d2d2123e13c8e5f28aeb7a8dbaaebfa569da6655555f8fc4c911a9648ffc7ff443c0a75e701c67926ddb377b9e37280802c4740db9bc8d0d2fbe8b3c3fc78ce88f4e1d5ac3ca16a02892aceca5112b63b1d8c342880255440e04e4ef007d1f7011941e0ec866b3ab3dcfad0665a824623ffa3661ea12b1fd851d7c51a263f55962db3fee0cdb5595858c868cc4c25040aa6a7ddc346fa411cf509f2047c477be260a9dfa1e5111941ecec8e5722ba367422a5ea8c484ef074a59690c35351f60c4a879d05b95d23b89fcb113b8ffcfd7e3b69b27a0be390755550211b83e7c578b349ba6f9007916bf461419101d7f07b8089acfe7ef21cfa5e57caa7ac2b554e3f8fb80dd02b55d3576d5273170e82d0ca084cab07b0d4d786cfd7c4c19710672fb7740d163b0db9c0b3f5e192070c3208412d3f5e71389c4cfa931a248737812b8085a281466642c6ba52225d31946e192c9bdeba036ed25908e90390ecf9b89db97be8b258bd7b384028c9b30148f3f3019c98f57d1b814d550ab598154f5b5f084114a117a54a61986f9502251f25b3e8e72ee47c0c52facd8018e5dd8a9eb2a0bca0d62665ce6ea0f7dad506a345182dec3a9ec87a0d72ff1d1ae7d2cb202060cec03ed700d62073622d493544597286775be1cfa5983e11732908a1a81ab42aad30dc3584d1135024640769dc2ee7dfb8f54cf9ab7d63b72ac491d336200e6cf1a0773d732c8fc098a9a809b46b6f32414da57a03c51c3d20a90ca9d0bd9588164ed72403270d415e90b07dd84651bf660f9aacd300c3d5830e74a79f14f7b1784d4cf641d1c2f02b39826799ef3e4c82b17796f6e795b45a214b052f8e35f6661ce941e70763d0b2d28c0a9a846fecc41d0932b11224556c050c2c9ce80fe653dcc63afc30f99fb6e1761536d0c6346ce01341a5370d0a64b07f7bdd7eed5da56562cd663b1db4494cb4221f7f6e1a32706f51e7a9befb99e12853bd798c1a5e3cfc7c6a7e6a3b9b11e9ac8c0573a336c7ba097aea1677130a8fccbc0cb8d826b5f0425fc1c9e27d1bab223eeb8672d16ce7f02f176a7b13642e49b32e1a68df3c588e17d1a4368dd0473db3f9d4ebfc77060c4150bf1e6961d40097395a5c77f9a8d39b7f440da7d1a8ae623b4bbb0ec87414bae86940df498de10dc49ff8651de0f25be13010b2e2e2fc14baf1818339a1eab2c21db41eb2e1df0c1f6fbfc64a9a9c462e64cc1d6b9d3ca66ef8a1b31ff8bc3f5cadc7b3760dfa7c771d9a87e9873d32510252bd8cb8d04300891829b9d88c0eb08b5ec259e39f0ad0be891895872390dd17916e51888793762f5ba7d7864edab284fc6592fe331f8bcae41c62a48c3886d12f4761bc7e170b64aa0c713320e0b5efa38d4566723ed1c879e7884caa890b983b0e03be7c1cd4d803ceeb00109d2d6805af226346323ebaa94b2141369d834b0441b0891ae83d013708d4a6433995072b6914e88542ab58f89ed1aaa66a8d5ef12f1cf3643096c38466be4ba4f815ab1158afa0ed5b18fa1c1c95d03b1a52be43b3484a331ecea2398d000add55f5968ac7e4a054127b8a9c930f63e8f586a1fc554e4db0f817dc670088f060bced554aaa5d6f383ee32f483e4ee1552da2da01110fcb4db0d65fb8c609a765036cfae3f07f8fc6c28ab58581a31983ea4017f820d0c3c0cc5afa5d28083fec7d08fd5a1e4c0d3eced321e45f74d8074f5afe1c52bf9bf1b7e031c7697be1d94ed5e2e859747a8c4209c747158585d26523f15ca1c027926c4c132286b086c52175b096981605c01c1600bd239c4b38077626f184776207ee845022729c6cbc9b791ee391d5ee2748a3811f07f436d7eb95dc40ebdca5c81913591ed3103a8fc049afe320f149eb762055f03b9be23642dc32c78dc368477750a7afbb50cf5673c20b0d7175ef3a548ec790cd23aca33ce9eaa7ec875bd9c8f23ef196a16d75b9eeb0ee63d14485593e5e1576cf80604e65968a6ed66f2210a46f95419ee343cf727705bc640eee5994bf07342c8aa77a0ebcf32bb09ca52130b346f4d4649d019b1c2276ca9042cbd23bbcaa7718415222378fd2de616314bd314dfb65d65d9e36f604fdd515c36b23f5baa331c6d297b960511ea7ca185153dba58d96a790d8aed9419c25036719aad216409cfe854988711ccc48e7f8578747d0d5a95c571d3b461685b551e140a8ee41dfd86705c7778aa25b5ad2c190fa7dcb0543cb56a23a3ca04063ed63eb900575fa5c3f29fe1dd0b845e07b8d62fa0253670601ce001c38f24c37f2d54633b646c37d31140c785f8e0bdee1832ea76f88d699ae2a1d790bea8d9b8c0e7daa498667c0e4766a0e4f3b98f8f7dd5d4adf7d0d9b4c86583ebc835b460ec8421786edd3c3465be80aa66999e2e90e200bd7b94a02c3006350abf9b1b0bcf194663f6c3f505da9475c19d8b9ec0dd773cc69159c1db3380dd6c855b5ebc4bfcec82ea3c84d6ebe425716b2e9b5d3278e43c77dfce3a4d699d847fa201bfbb633a1e983b1af90f3743f72d14daf485d3ae2362a50fd35356203d96220b277f0d64bd01f3680dbc4085d17338d6be721853272e00cacb58332ec7ace9edaab95feddab9c3139a1e9b5c04e65a638481f3f9d6d73faabcf6e6654153b325870ded89471fbc015507ff06d1c4b0aa06c3efc0fad174385519c4e3dbe86fc83bb90f644b3f94ed59566c19ce4f045a1276ffeb317bd146ac59b78dd152c28573afc275d32e66b3a93d754dab8d8055a0b8675d11f8eedfb92dfaa97456b66b5fc5da3b0af3c315dc806284605879c117b8d6c8bebf4273c36138ae8db61d3ac3addb8ef8c117587f49aa62029c0cf25daf80e83410c70e1f65c56b6e45ab84168472513c1e9f4b113502e6279b94f65a567689eb3ab7b2c25dc7f1b976f828af5dcd5e3c425183b7530ea2ff54ac7cad05f3fef038d75b1737cc1c8505d77152bdbf8ca645bac81c40294e29576f054383e3076c09c8ad6565c98b2910116bec6b6076579142cbb29e2bd8f67885338c2b8f54aca3d23cf206476806a2aa07ea444ff41e720b2f921caf208d932b87e75fbe0fe37a712138f04fbea2a350753e9cd3ba859c505c79428d4b7e5d3299ecc3fe2d1043928393c01145e0c57fd8db4bb9f8dd181d40d13958851a78ae282b4fe295577762ccd805305880bc69903bd288858b67e0f7b326a2beb185f9549915e10bdf56a85a309f5b4b1289b1df8016234b66eff3e9b728c22a1ef017c4341af070e0fb062f131e0a5f559876cb164347cf1707ff5d472905b222891d5bee469f9e9d825cdee156290818f00e57b86b198b4cd38c721ad129d088fe173822c214c3c1f60baa5874b3c9d3f8bd826143498981cf0e1dc7832bb6b0a21d4c9d74212e18dc1d29863c8a0039af69fa0682decb9f33b5d41351a42f209fa2ef033e49c56a27731a85ad1981cbb8268de7a43b3fa6a9ad4b1266f145bb608b6cceceb07277a9aab689e3708394f2533e8a28d2e1938bb2dfa6ff0031700bb6b1537e0b0000000049454e44ae426082""#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        
        remove_types_info(dbname).unwrap();
        let print = show_types_status(dbname).unwrap();
        let expected_print = r#""types_on_file":false"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        
        let history_printed = print_history(dbname).unwrap();
        let expected_element = r#"{"event":"types_removed","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a4849444154789c7557097454e515fefeff2df3de643209061216c105a42c611110a1142d540564aba2455a01414a5daac70aca394504ab60f5803dada82c4510c58a15ab28e00212ad6815152b1221a0a0b24842969979b3bcbddf1b856aabf7e49e99fcefbefbddfdbf23c230c40f9042f6c945f27dbfbfebbaa31dc7bd5088f0f4d2d2123e13c8e5f28aeb7a8dbaaebfa569da6655555f8fc4c911a9648ffc7ff443c0a75e701c67926ddb377b9e37280802c4740db9bc8d0d2fbe8b3c3fc78ce88f4e1d5ac3ca16a02892aceca5112b63b1d8c342880255440e04e4ef007d1f7011941e0ec866b3ab3dcfad0665a824623ffa3661ea12b1fd851d7c51a263f55962db3fee0cdb5595858c868cc4c25040aa6a7ddc346fa411cf509f2047c477be260a9dfa1e5111941ecec8e5722ba367422a5ea8c484ef074a59690c35351f60c4a879d05b95d23b89fcb113b8ffcfd7e3b69b27a0be390755550211b83e7c578b349ba6f9007916bf461419101d7f07b8089acfe7ef21cfa5e57caa7ac2b554e3f8fb80dd02b55d3576d5273170e82d0ca084cab07b0d4d786cfd7c4c19710672fb7740d163b0db9c0b3f5e192070c3208412d3f5e71389c4cfa931a248737812b8085a281466642c6ba52225d31946e192c9bdeba036ed25908e90390ecf9b89db97be8b258bd7b384028c9b30148f3f3019c98f57d1b814d550ab598154f5b5f084114a117a54a61986f9502251f25b3e8e72ee47c0c52facd8018e5dd8a9eb2a0bca0d62665ce6ea0f7dad506a345182dec3a9ec87a0d72ff1d1ae7d2cb202060cec03ed700d62073622d493544597286775be1cfa5983e11732908a1a81ab42aad30dc3584d1135024640769dc2ee7dfb8f54cf9ab7d63b72ac491d336200e6cf1a0773d732c8fc098a9a809b46b6f32414da57a03c51c3d20a90ca9d0bd9588164ed72403270d415e90b07dd84651bf660f9aacd300c3d5830e74a79f14f7b1784d4cf641d1c2f02b39826799ef3e4c82b17796f6e795b45a214b052f8e35f6661ce941e70763d0b2d28c0a9a846fecc41d0932b11224556c050c2c9ce80fe653dcc63afc30f99fb6e1761536d0c6346ce01341a5370d0a64b07f7bdd7eed5da56562cd663b1db4494cb4221f7f6e1a32706f51e7a9befb99e12853bd798c1a5e3cfc7c6a7e6a3b9b11e9ac8c0573a336c7ba097aea1677130a8fccbc0cb8d826b5f0425fc1c9e27d1bab223eeb8672d16ce7f02f176a7b13642e49b32e1a68df3c588e17d1a4368dd0473db3f9d4ebfc77060c4150bf1e6961d40097395a5c77f9a8d39b7f440da7d1a8ae623b4bbb0ec87414bae86940df498de10dc49ff8651de0f25be13010b2e2e2fc14baf1818339a1eab2c21db41eb2e1df0c1f6fbfc64a9a9c462e64cc1d6b9d3ca66ef8a1b31ff8bc3f5cadc7b3760dfa7c771d9a87e9873d32510252bd8cb8d04300891829b9d88c0eb08b5ec259e39f0ad0be891895872390dd17916e51888793762f5ba7d7864edab284fc6592fe331f8bcae41c62a48c3886d12f4761bc7e170b64aa0c713320e0b5efa38d4566723ed1c879e7884caa890b983b0e03be7c1cd4d803ceeb00109d2d6805af226346323ebaa94b2141369d834b0441b0891ae83d013708d4a6433995072b6914e88542ab58f89ed1aaa66a8d5ef12f1cf3643096c38466be4ba4f815ab1158afa0ed5b18fa1c1c95d03b1a52be43b3484a331ecea2398d000add55f5968ac7e4a054127b8a9c930f63e8f586a1fc554e4db0f817dc670088f060bced554aaa5d6f383ee32f483e4ee1552da2da01110fcb4db0d65fb8c609a765036cfae3f07f8fc6c28ab58581a31983ea4017f820d0c3c0cc5afa5d28083fec7d08fd5a1e4c0d3eced321e45f74d8074f5afe1c52bf9bf1b7e031c7697be1d94ed5e2e859747a8c4209c747158585d26523f15ca1c027926c4c132286b086c52175b096981605c01c1600bd239c4b38077626f184776207ee845022729c6cbc9b791ee391d5ee2748a3811f07f436d7eb95dc40ebdca5c81913591ed3103a8fc049afe320f149eb762055f03b9be23642dc32c78dc368477750a7afbb50cf5673c20b0d7175ef3a548ec790cd23aca33ce9eaa7ec875bd9c8f23ef196a16d75b9eeb0ee63d14485593e5e1576cf80604e65968a6ed66f2210a46f95419ee343cf727705bc640eee5994bf07342c8aa77a0ebcf32bb09ca52130b346f4d4649d019b1c2276ca9042cbd23bbcaa7718415222378fd2de616314bd314dfb65d65d9e36f604fdd515c36b23f5baa331c6d297b960511ea7ca185153dba58d96a790d8aed9419c25036719aad216409cfe854988711ccc48e7f8578747d0d5a95c571d3b461685b551e140a8ee41dfd86705c7778aa25b5ad2c190fa7dcb0543cb56a23a3ca04063ed63eb900575fa5c3f29fe1dd0b845e07b8d62fa0253670601ce001c38f24c37f2d54633b646c37d31140c785f8e0bdee1832ea76f88d699ae2a1d790bea8d9b8c0e7daa498667c0e4766a0e4f3b98f8f7dd5d4adf7d0d9b4c86583ebc835b460ec8421786edd3c3465be80aa66999e2e90e200bd7b94a02c3006350abf9b1b0bcf194663f6c3f505da9475c19d8b9ec0dd773cc69159c1db3380dd6c855b5ebc4bfcec82ea3c84d6ebe425716b2e9b5d3278e43c77dfce3a4d699d847fa201bfbb633a1e983b1af90f3743f72d14daf485d3ae2362a50fd35356203d96220b277f0d64bd01f3680dbc4085d17338d6be721853272e00cacb58332ec7ace9edaab95feddab9c3139a1e9b5c04e65a638481f3f9d6d73faabcf6e6654153b325870ded89471fbc015507ff06d1c4b0aa06c3efc0fad174385519c4e3dbe86fc83bb90f644b3f94ed59566c19ce4f045a1276ffeb317bd146ac59b78dd152c28573afc275d32e66b3a93d754dab8d8055a0b8675d11f8eedfb92dfaa97456b66b5fc5da3b0af3c315dc806284605879c117b8d6c8bebf4273c36138ae8db61d3ac3addb8ef8c117587f49aa62029c0cf25daf80e83410c70e1f65c56b6e45ab84168472513c1e9f4b113502e6279b94f65a567689eb3ab7b2c25dc7f1b976f828af5dcd5e3c425183b7530ea2ff54ac7cad05f3fef038d75b1737cc1c8505d77152bdbf8ca645bac81c40294e29576f054383e3076c09c8ad6565c98b2910116bec6b6076579142cbb29e2bd8f67885338c2b8f54aca3d23cf206476806a2aa07ea444ff41e720b2f921caf208d932b87e75fbe0fe37a712138f04fbea2a350753e9cd3ba859c505c79428d4b7e5d3299ecc3fe2d1043928393c01145e0c57fd8db4bb9f8dd181d40d13958851a78ae282b4fe295577762ccd805305880bc69903bd288858b67e0f7b326a2beb185f9549915e10bdf56a85a309f5b4b1289b1df8016234b66eff3e9b728c22a1ef017c4341af070e0fb062f131e0a5f559876cb164347cf1707ff5d472905b222891d5bee469f9e9d825cdee156290818f00e57b86b198b4cd38c721ad129d088fe173822c214c3c1f60baa5874b3c9d3f8bd826143498981cf0e1dc7832bb6b0a21d4c9d74212e18dc1d29863c8a0039af69fa0682decb9f33b5d41351a42f209fa2ef033e49c56a27731a85ad1981cbb8268de7a43b3fa6a9ad4b1266f145bb608b6cceceb07277a9aab689e3708394f2533e8a28d2e1938bb2dfa6ff0031700bb6b1537e0b0000000049454e44ae426082","verifier":{"public_key":"","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea20000002e49444154789cedcd410100200c0021ed1f7ab6381f8302dc99393f8833e28c3823ce8833e28c3823ce8833fbe20724cf59c50a861d5c0000000049454e44ae426082","encryption":"none"}}}"#;
        assert!(history_printed.contains(expected_element), "\nReceived history: \n{}", history_printed);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn path_is_known() {
        let dbname = "for_tests/path_is_known";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = dynamic_path_check (dbname, "Alice", "//Alice", "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e");
        let expected_print = r#""derivation_check":{"button_good":false,"collision":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","path":"//Alice","has_pwd":false,"identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","seed_name":"Alice"}}"#;
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
        let mut seed_draft = SeedDraft::new();
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
        let mut seed_draft = SeedDraft::new();
        seed_draft.added("fit", None);
        let print = seed_draft.print();
        let expected_print = r#"[{"order":0,"content":"fit"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
    }
    
    #[test]
    fn alice_recalls_seed_phrase_3() {
        let mut seed_draft = SeedDraft::new();
        seed_draft.added("obe", None);
        let print = seed_draft.print();
        let expected_print = r#"[{"order":0,"content":"obey"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
    }

}



