pub mod common;

use crate::common::{assert_cmd_stdout, setup, teardown};
use std::path::PathBuf;

#[test]
fn it_removes() {
    let files_dir = PathBuf::from("./tests/it_removes");
    setup(&files_dir);
    assert_cmd_stdout(
        &format!(
            "show networks --hot-db-path {0}",
            files_dir.to_string_lossy()
        ),
        "Address book has entries for following networks:\n
polkadot at wss://rpc.polkadot.io, encryption sr25519, Signer display title Polkadot\n",
    );
    assert_cmd_stdout(
        &format!(
            "remove title polkadot --hot-db-path {}",
            files_dir.to_string_lossy()
        ),
        "",
    );
    assert_cmd_stdout(
        &format!(
            "show networks --hot-db-path {}",
            files_dir.to_string_lossy()
        ),
        "Address book is empty.\n",
    );

    teardown(&files_dir);
}
