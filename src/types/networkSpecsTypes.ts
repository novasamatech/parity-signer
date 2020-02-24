import { NetworkProtocols } from 'constants/networkSpecs';

export type NetworkProtocol = 'ethereum' | 'substrate' | 'unknown';

export type NetworkParams =
	| SubstrateNetworkParams
	| EthereumNetworkParams
	| UnknownNetworkParams;

export type SubstrateNetworkParams = {
	color: string;
	decimals: number;
	genesisHash: string;
	logo: number;
	pathId: string;
	protocol: NetworkProtocol;
	prefix: number;
	secondaryColor: string;
	title: string;
	unit: string;
};

export type EthereumNetworkParams = {
	color: string;
	ethereumChainId: string;
	logo: number;
	protocol: NetworkProtocol;
	secondaryColor: string;
	title: string;
};

export type UnknownNetworkParams = {
	color: string;
	pathId: string;
	prefix: number;
	protocol: NetworkProtocol;
	secondaryColor: string;
	title: string;
};

export function isSubstrateNetworkParams(
	networkParams:
		| SubstrateNetworkParams
		| UnknownNetworkParams
		| EthereumNetworkParams
): networkParams is SubstrateNetworkParams {
	return (
		(networkParams as SubstrateNetworkParams).protocol ===
		NetworkProtocols.SUBSTRATE
	);
}

export function isEthereumNetworkParams(
	networkParams:
		| SubstrateNetworkParams
		| UnknownNetworkParams
		| EthereumNetworkParams
): networkParams is EthereumNetworkParams {
	return (
		(networkParams as EthereumNetworkParams).protocol ===
		NetworkProtocols.ETHEREUM
	);
}

export function isUnknownNetworkParams(
	networkParams:
		| SubstrateNetworkParams
		| UnknownNetworkParams
		| EthereumNetworkParams
): networkParams is UnknownNetworkParams {
	return (
		(networkParams as UnknownNetworkParams).protocol ===
		NetworkProtocols.UNKNOWN
	);
}
