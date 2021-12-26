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

//Call this after each change to seeds
void update_seed_names(struct ExternError*, const char* seed_names);

// qr frame count estimator
int get_packets_total(struct ExternError*, const char* data, int8_t cleaned);

// qr fountain decoder
const char * try_decode_qr_sequence(struct ExternError*, const char* data, int8_t cleaned);

// Guess next word for seed
const char * guess_word(struct ExternError*, const char* part);

// Check validity of proposed path and find password
int8_t check_path(struct ExternError*, const char* path);

// History access operations

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

