use hex;
use blake2_rfc::blake2b::blake2b;
use parity_scale_codec_derive::{Decode, Encode};
use sp_core;
use sp_runtime::MultiSigner;
use std::convert::TryInto;

use crate::{crypto::Encryption, helpers::{pic_meta, pic_types}, keyring::VerifierKey, metadata::MetaValues, network_specs::{NetworkSpecs, NetworkSpecsToSend, ValidCurrentVerifier, Verifier, VerifierValue}, print::{export_complex_single, export_complex_vector}, qr_transfers::{ContentLoadTypes}};

/// History log entry content for importing or removing metadata of a known network.
/// Contains network name, network version, metadata hash, verifier.
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct MetaValuesDisplay {
    name: String,
    version: u32,
    meta_hash: Vec<u8>,
}

impl MetaValuesDisplay {
    pub fn get(meta_values: &MetaValues) -> Self {
        Self {
            name: meta_values.name.to_string(),
            version: meta_values.version,
            meta_hash: blake2b(32, &[], &meta_values.meta).as_bytes().to_vec(),
        }
    }
    pub fn show(&self) -> String {
        let meta_id_pic = match pic_meta(&self.meta_hash) {
            Ok(a) => hex::encode(a),
            Err(_) => String::new(),
        };
        format!("\"specname\":\"{}\",\"spec_version\":\"{}\",\"meta_hash\":\"{}\",\"meta_id_pic\":\"{}\"", &self.name, &self.version, hex::encode(&self.meta_hash), meta_id_pic)
    }
}

/// History log entry content for creating and showing as a qr code `sufficient crypto`
/// content for load_metadata message;
/// effectively records that network metadata were signed by user.
/// Contains network name, network version, metadata hash, verifier value.
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct MetaValuesExport {
    name: String,
    version: u32,
    meta_hash: Vec<u8>,
    signed_by: VerifierValue,
}

