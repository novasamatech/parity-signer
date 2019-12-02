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

// @flow

export type Account = {
	address: string,
	biometricEnabled: boolean,
	createdAt: number,
	derivationPassword: string,
	derivationPath: string, // doesn't contain the ///password
	encryptedSeed: string,
	name: string,
	networkKey: string,
	pinKey: string,
	seed: string, //this is the SURI (seedPhrase + /soft//hard///password derivation)
	seedPhrase: string, //contains only the BIP39 words, no derivation path
	updatedAt: number,
	validBip39Seed: boolean
};

type AccountMeta = {
	address: string,
	createdAt: number,
	name: ?string,
	updatedAt: number
};

export type Identity = {
	accountIds: Map<string, string>,
	biometricEnabled: boolean,
	derivationPassword: string,
	// encrypted seed include seedPhrase and password
	encryptedSeed: string,
	meta: Map<string, AccountMeta>,
	name: string,
	pinKey: string
};

export type AccountsStoreState = {
	identities: [Identity],
	accounts: Map<string, Account>,
	newAccount: Account,
	newIdentity: ?Identity,
	selectedKey: string
};
