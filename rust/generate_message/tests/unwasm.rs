pub mod common;
use crate::common::{assert_cmd_stdout, assert_files_eq, setup};

use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn it_unwasm() {
    let files_dir = tempdir().unwrap();
    let db = sled::open(&files_dir).unwrap();
    setup(&db);

    drop(db);
    let cmd = format!(
        "unwasm --filename ./tests/for_tests/polkadot.wasm --update-db --hot-db-path {0} --files-dir {0}",
        files_dir.path().to_string_lossy()
    );
    assert_cmd_stdout(&cmd, "Unwasmed new metadata polkadot9270\n");

    let result_file = files_dir.path().join("sign_me_load_metadata_polkadotV9270");
    let expected_file = PathBuf::from("./tests/for_tests/sign_me_load_metadata_polkadotV9270");
    assert_files_eq(&result_file, &expected_file);
}
