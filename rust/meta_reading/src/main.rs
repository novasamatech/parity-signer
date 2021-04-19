use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::{DateTime, Utc};

use meta_reading::*;

fn main() {

// existing metadata is stored in networkMetadata.ts file, this part fetches the metadata as MetaValues from file
    let contents = fs::read_to_string("networkMetadata.ts");
    let old_full: Vec<MetaValues> = match contents {
        Ok(c) => {
            c
            .lines()
            .filter(|line| line.contains("export const"))
            .map(|line| split_properly(line))
            .collect()
        },
        Err(_) => Vec::new(),
    };
// Sorting old metadata into most recent group - one entry with latest version for each chain name (the one to check against), and the historical group (keep at most one historical entry for each chain name). "No version" is by default older than some version.
    let mut existing = split_existing_metadata(old_full);

// address book is stored in address_book text file, to be updated if necessary; this part fetches address book entries
    let filename = "address_book";
    let contents = fs::read_to_string(filename).expect("No address book found.");
    let address_book: Vec<AddressBookEntry> = {
        contents
        .lines()
        .map(|line| get_address(line))
        .collect()
    };
// getting the time stamp before fetching the metadata
    let timestamp_before: DateTime<Utc> = Utc::now();
// fetching the metadata from addresses provided; that one is slow;
    let meta_book: Vec<MetaEntry> = { 
        address_book
        .iter()
        .map(|x| make_meta_entry(x))
        .collect()
    };
// getting the time stamp after fetching the metadata
    let timestamp_after: DateTime<Utc> = Utc::now();
// creating log file
    let log_file_name = "metadata_fetching_log.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_name)
        .unwrap();
    if let Err(e) = writeln!(file, "{}", format!("======\nMetadata fetching started: {}", timestamp_before)) {
        eprintln!("Couldn't write to file: {}", e);
    }
    let mut counter = 0;
// processing all meta_book entries
    for x in meta_book.iter() {
// write name in log
        if let Err(e) = writeln!(file, "{}", format!("\n* Updating {}:", x.name)) {
            eprintln!("Couldn't write to file: {}", e);
        }
// checking if metadata is fetched, updating existing metadata, writing log
        match &x.meta {
            Ok(y) => {
                let res_const = get_meta_const(&y);
                let mut new = MetaValues {
                    name: x.name.to_owned(),
                    version: None,
                    meta: y.to_string(),
                };
                match res_const {
                    Ok(z) => {
                        let decoded = decode_version(z);
                        new.name = decoded.specname;
                        new.version = Some(decoded.spec_version);
                    },
                    Err(e) => {
                        if let Err(e) = writeln!(file, "{}", format!("W {}", e)) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    },
                };
                let upd = update(new, existing, log_file_name);
                existing = upd.data;
                if upd.flag {
                    counter = counter+1;
                }
            },
            Err(e) => {
                if let Err(e) = writeln!(file, "{}", format!("E {}", e.to_string())) {
                    eprintln!("Couldn't write to file: {}", e);
                }
                let name_for_search = x.name.to_owned();
                let upd = sar(name_for_search, existing, log_file_name);
                existing = upd.data;
                if upd.flag {
                    counter = counter+1;
                }
            },
        }
    }
// 

// completing the log file
    if let Err(e) = writeln!(file, "{}", format!("\nMetadata fetching finished: {}", timestamp_after)) {
        eprintln!("Couldn't write to file: {}", e);
    }
// commenting log file
    if counter == 0 {
        if let Err(e) = writeln!(file, "\n No updates made.") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    else {
// making new networkMetadata.ts file
        let mut file1 = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("networkMetadata.ts")
            .unwrap();
        let prelude1 = fs::read_to_string("text_parts/networkMetadata_text");
        let prelude1 = match prelude1 {
            Ok(t) => t,
            Err(_) => {
                eprintln!("Prelude to networkMetadata.ts lost");
                String::new()
            },
        };
        if let Err(e) = writeln!(file1, "{}", format!("{}\n\n// latest metadata versions:\n", prelude1)) {
            eprintln!("Couldn't write to file: {}", e);
        }
// making new networkMetadataList.ts file
        let mut file2 = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("networkMetadataList.ts")
            .unwrap();
        let prelude2 = fs::read_to_string("text_parts/networkMetadataList_text");
        let prelude2 = match prelude2 {
            Ok(t) => t,
            Err(_) => {
                eprintln!("Prelude to networkMetadataList.ts lost");
                String::new()
            },
        };
        if let Err(e) = writeln!(file2, "{}", prelude2) {
            eprintln!("Couldn't write to file: {}", e);
        }
// writing default line in networkMetadataList.ts file
        let mut found_kusama = false;
        for x in existing.latest.iter() {
            if x.name == "kusama" {
                match x.version {
                    Some(version) => {
                        if let Err(e) = writeln!(file2, "{}", format!("export const defaultMetaData = metadata.{}MetadataV{};\n", x.name, version)) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    },
                    None => {
                        if let Err(e) = writeln!(file2, "{}", format!("export const defaultMetaData = metadata.{}Metadata;\n", x.name)) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    }
                }
                found_kusama = true;
                break;
            }
        }
        if !found_kusama {
            eprintln!("Default kusama not found. No default printed to networkMetadataList.ts");
        }
// adding to networkMetadataList.ts file
        if let Err(e) = writeln!(file2, "export const allBuiltInMetadata = [") {
            eprintln!("Couldn't write to file: {}", e);
        }
// writing latest metadata into networkMetadata.ts and into networkMetadataList.ts
        for x in existing.latest.iter() {
            match x.version {
                Some(version) => {
                // writing into networkMetadata.ts file
                    if let Err(e) = writeln!(file1, "{}", format!("export const {}MetadataV{} = '{}';", x.name, version, x.meta)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                // writing into networkMetadataList file
                    if let Err(e) = writeln!(file2, "{}", format!("	metadata.{}MetadataV{},", x.name, version)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                },
                None => {
                // writing into networkMetadata.ts file
                    if let Err(e) = writeln!(file1, "{}", format!("export const {}Metadata = '{}';", x.name, x.meta)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                // writing into networkMetadataList file
                    if let Err(e) = writeln!(file2, "{}", format!("	metadata.{}Metadata,", x.name)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
        }
// completing networkMetadataList.ts
        if let Err(e) = writeln!(file2, "];") {
            eprintln!("Couldn't write to file: {}", e);
        }
// writing historical metadata into networkMetadata.ts
        if let Err(e) = writeln!(file1, "\n\n// historical metadata versions:\n") {
            eprintln!("Couldn't write to file: {}", e);
        }
        for x in existing.historical.iter() {
            match x.version {
                Some(version) => {
                // writing into networkMetadata.ts file
                    if let Err(e) = writeln!(file1, "{}", format!("export const {}MetadataV{} = '{}';", x.name, version, x.meta)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
                None => {
                // writing into networkMetadata.ts file
                    if let Err(e) = writeln!(file1, "{}", format!("export const {}Metadata = '{}';", x.name, x.meta)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
        }
    }
}



