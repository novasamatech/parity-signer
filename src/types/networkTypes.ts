import { MetadataHandle } from 'types/metadata';
import { NetworkProtocols, unknownNetworkPathId } from 'constants/networkSpecs';

export type NetworkProtocol = 'ethereum' | 'substrate' | 'unknown';

export type NetworkParams =
	| SubstrateNetworkParams
	| EthereumNetworkParams
	| UnknownNetworkParams;

export type SubstrateNetworkDefaultConstant = {
	color: string;
	decimals: number;
	deleted?: boolean;
	genesisHash: string;
	logo?: number;
	metadata: MetadataHandle | null;
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
	metadata: MetadataHandle | null;
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
	metadata: MetadataHandle | null;
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
	protocol?: NetworkProtocol;
	secondaryColor?: string;
	title: string;
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

export type Networks = Map<string, NetworkParams>;
export type SubstrateNetworks = Map<string, SubstrateNetworkParams>;

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
