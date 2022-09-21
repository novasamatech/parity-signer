use std::process::ExitCode;

use clap::Parser;
use generate_message::{full_run, parser::Cli};

fn main() -> ExitCode {
    let args = Cli::parse();
    if let Err(e) = full_run(args.command) {
        eprintln!("An error occurred: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
