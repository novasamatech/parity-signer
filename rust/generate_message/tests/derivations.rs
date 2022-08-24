pub mod common;
use crate::common::{assert_cmd_stdout, assert_files_eq, remove_if_exists, setup};

use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn it_derives() {
    let files_dir = tempdir().unwrap();
    setup(&files_dir);
    let cmd = format!(
        "derivations --title polkadot --goal text --derivations //1 --hot-db-path {}",
        files_dir.path().to_string_lossy()
    );
    assert_cmd_stdout(&cmd, "Found and used 1 valid derivations:\n\"//1\"\n");

    let result_file = PathBuf::from("derivations-polkadot.txt");
    let expected_file = PathBuf::from("./tests/for_tests/derivations-polkadot.txt");
    assert_files_eq(&expected_file, &result_file);
    remove_if_exists(&result_file);
}
