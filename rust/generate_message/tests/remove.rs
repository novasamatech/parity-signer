pub mod common;
use crate::common::{assert_cmd_stdout, setup, teardown};

#[test]
fn it_removes() {
    let db_path = "./tests/it_removes";
    setup(db_path);

    assert_cmd_stdout(
        &format!("show networks --hot-db-path {}", db_path),
        "Address book has entries for following networks:\n
polkadot at wss://rpc.polkadot.io, encryption sr25519, Signer display title Polkadot\n",
    );
    assert_cmd_stdout(
        &format!("remove title polkadot --hot-db-path {}", db_path),
        "",
    );
    assert_cmd_stdout(
        &format!("show networks --hot-db-path {}", db_path),
        "Address book is empty.\n",
    );

    teardown(db_path);
}
