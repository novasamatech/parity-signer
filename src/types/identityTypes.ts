// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import { AccountsContextState } from 'stores/AccountsContext';

export type AccountMeta = {
	address: string;
	createdAt: number;
	name: string;
	updatedAt: number;
	networkPathId?: string;
};

export interface FoundAccount extends AccountMeta {
	accountId: string;
	encryptedSeed: string;
	validBip39Seed: true;
	networkKey: string;
	path: string;
}

export type Identity = {
	// encrypted seed include seedPhrase and password
	encryptedSeed: string;
	meta: Map<string, AccountMeta>;
	addresses: Map<string, string>;
	name: string;
};

export type SerializedIdentity = {
	encryptedSeed: string;
	meta: Array<[string, AccountMeta]>;
	addresses: Array<[string, string]>;
	name: string;
};

export type AccountsStoreState = {
	identities: Identity[];
	currentIdentity: Identity | null;
	loaded: boolean;
	newIdentity: Identity;
	selectedKey: string;
};

type LensSet<T, R> = Omit<T, keyof R> & R;
export type AccountsStoreStateWithIdentity = LensSet<
	AccountsContextState,
	{ state: LensSet<AccountsStoreState, { currentIdentity: Identity }> }
>;
