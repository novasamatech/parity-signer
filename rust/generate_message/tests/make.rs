pub mod common;
use crate::common::{assert_cmd_stdout, assert_files_eq, remove_if_exists, run_cmd_test};

use constants::{EXPORT_FOLDER, FOLDER};
use std::fs;

#[test]
fn it_signs() {
    let unsigned = format!("{}/sign_me_load_metadata_polkadotV30", FOLDER);
    let unverified = format!("{}/load_metadata_polkadotV30_unverified.txt", EXPORT_FOLDER);
    let signed = format!(
        "{}/load_metadata_polkadotV30_Alice-sr25519.txt",
        EXPORT_FOLDER
    );
    remove_if_exists(&unverified);
    remove_if_exists(&unsigned);
    remove_if_exists(&signed);

    run_cmd_test("load-metadata -f -a", "", "./tests/it_signs");
    assert_cmd_stdout(
        "make --goal text --crypto sr25519 --msg load-metadata --payload sign_me_load_metadata_polkadotV30", 
        "");

    let expected = String::from("./tests/for_tests/load_metadata_polkadotV30_unverified.txt");
    assert_files_eq(&unverified, &expected);

    assert_cmd_stdout(
        "make --goal text --crypto sr25519 --msg load-metadata --verifier-alice sr25519 --payload sign_me_load_metadata_polkadotV30",
        "");
    // Signing result is not deterministic, so we can't compare the result to a known

    fs::remove_file(&unsigned).unwrap();
    fs::remove_file(&unverified).unwrap();
    fs::remove_file(&signed).unwrap();
}
