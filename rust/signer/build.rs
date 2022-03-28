use definitions::error::{Active, ErrorSource};
use generate_message::{full_run, parser::Command};

fn main() -> Result<(), String> {
    let command = Command::MakeColdRelease;
    match full_run(command) {
        Ok(_) => Ok(()),
        Err(e) => return Err(<Active>::show(&e)),
    }
}
