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
