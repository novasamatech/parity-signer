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

// qr code generator for utf-8 strings
const char* qrcode(struct ExternError*, const char* data);

// qr frame count estimator
int get_packets_total(struct ExternError*, const char* data);

// qr fountain decoder
const char * try_decode_qr_sequence(struct ExternError*, const char* data);

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
// In addition to these cards, an action object is produced that should be passed to handle_action function
// This function does not perform crypto operations and does not produce any side-effects
// other than formation of "action" record in db that could be used by handle_action function
const char * parse_transaction(struct ExternError*, const char* transaction, const char* dbname);

// Handle action;
// This function performs cryptographic signing or permanently changes Signer's state by modifying
// networks information
// takes 4 strings
// action - action descriptor with checksum produced in parse_transaction
// seed phrase (plaintext)
// password - optional derivation password (///password)
// dbname (from OS)
// Returns QR payload string or plaintext statement of action result
const char * handle_action(struct ExternError*, const char* action, const char* seed_phrase, const char* password, const char* dbname);

// Self descriptive: development test for channel
// TODO: remove for safety
const char * development_test(struct ExternError*, const char* input);

// Fetch one network for general display purposes
const char * get_network(struct ExternError*, const char* genesis_hash, const char* dbname);

// Fetch list of available networks for network selector screen
const char * get_all_networks_for_network_selector(struct ExternError*, const char* dbname);

// Filter identities derived for given seed and network
const char * get_relevant_identities(struct ExternError*, const char* seed_name, const char* genesis_hash, const char* dbname);

// Show all keys
const char * get_all_identities(struct ExternError*, const char* dbname);

// Function to create new seed
const char * try_create_seed(struct ExternError*, const char* seed_name, const char* crypto, const char* seed_phrase, int seed_length, const char* dbname);

// Suggest next numbered path
const char * suggest_n_plus_one(struct ExternError*, const char* path, const char* seed_name, const char* network_id_string, const char* dbname);

// Check validity of proposed path and find password
int8_t check_path(struct ExternError*, const char* path);

// Function to create new address
void try_create_identity(struct ExternError*, const char* id_name, const char* seed_name, const char* seed_phrase, const char* crypto, const char* path, const char* network, int8_t has_password, const char* dbname);

// Suggest convenient name for new identity created
const char * suggest_name(struct ExternError*, const char* path);

// Delete identity (really removes network from allowed networks list and trims identities with no networks)
void delete_identity(struct ExternError*, const char* pub_key, const char* network, const char* dbname);

// Get network specs for settings screen
const char * get_network_specs(struct ExternError*, const char* network_name, const char* dbname);

// Removes network from db
void remove_network(struct ExternError*, const char* network_name, const char* dbname);

// Removes metadata record from db
void remove_metadata(struct ExternError*, const char* network_name, int network_version, const char* dbname);

// Cleans identities after seed removal - deletes identities bound to given seed
void remove_seed(struct ExternError*, const char* seed_name, const char* dbname);
