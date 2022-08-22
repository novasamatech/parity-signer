pub mod common;

use crate::common::{assert_cmd_stdout, assert_files_eq, setup, teardown};
use std::path::PathBuf;

#[test]
fn it_adds_specs() {
    let files_dir = PathBuf::from("./tests/add_specs");
    setup(&files_dir);
    let cmd = format!(
        "add-specs -f --all --hot-db-path {0} --files-dir {0}",
        files_dir.to_string_lossy()
    );
    assert_cmd_stdout(&cmd, "");

    let specs = files_dir.join("sign_me_add_specs_polkadot_sr25519");
    let expected_specs = PathBuf::from("./tests/for_tests/add_specs_polkadot");
    assert_files_eq(specs, expected_specs);
    teardown(&files_dir);
}
