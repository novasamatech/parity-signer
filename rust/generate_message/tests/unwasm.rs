pub mod common;
use crate::common::{assert_cmd_stdout, remove_if_exists, setup, teardown};
use constants::FOLDER;
use std::fs;
use std::path::Path;

#[test]
fn it_unwasm() {
    let db_path = "./tests/unwasm";
    setup(db_path);

    let f_path = format!("{}/sign_me_load_metadata_polkadotV9270", FOLDER);
    remove_if_exists(&f_path);

    assert_cmd_stdout(
        "unwasm --filename ./tests/for_tests/polkadot.wasm --update-db",
        "Unwasmed new metadata polkadot9270\n",
        db_path,
    );

    // Confirm that the new metadata is in the database.
    assert_cmd_stdout(
        "show metadata",
        "Database has metadata information for following networks:

kusama 2030, metadata hash efaa97434a2e971067e5819f6f80e892daeb2711ac0544a4e8260d4ff0c14270, no block hash on record
westend 9000, metadata hash e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce, no block hash on record
westend 9010, metadata hash 70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf, no block hash on record
polkadot 30, metadata hash 93b9065e4a6b8327ca1ce90e9ac3d7d967a660dcc5cda408e2595aa3e5c1ab46, no block hash on record
polkadot 9270, metadata hash 01dfb0d5c7bebc31f1027cfd6bdc2a50295b3e39ee97e19156471044597ee20b, no block hash on record\n",
        db_path);

    let f_path = format!("{}/sign_me_load_metadata_polkadotV9270", FOLDER);
    assert!(Path::new(&f_path).exists());
    fs::remove_file(&f_path).unwrap();
    teardown(db_path);
}
