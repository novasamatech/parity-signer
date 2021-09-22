
# Crate `db_handling`

## Overview

This is a crate used to set to defaults *hot* and *cold* databases, and generally interact with `identities` part of database contents.  

Also, the database elements are used in tests of `transaction_parsing` and `transaction_signing` crates for relatively lighter databases generation.  

Crate is currently in draft stage and operable by uncommenting sections of `main.rs` as needed.  


## Instruction for a fresh start

This is for the cases when the databases are either missing entirely or have gone through major life changing updates.  

1. Remove the folder with databases. Just in case. They should be purging on their own.  

2. Populate *hot* database with defaults: default network specs for 4 networks (kusama, polkadot, rococo, westend), default types, default general verifier (none) and default address book with info of 4 networks. This is done by line `default_hot()?;` so uncomment it. Normally defaulting *hot* should not happen too often. Please comment it back as soon as done, so that the fetched metadata does not get accidentally deleted later on.  

3. If needed, fresh metadata sould be fetched into *hot* database using crate `generate_message` and its procedures. Simplest would be to run in `generate_message` command `$ cargo run load -a -p` to update metadata for all networks without printing any files. See documentation of `generate_message` for details.  

4. Populate *cold* database with defaults: default full network specs for 4 networks, default types, default general verifier (none), and default metadata read through regex from external file. This is done by function `default_cold()?;` so uncomment it.  

5. Add fresh metadata from *hot* database into *cold* database by uncommenting line `transfer_metadata(HOT_DB_NAME, COLD_DB_NAME)?;`. This step could be combined with previous one, please remember that `default_cold()?;` clears all entries and should be run first.  

Done! Both databases should be good to go.  


