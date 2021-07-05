use meta_reading::default_full_run_with_chainspecs;
//use meta_reading::add_from_address;


fn main() {
    let database_name = "tests/metadata_database";
    let logfile_name = "tests/log";
//    let address = "wss://mainnet-node.dock.io";
    match default_full_run_with_chainspecs(database_name, logfile_name) {
        Ok(()) => (),
        Err(e) => println!("Application error. {}", e),
    }

//    match add_from_address (address, database_name, logfile_name) {
//        Ok(()) => (),
//        Err(e) => println!("Application error. {}", e),
//    }

}



