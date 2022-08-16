pub mod common;
use crate::common::{assert_files_eq, remove_if_exists, run_cmd_test};

use constants::FOLDER;
use std::fs;

#[test]
fn it_loads_types() {
    let f_path = format!("{}/sign_me_load_types", FOLDER);
    remove_if_exists(&f_path);

    run_cmd_test("load-types", "");

    let expected_types = String::from("./tests/for_tests/load_types");
    assert_files_eq(&f_path, &expected_types);
    fs::remove_file(&f_path).unwrap();
}
