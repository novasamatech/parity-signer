pub mod common;

use crate::common::{assert_cmd_stdout, setup};

use tempfile::tempdir;

#[test]
fn it_removes() {
    let files_dir = tempdir().unwrap();
    let db = sled::open(&files_dir).unwrap();

    setup(&db);
    drop(db);
    assert_cmd_stdout(
        &format!(
            "show networks --hot-db-path {0}",
            files_dir.path().to_string_lossy()
        ),
        "Address book has entries for following networks:\n
polkadot at wss://rpc.polkadot.io, encryption sr25519, Signer display title Polkadot\n",
    );
    assert_cmd_stdout(
        &format!(
            "remove title polkadot --hot-db-path {}",
            files_dir.path().to_string_lossy()
        ),
        "",
    );
    assert_cmd_stdout(
        &format!(
            "show networks --hot-db-path {}",
            files_dir.path().to_string_lossy()
        ),
        "Address book is empty.\n",
    );
}
