
# Crate `generate_message`

## Overview

This is a crate used to generate messages that could be parsed by Signer and to maintain the *hot* database.  

Crate is expected to work for message generation in two stages.  

First, the message payload is created either from the existing database or by fetching through rpc calls. Resulting message (in form of `Vec<u8>`) is saved in plaintext in `../files/for_signing` folder.  

This message could be then fed to signing tool, such as subkey, to generate a signature.  

After the plaintext export is used to create the signature, final message could be formed. Final message of `load_metadata`, `load_types` or `add_network` consists of:  

- 53xxyy, where xx is information about cryptography algorithm used, and yy is message type,  
- (if verified) public key of message verifier as hex line  
- message body as hex line  
- (if verified) signature of message verifier as hex line  

Final message could be exported as fountain qr and/or as textfile containing hex string.  


## Message types

Types of messages that could be generated:  

- 53xx80 `load_metadata` (contains concatenated metadata and genesis hash of a network)  
- 53xx81 `load_types` (contains SCALE encoded `Vec<TypeEntry>`)  
- 53xxc0 `add_network` (contains concatenated SCALE encoded metadata vector and SCALE encoded *shortened* network chainspecs); shortened network chainspecs are stored in struct ChainSpecsToSend, and compared to the ChainSpecs that are stored in the database have no order (it appears only if the network is loaded, and depends on the database actual contents) and no verifier  

Message `load_metadata` is used to load new versions of metadata for networks already in users database.  

Message `load_types` is used to load types information in users database, and hopefully will go obsolete soon. Should be used really rarely.  

Message `add_network` is normally used to add entirely new networks, that are not yet in users database.  


## Possible output formats

Functions from `generate_message` crate can generate hex line outputs as .txt files and/or generate fountain qr codes in apng format.  

Note that qr code generation could be quite resourse demanding operation, and better not be done on half-dying laptops.  


## Current usage

Database addressed by crate is `../database/database_hot` as set in `definitions` crate.  

Messages ready for signing are generated in `../files/for_signing/` folder, as set in `definitions` crate.  

Final signed messages (as qr codes or as text files) are generated in `../files/signed/` folder, as set in in `definitions` crate.  

Examples of names for intermediate files are:  
- `sign_me_add_network_kusamaV9070`
- `sign_me_load_metadata_polkadotV9050`
- `sign_me_add_network_with_defaults_nonameV50`
- `sign_me_load_types`

Final file could be optionally named through the `-name` key, however, default name is generated as well during the message consistency check-up.  
Examples of default file names are:  
for apng export:  
- `add_network_kusamaV9070_unverified`  
- `load_metadata_polkadotV9050_Alice`  
- `load_types`  
for text export:  
- `add_network_kusamaV9070_unverified.txt`  
- `load_metadata_polkadotV9050_Alice.txt`  
- `load_types.txt`  
Unverified is added for names of unverified files, Alice is added for names of test files verified by Alice.  
Normally verified files are unmarked.  

Program is run by  

`$ cargo run COMMAND [KEY(s)]`

Possible commands are:  
- `show` followed by a key:  
    - `-database` to show network `specname` and `spec_version` for all networks in the metadata tree the database  
    - `-address_book` to show network `specname` and url address for all networks in the address_book tree of the database  
- `types` without any keys to generate `load_types` message  
- `load` to `load_metadata` and `add` to `add_network` with following possible keys:  
    - setting keys (maximum one can be used):  
        - `-d`: do NOT update the database, make rpc calls, and produce ALL requested output files  
        - `-f`: do NOT run rps calls, produce ALL requested output files from existing database  
        - `-k`: update database through rpc calls, produce requested output files only for UPDATED database entries  
        - `-p`: update database through rpc calls, do NOT produce any output files  
        - `-t` default setting: update database through rpc calls, produce ALL requested output files  
    - reference keys (exactly only one has to be used):  
        - `-a`: process all networks (either in database or in the address book, depending on setting key)  
        - `-n` followed by one or more network names (such as in `-n polkadot westend`) to process networks by the name (either from database or from the address book, depending on setting key)  
        - `-u` followed by one or more url addresses to process network by url address (is incompatible with `-f` key)
    - optional `-s` key to stop the program if any failure occurs. By default the program informs user of unsuccessful attempt and proceeds.  
- `make` to `make_message` with following possible keys:  
    - optional content key: `-qr` will generate only apng qr code, `-text` will generate only text file with hex encoded message; by default, both qr code and text message are generated; content keys are expected immediately after `make` command, if at all; keys to follow could go in any order, but with content immediately following the key.  
    - key `-crypto` followed by encryption variant used in message verification:  
        - `ed25519`  
        - `sr25519`  
        - `ecdsa`  
        - `none` if the message is not verified  
    - key `-msgtype` followed by message type:  
        - `load_types`  
        - `load_metadata`  
        - `add_network`  
    - key `-verifier` (has to be entered if only the `-crypto` was `ed25519`, `sr25519`, or `ecdsa`), followed by:  
        - `Alice` to generate messages "verified" by Alice (used for tests)  
        - `-hex` followed by actual hex line of public key  
        - `-file` followed by file name ****, to read verifier public key as Vec<u8> from file named `****` from folder `../files/for_signing/`  
    - key `-payload` followed by `****` - file name to read message content as Vec<u8> from file named `****` from folder `../files/for_signing/`  
    - key `-signature` followed by:  
        - `-hex` followed by actual hex line of signature  
        - `-file` followed by file name ****, to read verifier signature as Vec<u8> from file named `****` from folder `../files/for_signing/`  
    - optional key `-name` followed by `****` - name override to save file named `****` for apng export and file named `****.txt` into folder `../files/signed/`  


## Examples:  

`$ cargo run types` to generate payload of `load_types` message from the database  

`$ cargo run load -a` to run rpc calls for all networks in `address_book` of the database to fetch current metadata, update the metadata entries in the database if needed, and generate the `load_metadata` messages for all networks; if an error occurs for one of the networks, program informs of that and proceeds to try others.  

`$ cargo run add -a` to run rpc calls for all networks in `address_book` of the database to fetch current metadata, update the metadata entries in the database if needed, and generate the `add_network` messages for all networks; if an error occurs for one of the networks, program informs of that and proceeds to try others.  

`$ cargo run make -crypto sr25519 -msgtype load_metadata -verifier -file mock_key -payload sign_me_load_metadata_kusamaV9070 -signature -file mock_signature` to create both apng and text files with default names with load_metadata content verified by given verified.  

`$ cargo run make -text -crypto sr25519 -msgtype add_network -verifier Alice -payload sign_me_add_network_kusamaV9070` to create text file "verified" by Alice with sr25519 encryption for add_network.  

`$ cargo run make -text -crypto sr25519 -msgtype load_types -verifier -hex 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d -payload sign_me_load_types -signature -hex 0x5a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe` to create text file of load_types "verified" by verifier with given hex public key with given signature.  

`$ cargo run load -a -k` to run rpc calls for all networks in `address_book` of the database to fetch current metadata, update the metadata entries in the database if needed, and generate the `load_metadata` message(s) for updated networks; if an error occurs for one of the networks, program informs of that and proceeds to try others.  

`$ cargo run load -n polkadot -d` run rpc call for `polkadot` using the address from `address_book` of the database to fetch current metadata, and generate the `load_metadata` message without updating the database  
