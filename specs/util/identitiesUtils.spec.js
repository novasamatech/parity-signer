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

'use strict';

import {
	deserializeIdentities,
	getExistedNetworkKeys,
	getPathName,
	groupPaths,
	serializeIdentities
} from '../../src/util/identitiesUtils';
import {
	EthereumNetworkKeys,
	SubstrateNetworkKeys,
	UnknownNetworkKeys
} from '../../src/constants';

const addressFunding1 = 'address1',
	addressFunding2 = 'address2',
	addressSoft = 'address3',
	addressPolkadot = 'address5',
	addressStaking = 'address4',
	addressEthereum = 'address6',
	addressDefault = 'addressDefault',
	paths = [
		'//kusama//default',
		'//kusama//funding/1',
		'//kusama/softKey1',
		'//kusama//funding/2',
		'//kusama//staking/1',
		'//polkadot//default',
		'1'
	],
	metaDefault = {
		address: addressDefault,
		createdAt: 1571068850409,
		name: '',
		updatedAt: 1571078850509
	},
	metaFunding1 = {
		address: addressFunding1,
		createdAt: 1571068850409,
		name: 'funding account1',
		updatedAt: 1571078850509
	},
	metaFunding2 = {
		address: addressFunding2,
		createdAt: 1571068850409,
		name: '',
		updatedAt: 1571078850509
	},
	metaStaking = {
		address: addressStaking,
		createdAt: 1571068850409,
		name: '',
		updatedAt: 1571078850509
	},
	metaPolkadot = {
		address: addressPolkadot,
		createdAt: 1573142786972,
		name: 'PolkadotFirst',
		updatedAt: 1573142786972
	},
	metaEthereum = {
		address: addressEthereum,
		createdAt: 1573142786972,
		name: 'Eth account',
		updatedAt: 1573142786972
	},
	metaSoftKey = {
		address: addressSoft,
		createdAt: 1573142786972,
		name: '',
		updatedAt: 1573142786972
	};
const addressesMap = new Map([
	[addressDefault, paths[0]],
	[addressFunding1, paths[1]],
	[addressSoft, paths[2]],
	[addressFunding2, paths[3]],
	[addressStaking, paths[4]],
	[addressPolkadot, paths[5]],
	[addressEthereum, paths[6]]
]);
const metaMap = new Map([
	[paths[0], metaDefault],
	[paths[1], metaFunding1],
	[paths[2], metaSoftKey],
	[paths[3], metaStaking],
	[paths[4], metaFunding2],
	[paths[5], metaPolkadot],
	[paths[6], metaEthereum]
]);
const testIdentities = [
	{
		addresses: addressesMap,
		derivationPassword: '',
		encryptedSeed: 'yyyy',
		meta: metaMap,
		name: 'identity1'
	},
	{
		addresses: addressesMap,
		derivationPassword: '',
		encryptedSeed: 'xxxx',
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
		const kusamaPaths = paths.slice(0, 5);
		const groupResult = groupPaths(kusamaPaths);
		expect(groupResult).toEqual([
			{
				paths: [paths[0]],
				title: '//default'
			},
			{
				paths: [paths[2]],
				title: '/softKey1'
			},
			{
				paths: [paths[4]],
				title: '//staking'
			},
			{
				paths: [paths[1], paths[3]],
				title: '//funding'
			}
		]);
	});

	it('get the path name', () => {
		const expectNames = [
			'default',
			'funding account1',
			'softKey1',
			'funding2',
			'staking1',
			'PolkadotFirst',
			'Eth account'
		];
		paths.forEach((path, index) => {
			const name = getPathName(path, testIdentities[0]);
			expect(name).toEqual(expectNames[index]);
		});
	});

	it('get the correspond networkKeys', () => {
		const networkKeys = getExistedNetworkKeys(testIdentities[0]);
		expect(networkKeys).toEqual([
			EthereumNetworkKeys.FRONTIER,
			SubstrateNetworkKeys.KUSAMA,
			UnknownNetworkKeys.UNKNOWN
		]);
	});
});
