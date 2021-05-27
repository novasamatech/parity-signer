// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import colors from 'styles/colors';
import {
	EthereumNetworkDefaultConstants,
	EthereumNetworkParams,
	NetworkParams,
	NetworkProtocol,
	SubstrateNetworkDefaultConstant,
	SubstrateNetworkParams,
	UnknownNetworkParams
} from 'types/networkTypes';

export const unknownNetworkPathId = '';

export const NetworkProtocols: Record<string, NetworkProtocol> = Object.freeze({
	ETHEREUM: 'ethereum',
	SUBSTRATE: 'substrate',
	UNKNOWN: 'unknown'
});

// accounts for which the network couldn't be found (failed migration, removed network)
export const UnknownNetworkKeys: Record<string, string> = Object.freeze({
	UNKNOWN: 'unknown'
});

// ethereumChainId is used as Network key for Ethereum networks
/* eslint-disable sort-keys */
export const EthereumNetworkKeys: Record<string, string> = Object.freeze({
	FRONTIER: '1',
	ROPSTEN: '3',
	RINKEBY: '4',
	GOERLI: '5',
	KOVAN: '42',
	CLASSIC: '61'
});

/* eslint-enable sort-keys */

// genesisHash is used as Network key for Substrate networks
export const SubstrateNetworkKeys: Record<string, string> = Object.freeze({
	CENTRIFUGE:
		'0x67dddf2673b69e5f875f6f25277495834398eafd67f492e09f3f3345e003d1b5', // https://portal.chain.centrifuge.io/#/explorer/query/0
	CENTRIFUGE_AMBER:
		'0x092af6e7d25178ebab1677d15f66e37b30392b44ef442f728a53dd1bf48ec110', // https://portal.chain.centrifuge.io/#/explorer/query/0
	EDGEWARE:
		'0x742a2ca70c2fda6cee4f8df98d64c4c670a052d9568058982dad9d5a7a135c5b', // https://polkascan.io/pre/edgeware/block/0
	KULUPU: '0xf7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba',
	KUSAMA: '0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe', // https://polkascan.io/pre/kusama-cc3/block/0
	KUSAMA_DEV:
		'0x5e9679182f658e148f33d3f760f11179977398bb3da8d1f0bf7b267fe6b3ebb0',
	POLKADOT:
		'0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3',
	ROCOCO: '0x78ae7dc7e64637e01fa6a6b6e4fa252c486f62af7aa71c471ad17f015bd375ce',
	SUBSTRATE_DEV:
		'0x0d667fd278ec412cd9fccdb066f09ed5b4cfd9c9afa9eb747213acb02b1e70bc', // substrate --dev commit ac6a2a783f0e1f4a814cf2add40275730cd41be1 hosted on wss://dev-node.substrate.dev .
	WESTEND: '0xe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e'
});

export const unknownNetworkParams: UnknownNetworkParams = {
	color: colors.signal.error,
	order: 99,
	pathId: unknownNetworkPathId,
	prefix: 2,
	protocol: NetworkProtocols.UNKNOWN,
	secondaryColor: colors.background.card,
	title: 'Unknown network'
};

export const dummySubstrateNetworkParams: SubstrateNetworkParams = {
	...unknownNetworkParams,
	decimals: 12,
	deleted: false,
	genesisHash: UnknownNetworkKeys.UNKNOWN,
	logo: require('res/img/logos/Substrate_Dev.png'),
	metadata: {
		hash: 'd266fcfdf60c09b2065d6a36751fcde65600df34d372cf0fc1e9d86c1d9edb45',
		specName: 'kusama',
		specVersion: 9010
	},
	protocol: NetworkProtocols.SUBSTRATE,
	unit: 'UNIT'
};

const unknownNetworkBase: Record<string, UnknownNetworkParams> = {
	[UnknownNetworkKeys.UNKNOWN]: unknownNetworkParams
};

