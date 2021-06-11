use sled::{Db, open};
use std::fs;
use db_handling::{DataFiles, fill_database_from_files};

fn main() -> Result<(), Box<dyn std::error::Error>> {

// creating the database from the files

    let dbname = "tests/signer_database";
    let types_info = match fs::read_to_string("new_full_types") {
        Ok(x) => x,
        Err(_) => return Err(Box::from("Type database missing")),
    };
    
    let chain_spec_database = match fs::read_to_string("database_output") {
        Ok(x) => x,
        Err(_) => return Err(Box::from("Chain spec database missing")),
    };
    
    let metadata_contents = match fs::read_to_string("metadata_database.ts") {
        Ok(x) => x,
        Err(_) => return Err(Box::from("Metadata database missing")),
    };
    
    let identities = match fs::read_to_string("identities") {
        Ok(x) => x,
        Err(_) => return Err(Box::from("Identities database missing")),
    };
    
    let datafiles = DataFiles {
        chain_spec_database: &chain_spec_database,
        metadata_contents: &metadata_contents,
        types_info: &types_info,
        identities: &identities,
    };
    
    fill_database_from_files(dbname, datafiles)?;
    
    let database: Db = open(dbname)?;
    
    for x in database.tree_names().iter() {
        println!("{}", String::from_utf8(x.to_vec())?);
    }
    
    Ok(())

/*    
    let database: Db = open("signer_database").unwrap();
    let settings: Tree = database.open_tree(b"settings").unwrap();
    let types_data = settings.get(String::from("types").encode()).unwrap().unwrap();
    println!("{:?}", types_data);
*/    
/*
// reading already created database
    let database: Db = open("signer_database").unwrap();
    let chain_specs: Tree = database.open_tree(b"chain_specs").unwrap();
    let metadata: Tree = database.open_tree(b"metadata").unwrap();
    
    let found_genesis_hash = hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    
    let specs_line = chain_specs.get(found_genesis_hash).unwrap().unwrap();
    let specs_line_decoded = <ChainSpecsStorage>::decode(&mut &specs_line[..]).unwrap();
    
    let to_search = specs_line_decoded.name.encode();
//    println!("{:?}, its length: {}", to_search, to_search.len());
    for x in metadata.scan_prefix(&to_search) {
        let versioned_name = x.unwrap().0;
//        println!("{:?}", versioned_name);
        let version = &versioned_name[to_search.len()..];
        println!("{} version {} available!", specs_line_decoded.name, <u32>::decode(&mut &version[..]).unwrap());
    }
*/
}

