use sled::Db;

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

    static TESTDB: &str = "tests/testdb";

    //These tests are blocking each other due to DB access;
    //TODO: either consolidate tests into single thread
    //or give each a separate db to mess with
    #[test]
    #[ignore]
    fn successfully_flush_db() {
        let database: Db = open(TESTDB).unwrap();
        db_flush_check(&database).unwrap()
    }
}
