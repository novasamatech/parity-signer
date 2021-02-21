import { NetworkProtocols, unknownNetworkPathId } from 'constants/networkSpecs';

export type NetworkProtocol = 'ethereum' | 'substrate' | 'unknown';

export type NetworkParams =
	| SubstrateNetworkParams
	| EthereumNetwork
	| UnknownNetworkParams;

export type SubstrateNetworkDefaultConstant = {
	color: string;
	decimals: number;
	deleted?: boolean;
	genesisHash: string;
	logo?: number;
	order: number;
	pathId: string;
	protocol?: NetworkProtocol;
	prefix: number;
	secondaryColor?: string;
	title: string;
	unit: string;
};

export type SubstrateNetworkBasics = {
	color?: string;
	decimals: number;
	deleted?: boolean;
	genesisHash: string;
	order?: number;
	pathId: string;
	protocol?: NetworkProtocol;
	prefix: number;
	secondaryColor?: string;
	title: string;
	unit: string;
};

export type SubstrateNetworkParams = {
	color: string;
	decimals: number;
	deleted: boolean;
	genesisHash: string;
	logo: number;
	order: number;
	pathId: string;
	protocol: NetworkProtocol;
	prefix: number;
	secondaryColor: string;
	title: string;
	unit: string;
};

export type EthereumNetworkDefaultConstants = {
	color?: string;
	ethereumChainId: string;
	logo?: number;
	order: number;
	pathId: string;
	protocol?: NetworkProtocol;
	secondaryColor?: string;
	title: string;
};

export type EthereumNetwork = {
	color: string;
	ethereumChainId: string;
	logo: number;
	order: number;
	pathId: string;
	protocol: NetworkProtocol;
	secondaryColor: string;
	title: string;
};

export type UnknownNetworkParams = {
	color: string;
	order: number;
	pathId: string;
	prefix: number;
	protocol: NetworkProtocol;
	secondaryColor: string;
	title: string;
};

export type Networks = Map<string, NetworkParams>;
export type SubstrateNetworks = Map<string, SubstrateNetworkParams>;

export function isSubstrateNetwork(network?: SubstrateNetworkParams | UnknownNetworkParams | EthereumNetwork | null): network is SubstrateNetworkParams {

	if (!network) {

		return false;
	}

	const { pathId, protocol } = network;

	return (
		protocol === NetworkProtocols.SUBSTRATE && pathId !== unknownNetworkPathId
	);
}

export function isEthereumNetwork(network: | SubstrateNetworkParams | UnknownNetworkParams | EthereumNetwork): network is EthereumNetwork {
	return (
		(network as EthereumNetwork).protocol === NetworkProtocols.ETHEREUM
	);
}

export function isUnknownNetworkParams(networkParams:
		| SubstrateNetworkParams
		| UnknownNetworkParams
		| EthereumNetwork): networkParams is UnknownNetworkParams {
	const { pathId, protocol } = networkParams as SubstrateNetworkParams;

	return (
		(protocol === NetworkProtocols.SUBSTRATE &&
			pathId === unknownNetworkPathId) ||
		protocol === NetworkProtocols.UNKNOWN
	);
}
