import colors from './colors';

export const NetworkProtocols = Object.freeze({
	ETHEREUM: 'ethereum',
	SUBSTRATE: 'substrate',
	UNKNOWN: 'unknown'
});

// accounts for which the network couldn't be found (failed migration, removed network)
export const UnknownNetworkKeys = Object.freeze({
	UNKNOWN: 'unknown'
});

// ethereumChainId is used as Network key for Ethereum networks
/* eslint-disable sort-keys */
export const EthereumNetworkKeys = Object.freeze({
	FRONTIER: '1',
	ROPSTEN: '3',
	RINKEBY: '4',
	GOERLI: '5',
	KOVAN: '42',
	CLASSIC: '61'
});

/* eslint-enable sort-keys */

// genesisHash is used as Network key for Substrate networks
export const SubstrateNetworkKeys = Object.freeze({
	KUSAMA: '0xe3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636', // https://polkascan.io/pre/kusama-cc2/block/0
	SUBSTRATE_DEV:
		'0x4393a679e1830a487e8ae92733f089a80f3e24ba515b08dd8adb40fc6cedee8d' // substrate --dev commit ac6a2a783f0e1f4a814cf2add40275730cd41be1 hosted on wss://dev-node.substrate.dev .
});

const unknownNetworkBase = {
	[UnknownNetworkKeys.UNKNOWN]: {
		color: colors.bg_alert,
		protocol: NetworkProtocols.UNKNOWN,
		secondaryColor: colors.card_bg,
		title: 'Unknown network'
	}
};

const substrateNetworkBase = {
	[SubstrateNetworkKeys.KUSAMA]: {
		color: '#4C4646',
		decimals: 12,
		genesisHash: SubstrateNetworkKeys.KUSAMA,
		prefix: 2,
		title: 'Kusama CC2',
		unit: 'KSM'
	},
	[SubstrateNetworkKeys.SUBSTRATE_DEV]: {
		color: '#ff8c00',
		decimals: 15,
		genesisHash: SubstrateNetworkKeys.SUBSTRATE_DEV,
		prefix: 42,
		title: 'Substrate Development',
		unit: 'UNIT'
	}
	// [SubstrateNetworkKeys.POLKADOT]: {
	//   color: '#e6007a',
	//   decimals: 12,
	//   genesisHash: SubstrateNetworkKeys.POLKADOT,
	//   prefix: 0,
	//   title: 'Polkadot mainnet',
	//   unit: 'DOT'
	// }
};

const ethereumNetworkBase = {
	[EthereumNetworkKeys.FRONTIER]: {
		color: '#977CF6',
		ethereumChainId: EthereumNetworkKeys.FRONTIER,
		secondaryColor: colors.card_bg,
		title: 'Ethereum'
	},
	[EthereumNetworkKeys.CLASSIC]: {
		color: '#8C7166',
		ethereumChainId: EthereumNetworkKeys.CLASSIC,
		secondaryColor: colors.card_bg,
		title: 'Ethereum Classic'
	},
	[EthereumNetworkKeys.ROPSTEN]: {
		ethereumChainId: EthereumNetworkKeys.ROPSTEN,
		title: 'Ropsten Testnet'
	},
	[EthereumNetworkKeys.GOERLI]: {
		ethereumChainId: EthereumNetworkKeys.GOERLI,
		title: 'Görli Testnet'
	},
	[EthereumNetworkKeys.KOVAN]: {
		ethereumChainId: EthereumNetworkKeys.KOVAN,
		title: 'Kovan Testnet'
	}
};

const ethereumDefaultValues = {
	color: '#F2E265',
	protocol: NetworkProtocols.ETHEREUM,
	secondaryColor: colors.card_text
};

const substrateDefaultValues = {
	color: '#4C4646',
	protocol: NetworkProtocols.SUBSTRATE,
	secondaryColor: colors.card_bg
};

function setDefault(networkBase, defaultProps) {
	return Object.keys(networkBase).reduce((acc, networkKey) => {
		return {
			...acc,
			[networkKey]: {
				...defaultProps,
				...networkBase[networkKey]
			}
		};
	}, {});
}

export const ETHEREUM_NETWORK_LIST = Object.freeze(
	setDefault(ethereumNetworkBase, ethereumDefaultValues)
);
export const SUBSTRATE_NETWORK_LIST = Object.freeze(
	setDefault(substrateNetworkBase, substrateDefaultValues)
);
export const UNKNOWN_NETWORK = Object.freeze(unknownNetworkBase);

export const NETWORK_LIST = Object.freeze(
	Object.assign(
		{},
		SUBSTRATE_NETWORK_LIST,
		ETHEREUM_NETWORK_LIST,
		UNKNOWN_NETWORK
	)
);

export const TX_DETAILS_MSG = 'After signing and publishing you will have sent';
