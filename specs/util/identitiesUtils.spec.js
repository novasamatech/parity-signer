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

const raw = [
	{
		address: 'addressDefault',
		expectName: 'default',
		isKusamaPath: true,
		name: '',
		path: '//kusama//default'
	},
	{
		address: 'address1',
		expectName: 'funding account1',
		isKusamaPath: true,
		name: 'funding account1',
		path: '//kusama//funding/1'
	},
	{
		address: 'address3',
		expectName: 'softKey1',
		isKusamaPath: true,
		name: '',
		path: '//kusama/softKey1'
	},
	{
		address: 'address2',
		expectName: 'funding2',
		isKusamaPath: true,
		name: '',
		path: '//kusama//funding/2'
	},
	{
		address: 'address4',
		expectName: 'staking1',
		isKusamaPath: true,
		name: '',
		path: '//kusama//staking/1'
	},
	{
		address: 'address5',
		expectName: 'default',
		name: '',
		path: '//polkadot//default'
	},
	{
		address: 'address6',
		expectName: 'No name',
		name: '',
		path: '1'
	},
	{
		address: 'addressKusamaRoot',
		expectName: 'kusama',
		isKusamaPath: true,
		name: '',
		path: '//kusama'
	},
	{
		address: 'addressRoot',
		expectName: '',
		name: '',
		path: ''
	},
	{
		address: 'addressCustom',
		expectName: 'CustomName',
		name: 'CustomName',
		path: '//custom'
	},
	{
		address: 'addressKusamaSoft',
		expectName: 'kusama',
		name: '',
		path: '/kusama'
	},
	{
		address: 'softAddress',
		expectName: '1',
		name: '',
		path: '/kusama/1'
	}
];
const expectNames = raw.map(v => v.expectName);
const paths = raw.map(v => v.path);
const kusamaPaths = raw.filter(v => v.isKusamaPath).map(v => v.path);
const metaMap = raw.reduce((acc, v) => {
	const meta = {
		address: v.address,
		createdAt: 1573142786972,
		name: v.name,
		updatedAt: 1573142786972
	};
	acc.set(v.path, meta);
	return acc;
}, new Map());
const addressesMap = raw.reduce((acc, v) => {
	acc.set(v.address, v.path);
	return acc;
}, new Map());

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

	it('regroup the kusama paths', () => {
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

	it('regroup the unknown paths', () => {
		const unKnownPaths = [
			'//polkadot//default',
			'',
			'//custom',
			'/kusama',
			'/kusama/1'
		];
		const groupResult = groupPaths(unKnownPaths);
		expect(groupResult).toEqual([
			{
				paths: ['//polkadot//default'],
				title: '//default'
			},
			{
				paths: ['//custom'],
				title: 'custom'
			},
			{
				paths: ['/kusama'],
				title: 'kusama'
			},
			{
				paths: ['/kusama/1'],
				title: '/1'
			}
		]);
	});

	it('get the path name', () => {
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
