pub mod add;
    use add::gen_add_network;
pub mod load;
    use load::gen_load_meta;
pub mod parser;
    use parser::{Command, Show};
pub mod show;
    use show::{show_database, show_address_book};
pub mod types;
    use types::gen_types;
pub mod metadata_db_utils;
pub mod metadata_shortcut;
pub mod output_prep;
pub mod make_message;
    use make_message::make_message;


/// Function to process incoming command as interpreted by parser

pub fn full_run (command: Command) -> Result<(), String> {
    
    match command {
        Command::Show(x) => {
            match x {
                Show::Database => {
                    match show_database() {
                        Ok(()) => Ok(()),
                        Err(e) => return Err(format!("Error showing metadata from the hot database. {}", e)),
                    }
                },
                Show::AddressBook => {
                    match show_address_book() {
                        Ok(()) => Ok(()),
                        Err(e) => return Err(format!("Error showing address book from the hot database. {}", e)),
                    }
                },
            }
        },
        Command::Types => {
            match gen_types() {
                Ok(()) => Ok(()),
                Err(e) => return Err(format!("Error generating load_types message. {}", e)),
            }
        },
        Command::Load(instruction) => {
            match gen_load_meta(instruction) {
                Ok(()) => Ok(()),
                Err(e) => return Err(format!("Error generating load_meta message(s). {}", e)),
            }
        },
        Command::Add(instruction) => {
            match gen_add_network(instruction) {
                Ok(()) => Ok(()),
                Err(e) => return Err(format!("Error generating add_network message(s). {}", e)),
            }
        },
        Command::Make(make) => {
            match make_message(make) {
                Ok(()) => Ok(()),
                Err(e) => return Err(format!("Error assembling message. {}", e)),
            }
        }
    }
    
}

