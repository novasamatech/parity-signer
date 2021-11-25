//!List of all screens

///All screens
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Screen {
    Log,
	LogDetails,
	Scan,
	Transaction,
	SeedSelector,
	Keys,
	KeyDetails,
	Backup,
	NewSeed,
	RecoverSeedName,
	RecoverSeedPhrase,
	DeriveKey,
	Settings,
	Verifier,
	ManageNetwork,
    Nowhere,
}

impl Screen {
    ///Decode screen name string supplied from UI
    pub fn parse(input: &str) -> Screen {
        match input {
            "Log" => Screen::Log,
	        "LogDetails" => Screen::LogDetails,
        	"Scan" => Screen::Scan,
	        "Transaction" => Screen::Transaction,
        	"SeedSelector" => Screen::SeedSelector,
	        "Keys" => Screen::Keys,
	        "KeyDetails" => Screen::KeyDetails,
	        "Backup" => Screen::Backup,
	        "NewSeed" => Screen::NewSeed,
	        "RecoverSeedName" => Screen::RecoverSeedName,
	        "RecoverSeedPhrase" => Screen::RecoverSeedPhrase,
	        "DeriveKey" => Screen::DeriveKey,
	        "Settings" => Screen::Settings,
	        "Verifier" => Screen::Verifier,
	        "ManageNetwork" => Screen::ManageNetwork,
            _ => Screen::Nowhere,
        }
    }

    ///Encode screen name into string for UI
    pub fn get_name(&self) -> Option<String> {
        match self {
            Screen::Log => Some(String::from("Log")),
        	Screen::LogDetails => Some(String::from("LogDetails")),
        	Screen::Scan => Some(String::from("Scan")),
        	Screen::Transaction => Some(String::from("Transaction")),
        	Screen::SeedSelector => Some(String::from("SeedSelector")),
        	Screen::Keys => Some(String::from("Keys")),
        	Screen::KeyDetails => Some(String::from("KeyDetails")),
        	Screen::Backup => Some(String::from("Backup")),
        	Screen::NewSeed => Some(String::from("NewSeed")),
        	Screen::RecoverSeedName => Some(String::from("RecoverSeedName")),
        	Screen::RecoverSeedPhrase => Some(String::from("RecoverSeedPhrase")),
        	Screen::DeriveKey => Some(String::from("DeriveKey")),
        	Screen::Settings => Some(String::from("Settings")),
        	Screen::Verifier => Some(String::from("Verifier")),
        	Screen::ManageNetwork => Some(String::from("ManageNetwork")),
            Screen::Nowhere => None,
        }
    }

    pub fn get_default_label(&self) -> String {
        match self {
            Screen::Log => "Log",
        	Screen::LogDetails => "Event details",
        	Screen::Scan => "",
        	Screen::Transaction => "",
        	Screen::SeedSelector => "Select seed",
        	Screen::Keys => "",
        	Screen::KeyDetails => "Key",
        	Screen::Backup => "this should be popover",
        	Screen::NewSeed => "",
        	Screen::RecoverSeedName => "Recover Seed",
        	Screen::RecoverSeedPhrase => "Recover Seed",
        	Screen::DeriveKey => "",
        	Screen::Settings => "Settings",
        	Screen::Verifier => "VERIFIER CERTIFICATE",
        	Screen::ManageNetwork => "MANAGE NETWORKS",
            Screen::Nowhere => "",
        }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_some_screens() {
        assert_eq!(Screen::parse("Log"), Screen::Log);
        assert_eq!(Screen::parse(""), Screen::Nowhere);
        assert_eq!(Screen::parse("Sea of thought"), Screen::Nowhere);
    }
}
