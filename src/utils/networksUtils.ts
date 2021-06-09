import colors from 'styles/colors';
import { SubstrateNetworkParams } from 'types/networkTypes';

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
			const newNetworksList = Object.assign({}, acc);
			if (!defaultNetworks.hasOwnProperty(networkKey)) {
				// const newAcc = Object.assign({}, acc);
				newNetworksList[networkKey] = {
					...networkParams,
					logo: require('res/img/logos/Substrate_Dev.png')
				};
				return newNetworksList;
			}

			const defaultParams = defaultNetworks[networkKey];
			newNetworksList[networkKey] = {
				...networkParams,
				logo: defaultParams.logo
			};
			return newNetworksList;
		},
		defaultNetworks
	);
	return new Map(Object.entries(mergedNetworksObject));
};
