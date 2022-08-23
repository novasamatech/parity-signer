pub mod common;
use crate::common::base_cmd;

#[test]
fn it_shows_help() {
    base_cmd().arg("--help").assert().success().code(0);
}
