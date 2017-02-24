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
struct rust_string* rlp_item(const struct rust_string_ptr* rlp, const unsigned position);
