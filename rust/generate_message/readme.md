
# Crate `generate_message`

## Overview

This crate is intended to support the [Signer](https://github.com/paritytech/parity-signer) from the active (non air-gapped) side.

This crate is mainly used to:

 - fetch network data through rpc calls
 - prepare Signer update and derivation import payloads
 - generate Signer update QR codes, either signed or unsigned, and derivations import QR codes, to be scanned into Signer
 - maintain the hot database on the network-connected device, to store and manage the data that went into update QR codes
 - maintain Signer default network metadata set in `defaults` crate and prepare the cold database for the Signer release

## Crate documentation

Please refer to the crate documentation [here](https://paritytech.github.io/parity-signer/rustdocs/generate_message/index.html).

Documents could be built locally:

`$ cargo doc --open`

## Current usage

Program is run by

`$ cargo run COMMAND [KEY(s)]`

Available commands, keys and arguments are listed in crate documentation [here](https://paritytech.github.io/parity-signer/rustdocs/generate_message/index.html)

## Usage tutorial

Please refer to the manual [here](https://paritytech.github.io/parity-signer/user-docs/tutorials/Add-New-Network.html).
