//!List of all interactive actions in app

//use super::screens::Screen;
//use crate::navstate::{Navstate, State};

///All actions
#[derive(PartialEq, Debug)]
pub enum Action {
    Start,
    NavbarLog,
    NavbarScan,
    NavbarKeys,
    NavbarSettings,
    GoBack,
    GoForward,
    SelectSeed,
    SelectKey,
    NewKey,
    RightButton,
    Shield,
    NewSeed,
    RecoverSeed,
    BackupSeed,
    NetworkSelector,
    NextUnit,
    PreviousUnit,
    ChangeNetwork,
    CheckPassword,
    TransactionFetched,
    GenerateSufficientCrypto,
    Nothing,
}

impl Action {
    ///Decode action name string supplied from UI
    pub fn parse(input: &str) -> Action {
        match input {
            "Start" => Action::Start,
            "NavbarLog" => Action::NavbarLog,
            "NavbarScan" => Action::NavbarScan,
            "NavbarKeys" => Action::NavbarKeys,
            "NavbarSettings" => Action::NavbarSettings,
            "GoBack" => Action::GoBack,
            "GoForward" => Action::GoForward,
            "SelectSeed" => Action::SelectSeed,
            "SelectKey" => Action::SelectKey,
            "NewKey" => Action::NewKey,
            "RightButton" => Action::RightButton,
            "Shield" => Action::Shield,
            "NewSeed" => Action::NewSeed,
            "RecoverSeed" => Action::RecoverSeed,
            "BackupSeed" => Action::BackupSeed,
            "NetworkSelector" => Action::NetworkSelector,
            "NextUnit" => Action::NextUnit,
            "PreviousUnit" => Action::PreviousUnit,
            "ChangeNetwork" => Action::ChangeNetwork,
            "CheckPassword" => Action::CheckPassword,
            "TransactionFetched" => Action::TransactionFetched,
            "GenerateSufficientCrypto" => Action::GenerateSufficientCrypto,
            _ => Action::Nothing,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_some_actions() {
        assert_eq!(Action::parse("GoBack"), Action::GoBack);
        assert_eq!(Action::parse(""), Action::Nothing);
        assert_eq!(Action::parse("Booom!"), Action::Nothing);
    }
}
