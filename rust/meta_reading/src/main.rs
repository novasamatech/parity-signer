use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::{DateTime, Utc};
use regex::Regex;

use meta_reading::*;

fn main() {

// existing metadata is stored in networkMetadata.ts file, this part fetches the metadata as MetaValues from file
    let contents = fs::read_to_string("networkMetadata.ts");
    let old_full: Vec<MetaValues> = match contents {
        Ok(c) => split_properly(&c),
        Err(_) => Vec::new(),
    };
// Sorting old metadata into most recent group - one entry with latest version for each chain name (the one to check against), and the historical group (keep at most one historical entry for each chain name). "No version" is by default older than some version.
    let mut existing = split_existing_metadata(old_full);

// address book is stored in address_book text file, to be updated if necessary; this part fetches address book entries
    let filename = "address_book";
    let contents = fs::read_to_string(filename).expect("No address book found.");
    let address_book = get_addresses(&contents);
    
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
// writing default lines in networkMetadataList.ts file
        let mut found_kusama = false;
        let mut found_polka = false;
        for x in existing.latest.iter() {
            if x.name == "kusama" {
                match x.version {
                    Some(version) => {
                        if let Err(e) = writeln!(file2, "{}", format!("export const defaultMetadata = metadata.{}MetadataV{};\n", x.name, version)) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    },
                    None => {
                        if let Err(e) = writeln!(file2, "{}", format!("export const defaultMetadata = metadata.{}Metadata;\n", x.name)) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    }
                }
                found_kusama = true;
            }
            if x.name == "polkadot" {
                match x.version {
                    Some(version) => {
                        if let Err(e) = writeln!(file2, "{}", format!("export const defaultPolkadotMetadata = metadata.{}MetadataV{};\n", x.name, version)) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    },
                    None => {
                        if let Err(e) = writeln!(file2, "{}", format!("export const defaultPolkadotMetadata = metadata.{}Metadata;\n", x.name)) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    }
                }
                found_polka = true;
            }
            if found_kusama && found_polka {break;}
        }
        if !found_kusama {
            eprintln!("Default kusama not found. No default printed to networkMetadataList.ts");
        }
        if !found_polka {
            eprintln!("Default polkadot not found. No default printed to networkMetadataList.ts");
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
                    if let Err(e) = writeln!(file2, "{}", format!("\tmetadata.{}MetadataV{},", x.name, version)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                },
                None => {
                // writing into networkMetadata.ts file
                    if let Err(e) = writeln!(file1, "{}", format!("export const {}Metadata = '{}';", x.name, x.meta)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                // writing into networkMetadataList file
                    if let Err(e) = writeln!(file2, "{}", format!("\tmetadata.{}Metadata,", x.name)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
        }
// writing historical metadata into networkMetadata.ts and into networkMetadataList.ts
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
                // writing into networkMetadataList file
                    if let Err(e) = writeln!(file2, "{}", format!("\tmetadata.{}MetadataV{},", x.name, version)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                },
                None => {
                // writing into networkMetadata.ts file
                    if let Err(e) = writeln!(file1, "{}", format!("export const {}Metadata = '{}';", x.name, x.meta)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                // writing into networkMetadataList file
                    if let Err(e) = writeln!(file2, "{}", format!("\tmetadata.{}Metadata,", x.name)) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
        }
// completing networkMetadataList.ts
        if let Err(e) = writeln!(file2, "];") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
// Updating networkSpecs.ts file
    if let Err(e) = writeln!(file, "\nUpdating networkSpecs.ts file") {
        eprintln!("Couldn't write to file: {}", e);
    }

    let re = Regex::new(r#"metadata: \{\n.*hash: '(?P<hash>[^']+)',\n.*specName: '(?P<name>[^']+)',\n.*specVersion: (?P<vers>[0-9]*)\n.*\},\n"#).unwrap();
    let rehash = Regex::new(r#"hash: '(?P<hash>[^']+)'"#).unwrap();
    let revers = Regex::new(r#"specVersion: (?P<vers>[0-9]*)"#).unwrap();
    let old_specs = fs::read_to_string("networkSpecs.ts").unwrap();
    let mut new_specs = fs::read_to_string("networkSpecs.ts").unwrap();
    
    for caps in re.captures_iter(&old_specs) {
        if let Err(e) = writeln!(file, "* Updating {}", &caps["name"]) {
            eprintln!("Couldn't write to file: {}", e);
        }
        let mut found_flag = false;
        for x in existing.latest.iter() {
            if &caps["name"] == x.name {
                let mut new_line = caps[0].to_string();
                let hash_real = match hash_from_meta(&x.meta) {
                    Some(a) => a,
                    None => {
                        eprintln!("Couldn't calculate hash for: {}", x.name);
                        String::from("")
                    },
                };
                let hash_line = format!("hash: '{}'", hash_real);
                new_line = rehash.replace(&new_line, hash_line).into_owned();
                let ver_real = match x.version{
                    Some(v) => v,
                    None => 0,
                };
                let ver_line = format!("specVersion: {}", ver_real);
                new_line = revers.replace(&new_line, ver_line).into_owned();
                new_specs = new_specs.replace(&caps[0], &new_line);
                found_flag = true;
                if let Err(e) = writeln!(file, "S OK") {
                    eprintln!("Couldn't write to file: {}", e);
                }
                break;
            }
        }
        if !found_flag {
            if let Err(e) = writeln!(file, "E Name not found. Check manually.") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
    let mut ns_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("networkSpecs.ts")
        .unwrap();
    if let Err(e) = write!(ns_file, "{}", new_specs) {
        eprintln!("Couldn't write to file: {}", e);
    }
}



