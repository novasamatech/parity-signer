
## How to sign metadata payload using Signer  

1. In crate `generate_message`:  
`$ cargo run restore_defaults`  

2. In crate `generate_message`:  
`$ cargo run load_metadata -a`  

3. In crate `generate_message`:  
`$ cargo run show -database`  
This will produce the list of metadata currently in the database.  
At the moment, I have this:  
	kusama 9111  
	rococo 9106  
	westend 9120  
	polkadot 9110  
Say, we are doing the signing of polkadot 9110.  

4. In crate `generate_message`:  
`$ cargo run make -qr -crypto none -msgtype load_metadata -payload sign_me_load_metadata_polkadotV9110`  
This will generate apng qr code (with unsigned info) in `../files/signed` folder.  
Look for file named `load_metadata_polkadotV9110_unverified`, it should be somewhat large (few MiB).  

If the Signer has preset general verifier, this will not work for default networks (and polkadot is one of 4 default networks).  

If the general verifier in Signer is set to be Alice in sr25519 encryption (as it was in testing stages), run in crate `generate_message` instead:  
`$ cargo run make -qr -crypto sr25519 -msgtype load_metadata -payload sign_me_load_metadata_polkadotV9110 -verifier Alice`  
This will generate apng qr code (with info signed by Alice) in `../files/signed` folder.  
Look for file named `load_metadata_polkadotV9110_Alice-sr25519`, it should also be somewhat large (few MiB).  

5. Feed qr code from step 4 into your good Signer. Accept the metadata, then in Signer make qr to verify metadata.  

6. Read this qr from PC. You can use crate `qr_reader_pc` (just `$ cargo run` with camera on qr, wait for hex output).  
Warning: no graphic interface as of yet.  

7. In crate `generate_message`:  
`$ cargo run sign -qr -sufficient -hex ****** -msgtype load_metadata -payload sign_me_load_metadata_polkadotV9110`  
Instead of ****** goes the hex line you've got in step 6.  

8. Done.  
Generated apng file with signed content should be in `../files/signed` folder, named `load_metadata_polkadotV9110`.  
