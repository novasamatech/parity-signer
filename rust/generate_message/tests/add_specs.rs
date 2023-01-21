pub mod common;

use crate::common::{assert_cmd_stdout, setup};
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn it_adds_specs() {
    let files_dir = tempdir().unwrap();
    let db = sled::open(&files_dir).unwrap();

    setup(&db);
    let cmd = format!(
        "add-specs -f --all --hot-db-path {0} --files-dir {0}",
        files_dir.path().to_string_lossy()
    );
    assert_cmd_stdout(&cmd, "");

    let _specs = files_dir.path().join("sign_me_add_specs_polkadot_sr25519");
    let _expected_specs = PathBuf::from("./tests/for_tests/add_specs_polkadot");
    // TODO
    // assert_files_eq(specs, expected_specs);
}
