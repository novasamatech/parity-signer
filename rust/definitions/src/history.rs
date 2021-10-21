use hex;
use blake2_rfc::blake2b::blake2b;
use parity_scale_codec_derive::{Decode, Encode};
use crate::{crypto::Encryption, keyring::VerifierKey, metadata::MetaValues, network_specs::{ChainSpecs, ChainSpecsToSend, CurrentVerifier, Verifier}, qr_transfers::{ContentLoadTypes}};
use std::convert::TryInto;

/// Struct to store the metadata values in history log for adding/removing metadata
/// for known networks (network name, network version, metadata hash)
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
        format!("\"specname\":\"{}\",\"spec_version\":\"{}\",\"meta_hash\":\"{}\"", &self.name, &self.version, hex::encode(&self.meta_hash))
    }
}

/// Struct to store the metadata values in history log for signing load_metadata message
/// by user (network name, network version, metadata hash, verifier)
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct MetaValuesExport {
    name: String,
    version: u32,
    meta_hash: Vec<u8>,
    signed_by: Verifier,
}

impl MetaValuesExport {
    pub fn get(meta_values: &MetaValues, signed_by: &Verifier) -> Self {
        Self {
            name: meta_values.name.to_string(),
            version: meta_values.version,
            meta_hash: blake2b(32, &[], &meta_values.meta).as_bytes().to_vec(),
            signed_by: signed_by.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        format!("\"specname\":\"{}\",\"spec_version\":\"{}\",\"meta_hash\":\"{}\",\"signed_by\":{}", &self.name, &self.version, hex::encode(&self.meta_hash), &self.signed_by.show_card())
    }
}

/// Struct to store the network specs and network verifier values in history log
/// for adding/removing network specs
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct NetworkSpecsDisplay {
    specs: ChainSpecs,
    current_verifier: CurrentVerifier,
    general_verifier: Verifier,
}

impl NetworkSpecsDisplay {
    pub fn get(specs: &ChainSpecs, current_verifier: &CurrentVerifier, general_verifier: &Verifier) -> Self {
        Self {
            specs: specs.to_owned(),
            current_verifier: current_verifier.to_owned(),
            general_verifier: general_verifier.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        self.specs.show(&self.current_verifier, &self.general_verifier)
    }
}

/// Struct to store history entries for signing add_specs message by user
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct NetworkSpecsExport {
    specs_to_send: ChainSpecsToSend,
    signed_by: Verifier,
}

impl NetworkSpecsExport {
    pub fn get(specs_to_send: &ChainSpecsToSend, signed_by: &Verifier) -> Self {
        Self {
            specs_to_send: specs_to_send.to_owned(),
            signed_by: signed_by.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        format!("{},\"signed_by\":{}", &self.specs_to_send.show(), &self.signed_by.show_card())
    }
}

/// Struct to store history records for setting network verifier
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct NetworkVerifierDisplay {
    genesis_hash: Vec<u8>,
    current_verifier: CurrentVerifier,
    general_verifier: Verifier
}

impl NetworkVerifierDisplay {
    pub fn get(verifier_key: &VerifierKey, current_verifier: &CurrentVerifier, general_verifier: &Verifier) -> Self {
        Self {
            genesis_hash: verifier_key.genesis_hash(),
            current_verifier: current_verifier.to_owned(),
            general_verifier: general_verifier.to_owned(),
        }
    }
    pub fn show(&self) -> String {
        format!("\"genesis_hash\":\"{}\",\"current_verifier\":{}", hex::encode(&self.genesis_hash), &self.current_verifier.show(&self.general_verifier))
    }
}

/// Struct to store types updates in history log
/// Is used for both importing types and for recordng that the used has signed
/// types export, in which case verifier is user identity
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
        format!("\"types_hash\":\"{}\",\"verifier\":{}", hex::encode(&self.types_hash), &self.verifier.show_card())
    }
    pub fn show_export(&self) -> String {
        format!("\"types_hash\":\"{}\",\"signed_by\":{}", hex::encode(&self.types_hash), &self.verifier.show_card())
    }
}


/// Struct to store history entry for identity action
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

/// Struct to store information about signed transactions
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct SignDisplay {
    transaction: Vec<u8>, // transaction
    signed_by: Verifier, // signature author
    user_comment: String, // user entered comment for transaction
}

impl SignDisplay {
    pub fn get(transaction: &Vec<u8>, signed_by: &Verifier, user_comment: &str) -> Self {
        Self {
            transaction: transaction.to_vec(),
            signed_by: signed_by.to_owned(),
            user_comment: user_comment.to_string(),
        }
    }
    pub fn success(&self) -> String {
        format!("\"transaction\":\"{}\",\"signed_by\":{},\"user_comment\":\"{}\"", hex::encode(&self.transaction), &self.signed_by.show_card(), &self.user_comment)
    }
    pub fn pwd_failure(&self) -> String {
        format!("\"transaction\":\"{}\",\"signed_by\":{},\"user_comment\":\"{}\",\"error\":\"wrong_password_entered\"", hex::encode(&self.transaction), &self.signed_by.show_card(), &self.user_comment)
    }
}



#[derive(Decode, Encode, Clone)]
pub enum Event {
    MetadataAdded(MetaValuesDisplay),
    MetadataRemoved(MetaValuesDisplay),
    NetworkSpecsAdded(NetworkSpecsDisplay),
    NetworkSpecsRemoved(NetworkSpecsDisplay),
    NetworkVerifierSet(NetworkVerifierDisplay),
    GeneralVerifierSet(Verifier),
    TypesAdded(TypesDisplay),
    TypesRemoved(TypesDisplay),
    SignedTypes(TypesDisplay),
    SignedLoadMetadata(MetaValuesExport),
    SignedAddNetworkSpecs(NetworkSpecsExport),
    TransactionSigned(SignDisplay),
    IdentityAdded(IdentityHistory),
    IdentityRemoved(IdentityHistory),
    IdentitiesWiped,
    DeviceWasOnline,
    ResetDangerRecord,
    SeedNameWasShown(String), // for individual seed_name
    Warning(String), // TODO change to actual crate warning
    TransactionSignError(SignDisplay),
    WrongPassword,
    UserEntry(String),
    SystemEntry(String),
    HistoryCleared,
    DatabaseInitiated,
}

#[derive(Decode, Encode)]
pub struct Entry {
    pub timestamp: String,
    pub events: Vec<Event>, // events already in showable form
}

impl Event {
    pub fn show(&self) -> String {
        match &self {
            Event::MetadataAdded(x) => format!("{{\"event\":\"metadata_added\",\"payload\":{{{}}}}}", x.show()),
            Event::MetadataRemoved(x) => format!("{{\"event\":\"metadata_removed\",\"payload\":{{{}}}}}", x.show()),
            Event::NetworkSpecsAdded(x) => format!("{{\"event\":\"network_specs_added\",\"payload\":{{{}}}}}", x.show()),
            Event::NetworkSpecsRemoved(x) => format!("{{\"event\":\"network_removed\",\"payload\":{{{}}}}}", x.show()),
            Event::NetworkVerifierSet(x) => format!("{{\"event\":\"network_verifier_set\",\"payload\":{{{}}}}}", x.show()),
            Event::GeneralVerifierSet(x) => format!("{{\"event\":\"general_verifier_added\",\"payload\":{}}}", x.show_card()),
            Event::TypesAdded(x) => format!("{{\"event\":\"types_added\",\"payload\":{{{}}}}}", x.show()),
            Event::TypesRemoved(x) => format!("{{\"event\":\"types_removed\",\"payload\":{{{}}}}}", x.show()),
            Event::SignedTypes(x) => format!("{{\"event\":\"load_types_message_signed\",\"payload\":{{{}}}}}", x.show_export()),
            Event::SignedLoadMetadata(x) => format!("{{\"event\":\"load_metadata_message_signed\",\"payload\":{{{}}}}}", x.show()),
            Event::SignedAddNetworkSpecs(x) => format!("{{\"event\":\"add_specs_message_signed\",\"payload\":{{{}}}}}", x.show()),
            Event::TransactionSigned(x) => format!("{{\"event\":\"transaction_signed\",\"payload\":{{{}}}}}", x.success()),
            Event::IdentityAdded(x) => format!("{{\"event\":\"identity_added\",\"payload\":{{{}}}}}", x.show()),
            Event::IdentityRemoved(x) => format!("{{\"event\":\"identity_removed\",\"payload\":{{{}}}}}", x.show()),
            Event::IdentitiesWiped => String::from("{\"event\":\"identities_wiped\"}"),
            Event::DeviceWasOnline => String::from("{\"event\":\"device_online\"}"),
            Event::ResetDangerRecord => String::from("{\"event\":\"reset_danger_record\"}"),
            Event::SeedNameWasShown(seed_name) => format!("{{\"event\":\"seed_name_shown\",\"payload\":\"{}\"}}", seed_name),
            Event::Warning(x) => format!("{{\"event\":\"warning\",\"payload\":\"{}\"}}", x),
            Event::TransactionSignError(x) => format!("{{\"event\":\"sign_error\",\"payload\":{{{}}}}}", x.pwd_failure()),
            Event::WrongPassword => String::from("{\"event\":\"wrong_password_entered\"}"),
            Event::UserEntry(x) => format!("{{\"event\":\"user_entered_event\",\"payload\":\"{}\"}}", x),
            Event::SystemEntry(x) => format!("{{\"event\":\"system_entered_event\",\"payload\":\"{}\"}}", x),
            Event::HistoryCleared => String::from("{\"event\":\"history_cleared\"}"),
            Event::DatabaseInitiated => String::from("{\"event\":\"database_initiated\"}"),
        }
    }
}

impl Entry {
    pub fn show(&self) -> String {
        let mut events_chain = String::new();
        for (i,x) in self.events.iter().enumerate() {
            if i>0 {events_chain.push_str(",")}
            events_chain.push_str(&x.show());
        }
        format!("\"timestamp\":\"{}\",\"events\":[{}]", self.timestamp, events_chain)
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
    let general_verifier = Verifier::Sr25519(public);
    let current_verifier = CurrentVerifier::General;
    let network_specs = ChainSpecs {
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
    let network_specs_to_send = ChainSpecsToSend {
        base58prefix: 42,
        color: String::from("#660D35"),
        decimals: 12,
        encryption: Encryption::Sr25519,
        genesis_hash: hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value").try_into().expect("known value"),
        logo: String::from("westend"),
        name: String::from("westend"),
        path_id: String::from("//westend"),
        secondary_color: String::from("#262626"),
        title: String::from("Westend"),
        unit: String::from("WND"),
    };
    
    events.push(Event::MetadataAdded(MetaValuesDisplay::get(&meta_values)));
    events.push(Event::MetadataRemoved(MetaValuesDisplay::get(&meta_values)));
    events.push(Event::NetworkSpecsAdded(NetworkSpecsDisplay::get(&network_specs, &current_verifier, &general_verifier)));
    events.push(Event::NetworkSpecsRemoved(NetworkSpecsDisplay::get(&network_specs, &current_verifier, &general_verifier)));
    events.push(Event::NetworkVerifierSet(NetworkVerifierDisplay::get(&VerifierKey::from_parts(&network_specs.genesis_hash.to_vec()), &current_verifier, &general_verifier)));
    events.push(Event::GeneralVerifierSet(general_verifier.to_owned()));
    events.push(Event::TypesAdded(TypesDisplay::get(&ContentLoadTypes::from_vec(&Vec::new()), &general_verifier)));
    events.push(Event::TypesRemoved(TypesDisplay::get(&ContentLoadTypes::from_vec(&Vec::new()), &general_verifier)));
    events.push(Event::SignedTypes(TypesDisplay::get(&ContentLoadTypes::from_vec(&Vec::new()), &general_verifier)));
    events.push(Event::SignedLoadMetadata(MetaValuesExport::get(&meta_values, &general_verifier)));
    events.push(Event::SignedAddNetworkSpecs(NetworkSpecsExport::get(&network_specs_to_send, &general_verifier)));
    events.push(Event::TransactionSigned(SignDisplay::get(&Vec::new(), &general_verifier, "send to Alice")));
    events.push(Event::IdentityAdded(IdentityHistory::get("Alice", &Encryption::Sr25519, &public.to_vec(), "//", &network_specs.genesis_hash.to_vec())));
    events.push(Event::IdentityRemoved(IdentityHistory::get("Alice", &Encryption::Sr25519, &public.to_vec(), "//", &network_specs.genesis_hash.to_vec())));
    events.push(Event::IdentitiesWiped);
    events.push(Event::DeviceWasOnline);
    events.push(Event::ResetDangerRecord);
    events.push(Event::SeedNameWasShown(String::from("AliceSecretSeed")));
    events.push(Event::Warning(String::from("Received network information is not verified.")));
    events.push(Event::TransactionSignError(SignDisplay::get(&Vec::new(), &general_verifier, "send to Alice")));
    events.push(Event::WrongPassword);
    events.push(Event::UserEntry(String::from("Lalala!!!")));
    events.push(Event::SystemEntry(String::from("Blip blop")));
    events.push(Event::HistoryCleared);
    events.push(Event::DatabaseInitiated);
    
    events
}
