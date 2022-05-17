use definitions::{error::ErrorSource, error_active::Active};
use generate_message::{full_run, parser::Command};
use std::env;

fn main() -> Result<(), String> {
    let command = match Command::new(env::args()) {
        Ok(a) => a,
        Err(e) => return Err(<Active>::show(&e)),
    };
    match full_run(command) {
        Ok(_) => Ok(()),
        Err(e) => Err(<Active>::show(&e)),
    }
}
