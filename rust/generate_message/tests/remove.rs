pub mod common;
use crate::common::{assert_cmd_stdout, setup, teardown};

#[test]
fn it_removes() {
    let db_path = "./tests/it_removes";
    setup(db_path);

    assert_cmd_stdout(
        "show networks",
        "Address book has entries for following networks:\n
polkadot at wss://rpc.polkadot.io, encryption sr25519, Signer display title Polkadot\n",
        db_path,
    );
    assert_cmd_stdout("remove title polkadot", "", db_path);
    assert_cmd_stdout("show networks", "Address book is empty.\n", db_path);

    teardown(db_path);
}
