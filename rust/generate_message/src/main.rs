use std::env;
use generate_message::{full_run, parser::Command};

fn main() -> Result<(), String> {

    let command = match Command::new(env::args()) {
        Ok(x) => x,
        Err(e) => return Err(format!("Error parsing arguments. {}", e)),
    };
    
    match full_run(command) {
        Ok(()) => Ok(()),
        Err(e) => return Err(format!("Application error. {}", e)),
    }
    
}
