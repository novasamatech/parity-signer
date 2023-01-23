pub mod common;
use crate::common::{assert_cmd_stdout, assert_files_eq, setup};

use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn it_signs() {
    let files_dir = tempdir().unwrap();
    let db = sled::open(&files_dir).unwrap();

    setup(&db);
    drop(db);
    let cmd = format!(
        "load-metadata -f -a --hot-db-path {0} --files-dir {0}",
        files_dir.path().to_string_lossy()
    );
    assert_cmd_stdout(&cmd, "");

    let sign_cmd = format!(
        "make --goal text --crypto sr25519 --msg load-metadata \
        --payload sign_me_load_metadata_polkadotV30 --files-dir {0} --export-dir {0}",
        files_dir.path().to_string_lossy()
    );
    assert_cmd_stdout(&sign_cmd, "");

    let expected = PathBuf::from("./tests/for_tests/load_metadata_polkadotV30_unverified.txt");
    let unverified = files_dir
        .path()
        .join("load_metadata_polkadotV30_unverified.txt");
    assert_files_eq(&unverified, &expected);

    let sign_cmd = format!(
        "make --goal text --crypto sr25519 --msg load-metadata --verifier-alice sr25519 \
        --payload sign_me_load_metadata_polkadotV30 --files-dir {0} --export-dir {0}",
        files_dir.path().to_string_lossy()
    );
    assert_cmd_stdout(&sign_cmd, "");
    // Signing result is not deterministic, so we can't compare the result to a known
}
