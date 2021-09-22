use parity_scale_codec_derive::{Decode, Encode};

#[derive(Decode, Encode)]
pub enum Event {
    MetadataAdded(String), // MetaValuesDisplay.show()
    MetadataRemoved(String), // MetaValuesDisplay.show()
    NetworkAdded(String), // NetworkDisplay.show()
    NetworkRemoved(String), // ChainSpecs.show()
    MetadataVerifierAdded(String), // NetworkVerifier.show()
    MetadataVerifierRemoved(String), // NetworkVerifier.show()
    GeneralVerifierAdded(String), // Verifier.show_card()
    GeneralVerifierRemoved(String), // Verifier.show_card()
    TypesInfoUpdated(String), // TypesUpdate.show()
    SignedTypes(String), // TypesUpdate.show()
    SignedLoadMetadata(String), //  VerifiedMetaValuesDisplay.show()
    SignedAddNetwork(String), // NetworkDisplay.show()
    TransactionSigned(String), // SignDisplay.show()
    IdentityAdded(String), // IdentityHistory.show()
    IdentityRemoved(String), // IdentityHistory.show()
    IdentitiesWiped,
    DeviceWasOnline,
    SeedsWereAccessed,
    SeedsWereShown,
    Warning(String),
    Error(String),
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
            Event::MetadataAdded(x) => format!("{{\"event\":\"metadata_added\",\"payload\":{{{}}}}}", x),
            Event::MetadataRemoved(x) => format!("{{\"event\":\"metadata_removed\",\"payload\":{{{}}}}}", x),
            Event::NetworkAdded(x) => format!("{{\"event\":\"network_added\",\"payload\":{{{}}}}}", x),
            Event::NetworkRemoved(x) => format!("{{\"event\":\"network_removed\",\"payload\":{{{}}}}}", x),
            Event::MetadataVerifierAdded(x) => format!("{{\"event\":\"metadata_verifier_added\",\"payload\":{{{}}}}}", x),
            Event::MetadataVerifierRemoved(x) => format!("{{\"event\":\"metadata_verifier_removed\",\"payload\":{{{}}}}}", x),
            Event::GeneralVerifierAdded(x) => format!("{{\"event\":\"general_verifier_added\",\"payload\":{{\"verifier\":{}}}}}", x),
            Event::GeneralVerifierRemoved(x) => format!("{{\"event\":\"general_verifier_removed\",\"payload\":{{\"verifier\":{}}}}}", x),
            Event::TypesInfoUpdated(x) => format!("{{\"event\":\"types_info_updated\",\"payload\":{{{}}}}}", x),
            Event::SignedTypes(x) => format!("{{\"event\":\"load_types_message_signed\",\"payload\":{{{}}}}}", x),
            Event::SignedLoadMetadata(x) => format!("{{\"event\":\"load_metadata_message_signed\",\"payload\":{{{}}}}}", x),
            Event::SignedAddNetwork(x) => format!("{{\"event\":\"add_network_message_signed\",\"payload\":{{{}}}}}", x),
            Event::TransactionSigned(x) => format!("{{\"event\":\"transaction_signed\",\"payload\":{{{}}}}}", x),
            Event::IdentityAdded(x) => format!("{{\"event\":\"identity_added\",\"payload\":{{{}}}}}", x),
            Event::IdentityRemoved(x) => format!("{{\"event\":\"identity_removed\",\"payload\":{{{}}}}}", x),
            Event::IdentitiesWiped => String::from("{\"event\":\"identities_wiped\"}"),
            Event::DeviceWasOnline => String::from("{\"event\":\"device_online\"}"),
            Event::SeedsWereAccessed => String::from("{\"event\":\"seeds_accessed\"}"),
            Event::SeedsWereShown => String::from("{\"event\":\"seeds_shown\"}"),
            Event::Warning(x) => format!("{{\"event\":\"warning\",\"payload\":\"{}\"}}", x),
            Event::Error(x) => format!("{{\"event\":\"error\",\"payload\":\"{}\"}}", x),
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
