// Copyright 2015-2017 Parity Technologies (UK) Ltd.
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

// string
struct rust_string;

// string_ptr
struct rust_string_ptr {
	const uint8_t* ptr;
	size_t len;
};

// return ptr to rust_str
struct rust_string_ptr* rust_string_ptr(const struct rust_string* s);

// removes rust string
void rust_string_destroy(struct rust_string* s);

// removes string pointer
void rust_string_ptr_destroy(struct rust_string_ptr* s);

// ethkey ffi

// keypair pointer
struct keypair_ptr;

// removes keypair pointer
void ethkey_keypair_destroy(struct keypair_ptr* keypair);

// creates new brainwallet keypair from seed
struct keypair_ptr* ethkey_keypair_brainwallet(const struct rust_string_ptr* seed);

// returns keypair secret
struct rust_string* ethkey_keypair_secret(const struct keypair_ptr* keypair);

// return keypair address
struct rust_string* ethkey_keypair_address(const struct keypair_ptr* keypair);

// returns message signed with keypair
struct rust_string* ethkey_keypair_sign(const struct keypair_ptr* keypair, const struct rust_string_ptr* message);

// rlp ffi

// returns rlp item at given position
struct rust_string* rlp_item(const struct rust_string_ptr* rlp, const unsigned position, unsigned* error);

// sha3 ffi

struct rust_string* keccak256(const struct rust_string_ptr* data);

struct rust_string* eth_sign(const struct rust_string_ptr* data);

// blockies ffi

struct rust_string* blockies_icon(const struct rust_string_ptr* blockies_seed);

// random phrase ffi

struct rust_string* random_phrase(const uint32_t words);

// data encryption ffi

struct rust_string* encrypt_data(const struct rust_string_ptr* data, const struct rust_string_ptr* password);

struct rust_string* decrypt_data(const struct rust_string_ptr* encrypted_data, const struct rust_string_ptr* password, unsigned* error);
