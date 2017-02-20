#pragma once

#include <stdint.h>

// string
struct string_ptr {
	const uint8_t* ptr;
	size_t len;
};

// removes string pointer
void ethkey_string_destroy(struct string_ptr* s);

// keypair pointer
struct keypair_ptr;

// creates new brainwallet keypair from seed
struct keypair_ptr* ethkey_keypair_brainwallet(const struct string_ptr* seed);

// removes keypair pointer
void ethkey_keypair_destroy(struct keypair_ptr* keypair);

// returns keypair secret
struct string_ptr* ethkey_keypair_secret(const struct keypair_ptr* keypair);

// return keypair address
struct string_ptr* ethkey_keypair_address(const struct keypair_ptr* keypair);
