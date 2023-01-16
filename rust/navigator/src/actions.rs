//!List of all interactive actions in app

//use super::screens::Screen;
//use crate::navstate::{Navstate, State};

///All actions
#[derive(PartialEq, Eq, Debug)]
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
    RightButtonAction,
    Shield,
    NewSeed,
    RecoverSeed,
    BackupSeed,
    NetworkSelector,
    CheckPassword,
    TransactionFetched,
    RemoveNetwork,
    RemoveMetadata,
    RemoveTypes,
    SignNetworkSpecs,
    SignMetadata,
    SignTypes,
    ManageNetworks,
    ViewGeneralVerifier,
    ManageMetadata,
    RemoveKey,
    RemoveSeed,
    ClearLog,
    CreateLogComment,
    ShowLogDetails,
    Increment,
    ShowDocuments,
    TextEntry,
    PushWord,
    Nothing,
}
