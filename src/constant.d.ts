type NetworkProtocol = 'ethereum' | 'substrate' | 'unknown';

type NetworkParams =
	| SubstrateNetworkParams
	| EthereumNetworkParams
	| UnknownNetworkParams;

type SubstrateNetworkBasicParams = {
	color?: string;
	decimals?: number;
	genesisHash: string;
	logo?: number;
	pathId: string;
	protocol?: 'substrate';
	prefix?: number;
	secondaryColor?: string;
	title: string;
	unit?: string;
};

type EthereumNetworkBasicParams = {
	color?: string;
	ethereumChainId: string;
	logo?: number;
	protocol?: 'ethereum';
	secondaryColor?: string;
	title: string;
};

type SubstrateNetworkParams = {
	color: string;
	decimals: number;
	genesisHash: string;
	logo: number;
	pathId: string;
	protocol: 'substrate';
	prefix: number;
	secondaryColor: string;
	title: string;
	unit: string;
};

type EthereumNetworkParams = {
	color: string;
	ethereumChainId: string;
	logo: number;
	protocol: 'ethereum';
	secondaryColor: string;
	title: string;
};

type UnknownNetworkParams = {
	color: string;
	pathId: string;
	prefix: number;
	protocol: 'unknown';
	secondaryColor: string;
	title: string;
};
