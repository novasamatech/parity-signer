use crate::common::{assert_cmd_stdout, setup, teardown};

pub mod common;

pub fn run_cmd_test(command: &str, output: &'static str, db_path: &str) {
    let cmd = &format!("{} --hot-db-path {}", command, db_path);
    setup(db_path);
    assert_cmd_stdout(cmd, output);
    teardown(db_path);
}

#[test]
fn it_shows_block_history() {
    run_cmd_test(
        "show block-history",
        "Database has no metadata fetch history entries on record.\n",
        "./tests/it_shows_block_history",
    );
}

#[test]
fn it_shows_networks() {
    run_cmd_test(
        "show networks",
        "Address book has entries for following networks:

polkadot at wss://rpc.polkadot.io, encryption sr25519, Signer display title Polkadot\n",
        "./tests/it_shows_networks",
    );
}

#[test]
fn it_shows_metadata() {
    run_cmd_test(
        "show metadata",
        "Database has metadata information for following networks:

kusama 2030, metadata hash efaa97434a2e971067e5819f6f80e892daeb2711ac0544a4e8260d4ff0c14270, no block hash on record
westend 9000, metadata hash e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce, no block hash on record
westend 9010, metadata hash 70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf, no block hash on record
polkadot 30, metadata hash 93b9065e4a6b8327ca1ce90e9ac3d7d967a660dcc5cda408e2595aa3e5c1ab46, no block hash on record\n",
        "./tests/it_shows_metadata",
        );
}

#[test]
fn it_shows_specs() {
    run_cmd_test(
        "show specs polkadot",
        "address book title: polkadot
base58 prefix: 0
color: #E6027A
decimals: 10
encryption: sr25519
genesis_hash: 91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3
logo: polkadot
name: polkadot
path_id: //polkadot
secondary_color: #262626
title: Polkadot
unit: DOT\n",
        "./tests/it_shows_specs",
    );
}
