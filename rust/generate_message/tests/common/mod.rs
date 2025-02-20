mod data;

use assert_cmd::Command;
use db_handling::db_transactions::TrDbHot;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn setup(database: &sled::Db) {
    TrDbHot::new()
        .set_address_book(data::address_book())
        .set_network_specs_prep(data::network_specs_prep())
        .set_metadata(data::metadata())
        .set_settings(data::settings())
        .apply(database)
        .unwrap();
}

pub fn base_cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

pub fn assert_cmd_stdout(command: &str, output: &'static str) {
    base_cmd()
        .args(command.split(' ').collect::<Vec<&str>>())
        .assert()
        .success()
        .code(0)
        .stdout(output);
}

pub fn assert_files_eq<P: AsRef<Path>>(f1: P, f2: P) {
    let mut f1 = File::open(f1).unwrap();
    let mut buf1 = Vec::new();
    f1.read_to_end(&mut buf1).unwrap();

    let mut f2 = File::open(f2).unwrap();
    let mut buf2 = Vec::new();
    f2.read_to_end(&mut buf2).unwrap();

    assert_eq!(buf1, buf2);
}

pub fn remove_if_exists<P: AsRef<Path>>(path: P) {
    if Path::new(path.as_ref()).exists() {
        std::fs::remove_file(path.as_ref()).unwrap();
    }
}
