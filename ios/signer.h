#pragma once

#include <stdint.h>

// string
struct string_ptr {
	const uint8_t* ptr;
	size_t len;
};

// removes string pointer
void ethstore_string_destroy(struct string_ptr* s);

// keypair pointer
struct keypair_ptr;

// creates new brainwallet keypair from seed
struct keypair_ptr* ethstore_keypair_brainwallet(const struct string_ptr* seed);

// removes keypair pointer
void ethstore_keypair_destroy(struct keypair_ptr* keypair);

struct string_ptr* ethstore_keypair_address(const struct keypair_ptr* keypair);

// test
struct string_ptr* tmp_string();
