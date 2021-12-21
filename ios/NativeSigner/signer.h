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

//Main action that should replace all logic
const char * act(struct ExternError*, const char* action, const char* details, const char* seed_phrase);

//Init navigation
void init_navigation(struct ExternError*, const char* dbname, const char* seed_names);

// Show QR with key name, public key and network ID
const char * export_pubkey(struct ExternError*, const char* address, const char* network, const char* dbname);

// qr frame count estimator
int get_packets_total(struct ExternError*, const char* data, int8_t cleaned);

// qr fountain decoder
const char * try_decode_qr_sequence(struct ExternError*, const char* data, int8_t cleaned);

// Generate identicon from base58 address
const char * base58_identicon(struct ExternError*, const char* base58, int size);

// Generate identicon from public key
const char * identicon(struct ExternError*, const char* key, int size);

// Fetch one network for general display purposes
const char * get_network(struct ExternError*, const char* genesis_hash, const char* dbname);

// Fetch list of available networks for network selector screen
const char * get_all_networks_for_network_selector(struct ExternError*, const char* dbname);

// Filter identities derived for given seed and network
const char * get_relevant_identities(struct ExternError*, const char* seed_name, const char* genesis_hash, const char* dbname);

// Show all keys
const char * get_all_identities(struct ExternError*, const char* dbname);

// Guess next word for seed
const char * guess_word(struct ExternError*, const char* part);

// Function to create new seed
const char * try_create_seed(struct ExternError*, const char* seed_name, int seed_length, const char* dbname);

// Function to create new seed
const char * try_restore_seed(struct ExternError*, const char* seed_name, const char* seed_phrase, const char* dbname);

// Suggest next numbered path
const char * suggest_n_plus_one(struct ExternError*, const char* path, const char* seed_name, const char* network_id_string, const char* dbname);

// Check validity of proposed path and find password
int8_t check_path(struct ExternError*, const char* path);

// Function to create new address
void try_create_identity(struct ExternError*, const char* seed_name, const char* seed_phrase, const char* crypto, const char* path, const char* network, int8_t has_password, const char* dbname);

// Delete identity (really removes network from allowed networks list and trims identities with no networks)
void delete_identity(struct ExternError*, const char* pub_key, const char* network, const char* dbname);

// Get network specs for settings screen
const char * get_network_specs(struct ExternError*, const char* network_name, const char* dbname);

// History access operations
// Fetch history for display
const char * print_history(struct ExternError*, const char* dbname);

// Clear history (marks history with clearing event time)
void clear_history(struct ExternError*, const char* dbname);

// Init history - should be called after db copy from resources, marks signer factory reset event, installs general verifier
void init_history_with_cert(struct ExternError*, const char* dbname);

// Init history - should be called after db copy from resources, marks signer factory reset event. Use for jailbreak.
void init_history_no_cert(struct ExternError*, const char* dbname);

// Record going online event
void device_was_online(struct ExternError*, const char* dbname);

// Check if warnings are present
int8_t get_warnings(struct ExternError*, const char* dbname);

// Disarm log alert
void acknowledge_warnings(struct ExternError*, const char* dbname);

// Add custom record to history
void history_entry_user(struct ExternError*, const char* enrty, const char* dbname);

// Generic function to record system events
void history_entry_system(struct ExternError*, const char* entry, const char* dbname);

// Call on seed backup
void seed_name_was_shown(struct ExternError*, const char* seed_name, const char* dbname);

const char * get_general_certificate(struct ExternError*, const char* dbname);