const substrateNetworkBase: Record<string, SubstrateNetworkDefaultConstant> = {
	[SubstrateNetworkKeys.CENTRIFUGE]: {
		color: '#FCC367',
		decimals: 18,
		genesisHash: SubstrateNetworkKeys.CENTRIFUGE,
		logo: require('res/img/logos/Centrifuge.png'),
		metadata: {
			hash: '5159df481cf4378ad99f15cac521861bf32458e1b4fad19f03bb3dd8923f899d',
			specName: 'centrifuge',
			specVersion: 0
		},
		order: 7,
		pathId: 'centrifuge',
		prefix: 36,
		title: 'Centrifuge Mainnet',
		unit: 'RAD'
	},
	[SubstrateNetworkKeys.CENTRIFUGE_AMBER]: {
		color: '#7C6136',
		decimals: 18,
		genesisHash: SubstrateNetworkKeys.CENTRIFUGE_AMBER,
		logo: require('res/img/logos/Centrifuge.png'),
		metadata: {
			hash: '5159df481cf4378ad99f15cac521861bf32458e1b4fad19f03bb3dd8923f899d',
			specName: 'centrifugeAmber',
			specVersion: 0
		},
		order: 8,
		pathId: 'centrifuge_amber',
		prefix: 42,
		title: 'Centrifuge Testnet Amber',
		unit: 'ARAD'
	},
	[SubstrateNetworkKeys.EDGEWARE]: {
		color: '#0B95E0',
		decimals: 18,
		genesisHash: SubstrateNetworkKeys.EDGEWARE,
		logo: require('res/img/logos/Edgeware.png'),
		metadata: {
			hash: '9f7f918419a8937498e718e30164a69f6ac669bed3779f5a1c02589d7a831c41',
			specName: 'edgeware',
			specVersion: 0
		},
		order: 6,
		pathId: 'edgeware',
		prefix: 7,
		title: 'Edgeware',
		unit: 'EDG'
	},
	[SubstrateNetworkKeys.KULUPU]: {
		color: '#003366',
		decimals: 18,
		genesisHash: SubstrateNetworkKeys.KULUPU,
		metadata: {
			hash: '124a81a831e846343e15380846988ce0f744fbcccba90979c501477a2ff58b76',
			specName: 'kulupu',
			specVersion: 17
		},
		order: 5,
		pathId: 'kulupu',
		prefix: 16,
		title: 'Kulupu',
		unit: 'KULU'
	},
	[SubstrateNetworkKeys.KUSAMA]: {
		color: '#000',
		decimals: 12,
		genesisHash: SubstrateNetworkKeys.KUSAMA,
		logo: require('res/img/logos/Kusama.png'),
		metadata: {
			hash: 'd266fcfdf60c09b2065d6a36751fcde65600df34d372cf0fc1e9d86c1d9edb45',
			specName: 'kusama',
			specVersion: 9010
		},
		order: 2,
		pathId: 'kusama',
		prefix: 2,
		title: 'Kusama',
		unit: 'KSM'
	},
	[SubstrateNetworkKeys.KUSAMA_DEV]: {
		color: '#000',
		decimals: 12,
		genesisHash: SubstrateNetworkKeys.KUSAMA_DEV,
		metadata: {
			hash: 'd266fcfdf60c09b2065d6a36751fcde65600df34d372cf0fc1e9d86c1d9edb45',
			specName: 'kusama',
			specVersion: 9010
		},
		order: 99,
		pathId: 'kusama_dev',
		prefix: 2,
		title: 'Kusama Development',
		unit: 'KSM'
	},
	[SubstrateNetworkKeys.POLKADOT]: {
		color: '#E6027A',
		decimals: 10,
		genesisHash: SubstrateNetworkKeys.POLKADOT,
		logo: require('res/img/logos/Polkadot.png'),
		metadata: {
			hash: '93b9065e4a6b8327ca1ce90e9ac3d7d967a660dcc5cda408e2595aa3e5c1ab46',
			specName: 'polkadot',
			specVersion: 30
		},
		order: 1,
		pathId: 'polkadot',
		prefix: 0,
		title: 'Polkadot',
		unit: 'DOT'
	},
	[SubstrateNetworkKeys.ROCOCO]: {
		color: '#6f36dc',
		decimals: 12,
		genesisHash: SubstrateNetworkKeys.ROCOCO,
		logo: require('res/img/logos/Rococo.png'),
		metadata: {
			hash: 'a91f58c2a11b8bdc671f63f2cca291b417604d1d96df7cc8adbf30b03ed6b698',
			specName: 'rococo',
			specVersion: 9002
		},
		order: 4,
		pathId: 'rococo',
		prefix: 0,
		title: 'Rococo',
		unit: 'ROC'
	},
	[SubstrateNetworkKeys.WESTEND]: {
		color: '#660D35',
		decimals: 12,
		genesisHash: SubstrateNetworkKeys.WESTEND,
		logo: require('res/img/logos/Polkadot.png'),
		metadata: {
			hash: 'dd71fa0db7b0f678fa819fc01146b5ea25ed3ab91c5ce8477ed7695b1ad66626',
			specName: 'westend',
			specVersion: 9030
		},
		order: 3,
		pathId: 'westend',
		prefix: 42,
		title: 'Westend',
		unit: 'WND'
	}
};

