pub mod common;
use crate::common::{assert_cmd_stdout, assert_files_eq, setup, teardown};



use std::path::PathBuf;

#[test]
fn it_loads_metadata() {
    let files_dir = PathBuf::from("./tests/it_loads_metadata");
    setup(&files_dir);
    let cmd = format!(
        "load-metadata -f -a --hot-db-path {0} --files-dir {0}",
        files_dir.to_string_lossy()
    );
    assert_cmd_stdout(&cmd, "");

    let result_file = files_dir.join("sign_me_load_metadata_polkadotV30");
    let expected_file = PathBuf::from("./tests/for_tests/load_metadata_polkadotV30");
    assert_files_eq(&result_file, &expected_file);
    teardown(&files_dir);
}
