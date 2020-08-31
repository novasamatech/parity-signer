import {
	NetworkParams,
	SubstrateNetworkBasics,
	SubstrateNetworkParams
} from 'types/networkTypes';

export const serializeNetworks = (
	networks: Map<string, SubstrateNetworkBasics>
): string => {
	const networksEntries = Object.entries(networks);
	return JSON.stringify(networksEntries);
};

export const deserializeNetworks = (
	networksJson: string
): Map<string, SubstrateNetworkBasics> => {
	const networksEntries = JSON.parse(networksJson);
	return networksEntries.map(new Map(networksEntries));
};

export const deepCopyNetworks = (
	networks: Map<string, SubstrateNetworkBasics>
): Map<string, SubstrateNetworkBasics> =>
	deserializeNetworks(serializeNetworks(networks));

export const deepCopyNetwork = (
	identity: SubstrateNetworkBasics
): SubstrateNetworkBasics => JSON.parse(JSON.stringify(identity));

export const mergeNetworks = (
	defaultNetworks: Record<string, SubstrateNetworkParams>,
	newNetworksEntries: [string, SubstrateNetworkParams][]
): Map<string, SubstrateNetworkParams> => {
	const mergedNetworksObject = newNetworksEntries.reduce(
		(
			acc,
			[networkKey, networkParams],
			index
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