const ethereumNetworkBase: Record<string, EthereumNetworkDefaultConstants> = {
	[EthereumNetworkKeys.FRONTIER]: {
		color: '#8B94B3',
		ethereumChainId: EthereumNetworkKeys.FRONTIER,
		order: 101,
		secondaryColor: colors.background.card,
		title: 'Ethereum'
	},
	[EthereumNetworkKeys.CLASSIC]: {
		color: '#1a4d33',
		ethereumChainId: EthereumNetworkKeys.CLASSIC,
		logo: require('res/img/logos/Ethereum_Classic.png'),
		order: 102,
		secondaryColor: colors.background.card,
		title: 'Ethereum Classic'
	},
	[EthereumNetworkKeys.ROPSTEN]: {
		ethereumChainId: EthereumNetworkKeys.ROPSTEN,
		order: 104,
		title: 'Ropsten Testnet'
	},
	[EthereumNetworkKeys.GOERLI]: {
		ethereumChainId: EthereumNetworkKeys.GOERLI,
		order: 105,
		title: 'Görli Testnet'
	},
	[EthereumNetworkKeys.KOVAN]: {
		ethereumChainId: EthereumNetworkKeys.KOVAN,
		order: 103,
		title: 'Kovan Testnet'
	}
};

const ethereumDefaultValues = {
	color: '#434875',
	logo: require('res/img/logos/Ethereum.png'),
	protocol: NetworkProtocols.ETHEREUM,
	secondaryColor: colors.background.card
};

const substrateDefaultValues = {
	color: '#4C4646',
	deleted: false,
	logo: require('res/img/logos/Substrate_Dev.png'),
	protocol: NetworkProtocols.SUBSTRATE,
	secondaryColor: colors.background.card
};

function setEthereumNetworkDefault(): Record<string, EthereumNetworkParams> {
	return Object.keys(ethereumNetworkBase).reduce((acc, networkKey) => {
		return {
			...acc,
			[networkKey]: {
				...ethereumDefaultValues,
				...ethereumNetworkBase[networkKey]
			}
		};
	}, {});
}

function setSubstrateNetworkDefault(): Record<string, SubstrateNetworkParams> {
	return Object.keys(substrateNetworkBase).reduce((acc, networkKey) => {
		return {
			...acc,
			[networkKey]: {
				...substrateDefaultValues,
				...substrateNetworkBase[networkKey]
			}
		};
	}, {});
}

export const ETHEREUM_NETWORK_LIST: Record<
	string,
	EthereumNetworkParams
> = Object.freeze(setEthereumNetworkDefault());
export const SUBSTRATE_NETWORK_LIST: Record<
	string,
	SubstrateNetworkParams
> = Object.freeze(setSubstrateNetworkDefault());
export const UNKNOWN_NETWORK: Record<
	string,
	UnknownNetworkParams
> = Object.freeze(unknownNetworkBase);

export const NETWORK_LIST: Record<string, NetworkParams> = Object.freeze(
	Object.assign(
		{},
		SUBSTRATE_NETWORK_LIST,
		ETHEREUM_NETWORK_LIST,
		UNKNOWN_NETWORK
	)
);

export const defaultNetworkKey = SubstrateNetworkKeys.KUSAMA;
