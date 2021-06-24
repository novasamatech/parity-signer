use meta_reading::full_run;

fn main() {
    let database_name = "tests/metadata_database";
    let logfile_name = "tests/log";
    match full_run(database_name, logfile_name) {
        Ok(()) => (),
        Err(e) => println!("Application error. {}", e),
    }
}



