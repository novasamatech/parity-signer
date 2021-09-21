use anyhow;
use db_handling::default_hot;

mod add;
    use add::gen_add_network;
mod error;
mod helpers;
mod load;
    use load::gen_load_meta;
pub mod parser;
    use parser::{Command, Show};
mod remove;
    use remove::remove_info;
mod show;
    use show::{show_database, show_address_book};
mod specs;
    use specs::gen_add_specs;
mod types;
    use types::gen_types;
mod metadata_db_utils;
mod metadata_shortcut;
mod output_prep;
mod make_message;
    use make_message::make_message;


/// Function to process incoming command as interpreted by parser

pub fn full_run (command: Command) -> anyhow::Result<()> {
    
    match command {
        Command::Show(x) => {
            match x {
                Show::Database => show_database(),
                Show::AddressBook => show_address_book(),
            }
        },
        Command::Types => gen_types(),
        Command::Load(instruction) => gen_load_meta(instruction),
        Command::Add(instruction) => gen_add_network(instruction),
        Command::Specs(instruction) => gen_add_specs(instruction),
        Command::Make(make) => make_message(make),
        Command::Remove(info) => remove_info(info),
        Command::RestoreDefaults => default_hot(),
    }
}