impl MetaValuesExport {
    pub fn get(meta_values: &MetaValues, signed_by: &VerifierValue) -> Self {
        Self {
            name: meta_values.name.to_string(),
            version: meta_values.version,
            meta_hash: blake2b(32, &[], &meta_values.meta).as_bytes().to_vec(),
            signed_by: signed_by.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        let meta_id_pic = match pic_meta(&self.meta_hash) {
            Ok(a) => hex::encode(a),
            Err(_) => String::new(),
        };
        format!("\"specname\":\"{}\",\"spec_version\":\"{}\",\"meta_hash\":\"{}\",\"meta_id_pic\":\"{}\",\"signed_by\":{}", &self.name, &self.version, hex::encode(&self.meta_hash), meta_id_pic, export_complex_single(&self.signed_by, |a| a.show_card()))
    }
}

/// History log entry content for importing or removing network specs and corresponding network verifier.
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct NetworkSpecsDisplay {
    specs: NetworkSpecs,
    valid_current_verifier: ValidCurrentVerifier,
    general_verifier: Verifier,
}

impl NetworkSpecsDisplay {
    pub fn get(specs: &NetworkSpecs, valid_current_verifier: &ValidCurrentVerifier, general_verifier: &Verifier) -> Self {
        Self {
            specs: specs.to_owned(),
            valid_current_verifier: valid_current_verifier.to_owned(),
            general_verifier: general_verifier.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        self.specs.show(&self.valid_current_verifier, &self.general_verifier)
    }
}

/// History log entry content for creating and showing as a qr code `sufficient crypto`
/// content for add_specs message;
/// effectively records that network specs were signed by user.
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct NetworkSpecsExport {
    specs_to_send: NetworkSpecsToSend,
    signed_by: VerifierValue,
}

impl NetworkSpecsExport {
    pub fn get(specs_to_send: &NetworkSpecsToSend, signed_by: &VerifierValue) -> Self {
        Self {
            specs_to_send: specs_to_send.to_owned(),
            signed_by: signed_by.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        format!("{},\"signed_by\":{}", &self.specs_to_send.show(), export_complex_single(&self.signed_by, |a| a.show_card()))
    }
}

/// History log entry content for setting network verifier
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct NetworkVerifierDisplay {
    genesis_hash: Vec<u8>,
    valid_current_verifier: ValidCurrentVerifier,
    general_verifier: Verifier,
}

impl NetworkVerifierDisplay {
    pub fn get(verifier_key: &VerifierKey, valid_current_verifier: &ValidCurrentVerifier, general_verifier: &Verifier) -> Self {
        Self {
            genesis_hash: verifier_key.genesis_hash(),
            valid_current_verifier: valid_current_verifier.to_owned(),
            general_verifier: general_verifier.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        format!("\"genesis_hash\":\"{}\",\"current_verifier\":{}", hex::encode(&self.genesis_hash), export_complex_single(&self.valid_current_verifier, |a| a.show(&self.general_verifier)))
    }
}

/// History log entry content for importing types information.
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct TypesDisplay {
    types_hash: Vec<u8>,
    verifier: Verifier,
}

impl TypesDisplay {
    pub fn get(types_content: &ContentLoadTypes, verifier: &Verifier) -> Self {
        Self {
            types_hash: blake2b(32, &[], &types_content.store()).as_bytes().to_vec(),
            verifier: verifier.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        let types_id_pic = match pic_types(&self.types_hash) {
            Ok(a) => hex::encode(a),
            Err(_) => String::new(),
        };
        format!("\"types_hash\":\"{}\",\"types_id_pic\":\"{}\",\"verifier\":{}", hex::encode(&self.types_hash), types_id_pic, export_complex_single(&self.verifier, |a| a.show_card()))
    }
}

/// History log entry content for creating and showing as a qr code `sufficient crypto`
/// content for load_types message;
/// effectively records that types information was signed by user.
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct TypesExport {
    types_hash: Vec<u8>,
    signed_by: VerifierValue,
}

impl TypesExport {
    pub fn get(types_content: &ContentLoadTypes, signed_by: &VerifierValue) -> Self {
        Self {
            types_hash: blake2b(32, &[], &types_content.store()).as_bytes().to_vec(),
            signed_by: signed_by.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        let types_id_pic = match pic_types(&self.types_hash) {
            Ok(a) => hex::encode(a),
            Err(_) => String::new(),
        };
        format!("\"types_hash\":\"{}\",\"types_id_pic\":\"{}\",\"signed_by\":{}", hex::encode(&self.types_hash), types_id_pic, export_complex_single(&self.signed_by, |a| a.show_card()))
    }
}


/// History log entry content for identity action
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct IdentityHistory {
    seed_name: String,
    encryption: Encryption,
    public_key: Vec<u8>,
    path: String,
    network_genesis_hash: Vec<u8>,
}

impl IdentityHistory {
    pub fn get(seed_name: &str, encryption: &Encryption, public_key: &Vec<u8>, path: &str, network_genesis_hash: &Vec<u8>) -> Self {
        Self {
            seed_name: seed_name.to_string(),
            encryption: encryption.to_owned(),
            public_key: public_key.to_vec(),
            path: path.to_string(),
            network_genesis_hash: network_genesis_hash.to_vec(),
        }
    }
    pub fn show(&self) -> String {
        format!("\"seed_name\":\"{}\",\"encryption\":\"{}\",\"public_key\":\"{}\",\"path\":\"{}\",\"network_genesis_hash\":\"{}\"", &self.seed_name, &self.encryption.show(), hex::encode(&self.public_key), &self.path, hex::encode(&self.network_genesis_hash))
    }
}

/// Struct to store information in history log about transactions,
/// both successfully signed and the ones with wrong password entered by user
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct SignDisplay {
    transaction: Vec<u8>, // transaction
    network_name: String, // network name
    signed_by: VerifierValue, // signature author
    user_comment: String, // user entered comment for transaction
}

impl SignDisplay {
    pub fn get(transaction: &Vec<u8>, network_name: &str, signed_by: &VerifierValue, user_comment: &str) -> Self {
        Self {
            transaction: transaction.to_vec(),
            network_name: network_name.to_string(),
            signed_by: signed_by.to_owned(),
            user_comment: user_comment.to_string(),
        }
    }
    pub fn transaction_network_encryption(&self) -> (Vec<u8>, String, Encryption) {
        let encryption = match &self.signed_by {
            VerifierValue::Standard(MultiSigner::Ed25519(_)) => Encryption::Ed25519,
            VerifierValue::Standard(MultiSigner::Sr25519(_)) => Encryption::Sr25519,
            VerifierValue::Standard(MultiSigner::Ecdsa(_)) => Encryption::Ecdsa,
        };
        (self.transaction.to_vec(), self.network_name.to_string(), encryption)
    }
    pub fn transaction(&self) -> Vec<u8> {
        self.transaction.to_vec()
    }
    pub fn success<O>(&self, op: O) -> String 
    where O: Fn(&Self) -> String + Copy,
    {
        format!("\"transaction\":{},\"network_name\":\"{}\",\"signed_by\":{},\"user_comment\":\"{}\"", op(&self), &self.network_name, export_complex_single(&self.signed_by, |a| a.show_card()), &self.user_comment)
    }
    pub fn pwd_failure<O>(&self, op: O) -> String 
    where O: Fn(&Self) -> String + Copy,
    {
        format!("\"transaction\":{},\"network_name\":\"{}\",\"signed_by\":{},\"user_comment\":\"{}\",\"error\":\"wrong_password_entered\"", op(&self), &self.network_name, export_complex_single(&self.signed_by, |a| a.show_card()), &self.user_comment)
    }
}

/// Struct to store information in history log about messages,
/// both successfully signed and the ones with wrong password entered by user
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct SignMessageDisplay {
    message: String, // message
    network_name: String, // network name
    signed_by: VerifierValue, // signature author
    user_comment: String, // user entered comment for message
}

impl SignMessageDisplay {
    pub fn get(message: &str, network_name: &str, signed_by: &VerifierValue, user_comment: &str) -> Self {
        Self {
            message: message.to_string(),
            network_name: network_name.to_string(),
            signed_by: signed_by.to_owned(),
            user_comment: user_comment.to_string(),
        }
    }
    pub fn success(&self) -> String {
        format!("\"message\":\"{}\",\"network_name\":\"{}\",\"signed_by\":{},\"user_comment\":\"{}\"", hex::encode(&self.message.as_bytes()), &self.network_name, export_complex_single(&self.signed_by, |a| a.show_card()), &self.user_comment)
    }
    pub fn pwd_failure(&self) -> String {
        format!("\"message\":\"{}\",\"network_name\":\"{}\",\"signed_by\":{},\"user_comment\":\"{}\",\"error\":\"wrong_password_entered\"", hex::encode(&self.message.as_bytes()), &self.network_name, export_complex_single(&self.signed_by, |a| a.show_card()), &self.user_comment)
    }
}


/// Possible events to be recorded in the history log
#[derive(Decode, Encode, Clone)]
pub enum Event {
    MetadataAdded(MetaValuesDisplay),
    MetadataRemoved(MetaValuesDisplay),
    MetadataSigned(MetaValuesExport),
    NetworkSpecsAdded(NetworkSpecsDisplay),
    NetworkSpecsRemoved(NetworkSpecsDisplay),
    NetworkSpecsSigned(NetworkSpecsExport),
    NetworkVerifierSet(NetworkVerifierDisplay),
    GeneralVerifierSet(Verifier),
    TypesAdded(TypesDisplay),
    TypesRemoved(TypesDisplay),
    TypesSigned(TypesExport),
    TransactionSigned(SignDisplay),
    TransactionSignError(SignDisplay),
    MessageSigned(SignMessageDisplay),
    MessageSignError(SignMessageDisplay),
    IdentityAdded(IdentityHistory),
    IdentityRemoved(IdentityHistory),
    IdentitiesWiped,
    DeviceWasOnline,
    ResetDangerRecord,
    SeedCreated(String),
    SeedNameWasShown(String), // for individual seed_name
    Warning(String), // TODO change to actual crate warning
    WrongPassword,
    UserEntry(String),
    SystemEntry(String),
    HistoryCleared,
    DatabaseInitiated,
}

#[derive(Decode, Encode, Clone)]
pub struct Entry {
    pub timestamp: String,
    pub events: Vec<Event>, // events already in showable form
}

impl Event {
    pub fn show<O>(&self, op: O) -> String 
    where O: Fn(&SignDisplay) -> String + Copy,
    {
        match &self {
            Event::MetadataAdded(x) => format!("\"event\":\"metadata_added\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::MetadataRemoved(x) => format!("\"event\":\"metadata_removed\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::MetadataSigned(x) => format!("\"event\":\"load_metadata_message_signed\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::NetworkSpecsAdded(x) => format!("\"event\":\"network_specs_added\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::NetworkSpecsRemoved(x) => format!("\"event\":\"network_removed\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::NetworkSpecsSigned(x) => format!("\"event\":\"add_specs_message_signed\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::NetworkVerifierSet(x) => format!("\"event\":\"network_verifier_set\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::GeneralVerifierSet(x) => format!("\"event\":\"general_verifier_added\",\"payload\":{}", export_complex_single(x, |a| a.show_card())),
            Event::TypesAdded(x) => format!("\"event\":\"types_added\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::TypesRemoved(x) => format!("\"event\":\"types_removed\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::TypesSigned(x) => format!("\"event\":\"load_types_message_signed\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::TransactionSigned(x) => format!("\"event\":\"transaction_signed\",\"payload\":{}", export_complex_single(x, |a| a.success(|b| op(b)))),
            Event::TransactionSignError(x) => format!("\"event\":\"transaction_sign_error\",\"payload\":{}", export_complex_single(x, |a| a.pwd_failure(|b| op(b)))),
            Event::MessageSigned(x) => format!("\"event\":\"message_signed\",\"payload\":{}", export_complex_single(x, |a| a.success())),
            Event::MessageSignError(x) => format!("\"event\":\"message_sign_error\",\"payload\":{}", export_complex_single(x, |a| a.pwd_failure())),
            Event::IdentityAdded(x) => format!("\"event\":\"identity_added\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::IdentityRemoved(x) => format!("\"event\":\"identity_removed\",\"payload\":{}", export_complex_single(x, |a| a.show())),
            Event::IdentitiesWiped => String::from("\"event\":\"identities_wiped\""),
            Event::DeviceWasOnline => String::from("\"event\":\"device_online\""),
            Event::ResetDangerRecord => String::from("\"event\":\"reset_danger_record\""),
            Event::SeedCreated(x) => format!("\"event\":\"seed_created\",\"payload\":\"{}\"", x),
            Event::SeedNameWasShown(seed_name) => format!("\"event\":\"seed_name_shown\",\"payload\":\"{}\"", seed_name),
            Event::Warning(x) => format!("\"event\":\"warning\",\"payload\":\"{}\"", x),
            Event::WrongPassword => String::from("\"event\":\"wrong_password_entered\""),
            Event::UserEntry(x) => format!("\"event\":\"user_entered_event\",\"payload\":\"{}\"", x),
            Event::SystemEntry(x) => format!("\"event\":\"system_entered_event\",\"payload\":\"{}\"", x),
            Event::HistoryCleared => String::from("\"event\":\"history_cleared\""),
            Event::DatabaseInitiated => String::from("\"event\":\"database_initiated\""),
        }
    }
}

impl Entry {
    pub fn show<O>(&self, op: O) -> String 
    where O: Fn(&SignDisplay) -> String + Copy,
    {
        let events_chain = export_complex_vector(&self.events, |a| a.show(|b| op(b)));
        format!("\"timestamp\":\"{}\",\"events\":{}", self.timestamp, events_chain)
    }
}

pub fn all_events_preview() -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();
    let meta_values = MetaValues {
        name: String::from("westend"),
        version: 9000,
        meta: Vec::new(),
    };
    let public = [142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];
    let verifier_value = VerifierValue::Standard(MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(public)));
    let verifier = Verifier(Some(verifier_value.clone()));
    let valid_current_verifier = ValidCurrentVerifier::General;
    let network_specs = NetworkSpecs {
        base58prefix: 42,
        color: String::from("#660D35"),
        decimals: 12,
        encryption: Encryption::Sr25519,
        genesis_hash: hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value").try_into().expect("known value"),
        logo: String::from("westend"),
        name: String::from("westend"),
        order: 3,
        path_id: String::from("//westend"),
        secondary_color: String::from("#262626"),
        title: String::from("Westend"),
        unit: String::from("WND"),
    };
    
