use sled::Db;

pub type SeedKey = Vec<u8>;
pub type AddressKey = Vec<u8>;
pub type NetworkKey = Vec<u8>;

///These abstractions are in place in case more complexity is needed
///They should not fail

///Generate seed key from minimal amount of information
pub fn generate_seed_key (name: &str) -> SeedKey {
    name.as_bytes().to_vec()
}

///Generate address key from minimal amount of information
pub fn generate_address_key (public: Vec<u8>) -> AddressKey {
    public
}

///Generate network key from minimal amount of information
pub fn generate_network_key (genhash: Vec<u8>) -> NetworkKey {
    genhash
}

///This could replace Ok(()) at the end of all db-opening functions
///

pub fn db_flush_check (database: &Db) -> Result<(), Box<dyn std::error::Error>> {
    match database.flush() {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::from(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sled::open;

    #[test]
    fn successfully_flush_db() {
        let database: Db = open("tests/test_flush_db").unwrap();
        db_flush_check(&database).unwrap()
    }
}
