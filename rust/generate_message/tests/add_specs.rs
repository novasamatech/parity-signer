pub mod common;
use crate::common::{assert_files_eq, remove_if_exists, run_cmd_test};

use constants::FOLDER;
use std::fs;

#[test]
fn it_adds_specs() {
    let f_path = format!("{}/sign_me_add_specs_polkadot_sr25519", FOLDER);
    remove_if_exists(&f_path);

    run_cmd_test("add-specs -f --all", "", "./tests/add_specs");

    let expected_specs = String::from("./tests/for_tests/add_specs_polkadot");
    assert_files_eq(&f_path, &expected_specs);
    fs::remove_file(&f_path).unwrap();
}
