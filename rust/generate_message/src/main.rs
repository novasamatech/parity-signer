use clap::Parser;
use generate_message::{full_run, parser::Cli};

fn main() {
    let args = Cli::parse();
    if let Err(e) = full_run(args.command) {
        eprintln!("An error occurred: {}", e);
    }
}
