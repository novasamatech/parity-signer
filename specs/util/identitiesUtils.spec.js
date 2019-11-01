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

import {
	deserializeIdentities,
	serializeIdentities
} from '../../src/util/identitiesUtils';

const address1 = 'address1',
	address2 = 'address2',
	path1 = '//kusama//funding/1',
	path2 = '//kusama//funding/2',
	pathMeta1 = {
		address: address1,
		createdAt: 1571068850409,
		name: 'funding account1',
		updatedAt: 1571078850509
	},
	pathMeta2 = {
		address: address2,
		createdAt: 1571068850409,
		name: 'funding account2',
		updatedAt: 1571078850509
	};
const addressMap = new Map([[address1, path1], [address2, path2]]);
const metaMap = new Map([[path1, pathMeta1], [path2, pathMeta2]]);
const testIdentities = [
	{
		addresses: addressMap,
		derivationPassword: '',
		encryptedSeedPhrase: 'yyyy',
		meta: metaMap,
		name: 'identity1'
	},
	{
		addresses: addressMap,
		derivationPassword: '',
		encryptedSeedPhrase: 'xxxx',
		meta: metaMap,
		name: 'identity2'
	}
];

describe('IdentitiesUtils', () => {
	it('works with serialize and deserialize', () => {
		const serializedJson = serializeIdentities(testIdentities);
		const originItem = deserializeIdentities(serializedJson);
		expect(originItem).toEqual(testIdentities);
	});
});
