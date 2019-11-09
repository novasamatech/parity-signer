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
	getAvailableNetworkKeys,
	getPathName,
	groupPaths,
	serializeIdentities
} from '../../src/util/identitiesUtils';
import {
	EthereumNetworkKeys,
	SubstrateNetworkKeys,
	UnknownNetworkKeys
} from '../../src/constants';

const address1 = 'address1',
	address2 = 'address2',
	addressPolkadot = 'address5',
	addressEthereum = 'address6',
	paths = [
		'//kusama_CC2//default',
		'//kusama_CC2//funding/1',
		'//kusama_CC2//funding/2',
		'//kusama_CC2//stacking/1',
		'//polkadot//default',
		'1'
	],
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
	},
	pathMetaPolkadot = {
		address: addressPolkadot,
		createdAt: 1573142786972,
		name: 'PolkadotFirst',
		updatedAt: 1573142786972
	},
	pathMetaEthereum = {
		address: addressEthereum,
		createdAt: 1573142786972,
		name: 'Eth account',
		updatedAt: 1573142786972
	};
const addressMap = new Map([
	[address1, paths[1]],
	[address2, paths[2]],
	[addressPolkadot, paths[4]],
	[addressEthereum, paths[5]]
]);
const metaMap = new Map([
	[paths[1], pathMeta1],
	[paths[2], pathMeta2],
	[paths[4], pathMetaPolkadot],
	[paths[5], pathMetaEthereum]
]);
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

	it('regroup the paths', () => {
		const kusamaPaths = paths.slice(0, 4);
		const groupResult = groupPaths(kusamaPaths);
		expect(groupResult).toEqual([
			{
				paths: [paths[0]],
				title: 'default'
			},
			{
				paths: [paths[3]],
				title: 'stacking'
			},
			{
				paths: [paths[1], paths[2]],
				title: 'funding'
			}
		]);
	});

	it('get the path name', () => {
		const gotDefaultPathName = getPathName(paths[0], testIdentities[0]);
		expect(gotDefaultPathName).toEqual('default');
		const gotIdentity1PathName = getPathName(paths[1], testIdentities[0]);
		expect(gotIdentity1PathName).toEqual('funding account1');
		const gotStacking1PathName = getPathName(paths[3], testIdentities[0]);
		expect(gotStacking1PathName).toEqual('stacking1');
	});

	it('get the correspond networkKeys', () => {
		const networkKeys = getAvailableNetworkKeys(testIdentities[0]);
		expect(networkKeys).toEqual([
			EthereumNetworkKeys.FRONTIER,
			SubstrateNetworkKeys.KUSAMA,
			UnknownNetworkKeys.UNKNOWN
		]);
	});
});
