import { NetworkProtocols, unknownNetworkPathId } from 'constants/networkSpecs';

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
	order: number;
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
	order: number;
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

export function isSubstrateNetworkParams(
	networkParams:
		| SubstrateNetworkParams
		| UnknownNetworkParams
		| EthereumNetworkParams
): networkParams is SubstrateNetworkParams {
	const { protocol, pathId } = networkParams as SubstrateNetworkParams;
	return (
		protocol === NetworkProtocols.SUBSTRATE && pathId !== unknownNetworkPathId
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
	const { protocol, pathId } = networkParams as SubstrateNetworkParams;
	return (
		(protocol === NetworkProtocols.SUBSTRATE &&
			pathId === unknownNetworkPathId) ||
		protocol === NetworkProtocols.UNKNOWN
	);
}
