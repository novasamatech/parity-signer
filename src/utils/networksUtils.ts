import colors from 'styles/colors';
import { SubstrateNetworkParams } from 'types/networkTypes';
import { NetworkParsedData } from 'types/scannerTypes';

export const serializeNetworks = (
	networks: Map<string, SubstrateNetworkParams>
): string => {
	const networksEntries = Array.from(networks.entries());
	return JSON.stringify(networksEntries);
};

export const deserializeNetworks = (
	networksJson: string
): Map<string, SubstrateNetworkParams> => {
	const networksEntries = JSON.parse(networksJson);
	return new Map(networksEntries);
};

export const deepCopyNetworks = (
	networks: Map<string, SubstrateNetworkParams>
): Map<string, SubstrateNetworkParams> =>
	deserializeNetworks(serializeNetworks(networks));

export const mergeNetworks = (
	defaultNetworks: Record<string, SubstrateNetworkParams>,
	newNetworksEntries: [string, SubstrateNetworkParams][]
): Map<string, SubstrateNetworkParams> => {
	const mergedNetworksObject = newNetworksEntries.reduce(
		(
			acc,
			[networkKey, networkParams]
		): Record<string, SubstrateNetworkParams> => {
			if (!defaultNetworks.hasOwnProperty(networkKey)) {
				acc[networkKey] = {
					...networkParams,
					logo: require('res/img/logos/Substrate_Dev.png')
				};
				return acc;
			}
			const defaultParams = defaultNetworks[networkKey];
			acc[networkKey] = { ...networkParams, logo: defaultParams.logo };
			return acc;
		},
		defaultNetworks
	);
	return new Map(Object.entries(mergedNetworksObject));
};

export const generateNetworkParamsFromParsedData = (
	networkParsedData: NetworkParsedData
): SubstrateNetworkParams => {
	const pathId = networkParsedData.data.title.toLowerCase();
	return {
		...networkParsedData.data,
		deleted: false,
		logo: require('res/img/logos/Substrate_Dev.png'),
		order: 0,
		pathId: pathId.replace(/ /g, '_'),
		protocol: 'substrate',
		secondaryColor: colors.background.card
	};
};
