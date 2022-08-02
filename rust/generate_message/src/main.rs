use clap::Parser;
use generate_message::{full_run, parser::Cli, Error};

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    full_run(args.command).unwrap();
    Ok(())
}