    events.push(Event::MetadataAdded(MetaValuesDisplay::get(&meta_values)));
    events.push(Event::MetadataRemoved(MetaValuesDisplay::get(&meta_values)));
    events.push(Event::MetadataSigned(MetaValuesExport::get(&meta_values, &verifier_value)));
    events.push(Event::NetworkSpecsAdded(NetworkSpecsDisplay::get(&network_specs, &valid_current_verifier, &verifier)));
    events.push(Event::NetworkSpecsRemoved(NetworkSpecsDisplay::get(&network_specs, &valid_current_verifier, &verifier)));
    events.push(Event::NetworkSpecsSigned(NetworkSpecsExport::get(&network_specs.to_send(), &verifier_value)));
    events.push(Event::NetworkVerifierSet(NetworkVerifierDisplay::get(&VerifierKey::from_parts(&network_specs.genesis_hash.to_vec()), &valid_current_verifier, &verifier)));
    events.push(Event::GeneralVerifierSet(verifier.to_owned()));
    events.push(Event::TypesAdded(TypesDisplay::get(&ContentLoadTypes::from_vec(&Vec::new()), &verifier)));
    events.push(Event::TypesRemoved(TypesDisplay::get(&ContentLoadTypes::from_vec(&Vec::new()), &verifier)));
    events.push(Event::TypesSigned(TypesExport::get(&ContentLoadTypes::from_vec(&Vec::new()), &verifier_value)));
    events.push(Event::TransactionSigned(SignDisplay::get(&Vec::new(), "westend", &verifier_value, "send to Alice")));
    events.push(Event::TransactionSignError(SignDisplay::get(&Vec::new(), "westend", &verifier_value, "send to Alice")));
    events.push(Event::MessageSigned(SignMessageDisplay::get("This is Alice\nRoger", "westend", &verifier_value, "send to Alice")));
    events.push(Event::MessageSignError(SignMessageDisplay::get("This is Alice\nRoger", "westend", &verifier_value, "send to Alice")));
    events.push(Event::IdentityAdded(IdentityHistory::get("Alice", &Encryption::Sr25519, &public.to_vec(), "//", &network_specs.genesis_hash.to_vec())));
    events.push(Event::IdentityRemoved(IdentityHistory::get("Alice", &Encryption::Sr25519, &public.to_vec(), "//", &network_specs.genesis_hash.to_vec())));
    events.push(Event::IdentitiesWiped);
    events.push(Event::DeviceWasOnline);
    events.push(Event::ResetDangerRecord);
    events.push(Event::SeedCreated(String::from("Alice")));
    events.push(Event::SeedNameWasShown(String::from("AliceSecretSeed")));
    events.push(Event::Warning(String::from("Received network information is not verified.")));
    events.push(Event::WrongPassword);
    events.push(Event::UserEntry(String::from("Lalala!!!")));
    events.push(Event::SystemEntry(String::from("Blip blop")));
    events.push(Event::HistoryCleared);
    events.push(Event::DatabaseInitiated);
    
    events
}

pub fn print_all_events() -> String {
    let events = all_events_preview();
    let entry = Entry {
        timestamp: String::from("2019-12-15 12:00:0.00000000 UTC"),
        events,
    };
    format!("{{\"order\":0,{}}}", entry.show(|a| format!("\"{}\"", hex::encode(a.transaction()))))
}
