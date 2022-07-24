use generate_message::{full_run, parser::Command, Error};
use std::env;

fn main() -> Result<(), Error> {
    let command = Command::new(env::args())?;
    full_run(command).unwrap();
    Ok(())
}
