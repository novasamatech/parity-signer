// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import { colors } from 'styles';
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
	GOERLI: '5'
});

/* eslint-enable sort-keys */

// genesisHash is used as Network key for Substrate networks
export const SubstrateNetworkKeys: Record<string, string> = Object.freeze({
	// genesis hashes can be found at e.g. https://edgeware.subscan.io/block/0
	EDGEWARE:
		'0x742a2ca70c2fda6cee4f8df98d64c4c670a052d9568058982dad9d5a7a135c5b',
	KULUPU: '0xf7a99d3cb92853d00d5275c971c132c074636256583fee53b3bbe60d7b8769ba',
	KUSAMA: '0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe',
	POLKADOT:
		'0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3',
	ROCOCO: '0x78ae7dc7e64637e01fa6a6b6e4fa252c486f62af7aa71c471ad17f015bd375ce',
	WESTEND: '0xe143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e'
});

export const unknownNetworkParams: UnknownNetworkParams = {
	color: colors.text.error,
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
	protocol: NetworkProtocols.SUBSTRATE,
	unit: 'UNIT',
	url: ''
};

const unknownNetworkBase: Record<string, UnknownNetworkParams> = {
	[UnknownNetworkKeys.UNKNOWN]: unknownNetworkParams
};

const substrateNetworkBase: Record<string, SubstrateNetworkDefaultConstant> = {
	[SubstrateNetworkKeys.EDGEWARE]: {
		color: '#0B95E0',
		decimals: 18,
		genesisHash: SubstrateNetworkKeys.EDGEWARE,
		isTestnet: false,
		logo: require('res/img/logos/Edgeware.png'),
		order: 6,
		pathId: 'edgeware',
		prefix: 7,
		title: 'Edgeware',
		unit: 'EDG',
		url: 'wss://mainnet1.edgewa.re'
	},
	[SubstrateNetworkKeys.KULUPU]: {
		color: '#003366',
		decimals: 18,
		genesisHash: SubstrateNetworkKeys.KULUPU,
		isTestnet: false,
		logo: require('res/img/logos/Kulupu.png'),
		order: 5,
		pathId: 'kulupu',
		prefix: 16,
		title: 'Kulupu',
		unit: 'KLP',
		url: 'wss://rpc.kulupu.corepaper.org/ws'
	},
	[SubstrateNetworkKeys.KUSAMA]: {
		color: '#000',
		decimals: 12,
		genesisHash: SubstrateNetworkKeys.KUSAMA,
		isTestnet: false,
		logo: require('res/img/logos/Kusama.png'),
		order: 2,
		pathId: 'kusama',
		prefix: 2,
		title: 'Kusama',
		unit: 'KSM',
		url: 'wss://kusama-rpc.polkadot.io'
	},
	[SubstrateNetworkKeys.POLKADOT]: {
		color: '#E6027A',
		decimals: 10,
		genesisHash: SubstrateNetworkKeys.POLKADOT,
		isTestnet: false,
		logo: require('res/img/logos/Polkadot.png'),
		order: 1,
		pathId: 'polkadot',
		prefix: 0,
		title: 'Polkadot',
		unit: 'DOT',
		url: 'wss://rpc.polkadot.io'
	},
	[SubstrateNetworkKeys.ROCOCO]: {
		color: '#6f36dc',
		decimals: 12,
		genesisHash: SubstrateNetworkKeys.ROCOCO,
		isTestnet: true,
		logo: require('res/img/logos/Rococo.png'),
		order: 4,
		pathId: 'rococo',
		prefix: 0,
		title: 'Rococo',
		unit: 'ROC',
		url: 'wss://rococo-rpc.polkadot.io'
	},
	[SubstrateNetworkKeys.WESTEND]: {
		color: '#660D35',
		decimals: 12,
		genesisHash: SubstrateNetworkKeys.WESTEND,
		isTestnet: true,
		logo: require('res/img/logos/Polkadot.png'),
		order: 3,
		pathId: 'westend',
		prefix: 42,
		title: 'Westend',
		unit: 'WND',
		url: 'wss://westend-rpc.polkadot.io'
	}
};

const ethereumNetworkBase: Record<string, EthereumNetworkDefaultConstants> = {
	[EthereumNetworkKeys.FRONTIER]: {
		color: '#8B94B3',
		ethereumChainId: EthereumNetworkKeys.FRONTIER,
		isTestnet: false,
		order: 101,
		secondaryColor: colors.background.card,
		title: 'Ethereum'
	},
	[EthereumNetworkKeys.ROPSTEN]: {
		ethereumChainId: EthereumNetworkKeys.ROPSTEN,
		isTestnet: true,
		order: 104,
		title: 'Ropsten Testnet'
	},
	[EthereumNetworkKeys.GOERLI]: {
		ethereumChainId: EthereumNetworkKeys.GOERLI,
		isTestnet: true,
		order: 105,
		title: 'Görli Testnet'
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
