pub mod common;
use crate::common::{assert_files_eq, remove_if_exists, run_cmd_test};

use constants::FOLDER;
use std::fs;

#[test]
fn it_loads_metadata() {
    let f_path = format!("{}/sign_me_load_metadata_polkadotV30", FOLDER);
    remove_if_exists(&f_path);

    run_cmd_test("load-metadata -f -a", "", "./tests/load_metadata");

    let expected_meta = String::from("./tests/for_tests/load_metadata_polkadotV30");
    assert_files_eq(&f_path, &expected_meta);
    fs::remove_file(&f_path).unwrap();
}
