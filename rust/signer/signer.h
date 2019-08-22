// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

// return keypair address, automatically picking BIP39 or parity phrases
struct rust_string* ethkey_brainwallet_address(unsigned* error, const struct rust_string_ptr* seed);

// return keypair address from BIP39 phrase
struct rust_string* ethkey_brainwallet_bip39_address(unsigned* error, const struct rust_string_ptr* seed);

// returns message signed with keypair
struct rust_string* ethkey_brainwallet_sign(unsigned* error, const struct rust_string_ptr* seed, const struct rust_string_ptr* message);

// returns rlp item at given position
struct rust_string* rlp_item(unsigned* error, const struct rust_string_ptr* rlp, const unsigned position);

struct rust_string* keccak256(unsigned* error, const struct rust_string_ptr* data);

struct rust_string* blake(unsigned* error, const struct rust_string_ptr* data);

struct rust_string* eth_sign(unsigned* error, const struct rust_string_ptr* data);

struct rust_string* blockies_icon(unsigned* error, const struct rust_string_ptr* blockies_seed);

struct rust_string* random_phrase(unsigned* error);

struct rust_string* encrypt_data(unsigned* error, const struct rust_string_ptr* data, const struct rust_string_ptr* password);

struct rust_string* decrypt_data(unsigned* error, const struct rust_string_ptr* encrypted_data, const struct rust_string_ptr* password);

// qr code generator for utf-8 strings
struct rust_string* qrcode(unsigned* error, const struct rust_string_ptr* data);

// qr code generator for hex-encoded binary
struct rust_string* qrcode_hex(unsigned* error, const struct rust_string_ptr* data);

// ss58 address (including prefix) for sr25519 key generated out of BIP39 phrase
struct rust_string* substrate_brainwallet_address(unsigned* error, const struct rust_string_ptr* seed, const unsigned prefix);

struct rust_string* substrate_brainwallet_sign(unsigned* error, const struct rust_string_ptr* seed, const struct rust_string_ptr* data);
