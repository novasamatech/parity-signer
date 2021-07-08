// Copyright 2015-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#pragma once

#include <stdint.h>

// rust ffi

struct ExternError {
    int32_t code;
    char *message; // note: nullable
};

void signer_destroy_string(const char* cstring);

// ethkey ffi

// return keypair address, automatically picking BIP39 or parity phrases
const char* ethkey_brainwallet_address(struct ExternError*, const char* seed);

// return keypair address from BIP39 phrase
const char* ethkey_brainwallet_bip39_address(struct ExternError*, const char* seed);

// returns message signed with keypair
const char* ethkey_brainwallet_sign(struct ExternError*, const char* seed, const char* message);

// returns rlp item at given position
const char* rlp_item(struct ExternError*, const char* rlp, const unsigned position);

const char* keccak256(struct ExternError*, const char* data);

const char* blake(struct ExternError*, const char* data);

const char* eth_sign(struct ExternError*, const char* data);

const char* blockies_icon(struct ExternError*, const char* blockies_seed);

const char* random_phrase(struct ExternError*, int words_number);

const char* encrypt_data(struct ExternError*, const char* data, const char* password);

const char* decrypt_data(struct ExternError*, const char* encrypted_data, const char* password);

// qr code generator for utf-8 strings
const char* qrcode(struct ExternError*, const char* data);

// qr code generator for hex-encoded binary
const char* qrcode_hex(struct ExternError*, const char* data);

// ss58 address (including prefix) for sr25519 key generated out of BIP39 phrase
const char* substrate_brainwallet_address(struct ExternError*, const char* seed, const unsigned prefix);

const char* substrate_brainwallet_sign(struct ExternError* err, const char* seed, const char* data);

const char* schnorrkel_verify(struct ExternError*, const char* seed, const char* msg, const char* signature);

int64_t decrypt_data_ref(struct ExternError*, const char* encrypted_data, const char* password);

void destroy_data_ref(struct ExternError*, int64_t data_ref);

const char* ethkey_brainwallet_sign_with_ref(struct ExternError*, int64_t seed_ref, const char* message);

const char* substrate_brainwallet_sign_with_ref(struct ExternError*, int64_t seed_ref, const char* suri_suffix, const char* data);

const char* substrate_address_with_ref(struct ExternError*, int64_t seed_ref, const char* suri_suffix, const unsigned prefix);

const char* brain_wallet_address_with_ref(struct ExternError*, int64_t seed_ref);

const char* substrate_mini_secret_key_with_ref(struct ExternError*, int64_t seed_ref, const char* suri_suffix);

const char* substrate_mini_secret_key(struct ExternError*, const char* suri);

// qr fountain decoder
const char * try_decode_qr_sequence(struct ExternError*, int size, int chunk_size, const char* data);

// generate metadata handle from hex-encoded metadata string
// metadata: "0x..."
const char * generate_metadata_handle(struct ExternError*, const char* metadata);

// Parse transaction
// takes 2 strings:
// transaction, dbname (from OS)
// Returns decoded payload as serialized payload cards contents with following structure (JSON):
// {author:[...], warning:[...], error:[...], method:[...], extrinsic:[...]}
// Each card has following fields:
// index - to sort cards on screen, use as key in flatlist env
// indent - indentation to visualize cards hierarchy
// type - type of card
// payload - contents of card
const char * parse_transaction(struct ExternError*, const char* transaction, const char* dbname);

// Sign transaction
// wrapper to actually sign transaction in 1 simple call:
// takes 4 strings
// action: json-payload with action to perform (will be expanded later to include other actions
// including non-signing);
// pin code; and password
// dbname (from OS)
const char * sign_transaction(struct ExternError*, const char* action, const char* pin, const char* password, const char* dbname);

// Self descriptive: development test for channel
// TODO: remove for safety
const char * development_test(struct ExternError*, const char* input);

// Initial population of DB, currently in "development tool" state
// TODO: leave only dbname in input
void db_init(struct ExternError*, const char* metadata, const char* dbname);

// Fetch list of available networks for network selector screen
const char * get_all_networks_for_network_selector(struct ExternError*, const char* dbname);

// Fetch one network for general display purposes
const char * get_network(struct ExternError*, const char* genesis_hash, const char* dbname);

// Fetch list of all root seeds
const char * get_all_seed_names(struct ExternError*, const char* dbname);

//Filter identities derived for given seed and network
const char * get_relevant_identities(struct ExternError*, const char* seed_name, const char* genesis_hash, const char* dbname);

//Suggest next numbered path
const char * suggest_n_plus_one(struct ExternError*, const char* path, const char* seed_name, const char* network_id_string, const char* dbname);

//Check validity of proposed path and find password
bool check_path(struct ExternError*, const char* path);

//Acknowledge user agreement
void ack_user_agreement(struct ExternError*, const char* dbname);

//Check whether user agreement is acknowledged
bool check_user_agreement(struct ExternError*, const char* dbname);

//Function to create new seed
void try_create_seed(struct ExternError*, const char* seed_name, const char* crypto, const char* seed_phrase, int seed_length, const char* dbname);

//Function to create new address
void try_create_identity(struct ExternError*, const char* id_name, const char* seed_name, const char* seed_phrase, const char* crypto, const char* path, const char* network, bool has_password, const char* dbname);
