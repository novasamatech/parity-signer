pub mod common;
use crate::common::{assert_files_eq, remove_if_exists, run_cmd_test};

use std::fs;

#[test]
fn it_derives() {
    let result_path = String::from("derivations-polkadot.txt");
    remove_if_exists(&result_path);
    let expected_text = String::from("./tests/for_tests/derivations-polkadot.txt");

    run_cmd_test(
        "derivations --title polkadot --goal text --derivations //1",
        "Found and used 1 valid derivations:\n\"//1\"\n",
    );

    assert_files_eq(&expected_text, &result_path);
    fs::remove_file(&result_path).unwrap();
}
