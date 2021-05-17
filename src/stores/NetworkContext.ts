// Copyright 2015-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import { Metadata } from '@polkadot/metadata';
import { TypeRegistry } from '@polkadot/types';
import { getSpecTypes } from '@polkadot/types-known';
import { default as React, useEffect, useMemo, useState } from 'react';

import { deepCopyMap } from 'stores/utils';
import {
	dummySubstrateNetworkParams,
	ETHEREUM_NETWORK_LIST,
	UnknownNetworkKeys,
	unknownNetworkParams,
	unknownNetworkPathId
} from 'constants/networkSpecs';
import { SubstrateNetworkParams, NetworkParams } from 'types/networkTypes';
import { NetworkParsedData } from 'types/scannerTypes';
import {
	loadNetworks,
	saveNetworks,
	getMetadata,
	populateMetadata
} from 'utils/db';
import {
	deepCopyNetworks,
	generateNetworkParamsFromParsedData
} from 'utils/networksUtils';
import { MetadataHandle } from 'types/metadata';

// https://github.com/polkadot-js/ui/blob/f2f36e2db07f5faec14ee43cf4295f5e8a6f3cfa/packages/reactnative-identicon/src/icons/Polkadot.tsx#L37.

// we will need the generate function to be standardized to take an ss58 check address and isSixPoint boolean flag and returns a Circle https://github.com/polkadot-js/ui/blob/ff351a0f3160552f38e393b87fdf6e85051270de/packages/ui-shared/src/polkadotIcon.ts#L12.

//Map PathId to Polkadot.js/api spec names and chain names
type NetworkTypes = {
	alias?: string;
	chains: {
		[key: string]: string;
	};
};
type NetworkTypesMap = {
	[key: string]: NetworkTypes;
};
const networkTypesMap: NetworkTypesMap = {
	centrifuge: {
		alias: 'centrifuge-chain',
		chains: {
			centrifuge_amber: 'centrifuge-chain-amber',
			edgeware: 'edgeware'
		}
	},
	kusama: { chains: {} },
	polkadot: { chains: {} },
	rococo: { chains: {} },
	westend: { chains: {} }
};

export const getOverrideTypes = (
	registry: TypeRegistry,
	pathId: string
): any => {
	let specName = '',
		chainName = '';
	Object.entries(networkTypesMap).find(
		([networkName, networkTypes]: [string, NetworkTypes]) => {
			if (networkName === pathId) {
				specName = networkTypes.alias ?? networkName;
			} else if (networkTypes.chains.hasOwnProperty(pathId)) {
				const chainAlias = networkTypes.chains[pathId];
				specName = networkTypes.alias ?? networkName;
				chainName = chainAlias ?? pathId;
			} else {
				return false;
			}
			return true;
		}
	);
	return getSpecTypes(registry, chainName, specName, Number.MAX_SAFE_INTEGER);
};

export type GetNetwork = (networkKey: string) => NetworkParams;

export type GetSubstrateNetwork = (
	networkKey: string
) => SubstrateNetworkParams;

export type NetworksContextState = {
	populateNetworks(): Promise<void>;
	addNetwork(networkParsedData: NetworkParsedData): void;
	networks: Map<string, SubstrateNetworkParams>;
	allNetworks: Map<string, NetworkParams>;
	getSubstrateNetwork: GetSubstrateNetwork;
	getNetwork: GetNetwork;
	pathIds: string[];
	registries: Map<string, TypeRegistry>;
	registriesReady: boolean;
	startupAttraction: string;
	getTypeRegistry: (
		//networks: Map<string, SubstrateNetworkParams>,
		networkKey: string
	) => TypeRegistry | null;
	updateTypeRegistries: () => Promise<void>;
	initTypeRegistry: (networkKey: string) => Promise<TypeRegistry | null>;
	isMetadataActive: (metadataHandle: MetadataHandle) => boolean;
	setMetadataVersion: (
		networkKey: string,
		metadataHandle: MetadataHandle
	) => Promise<void>;
};

export function useNetworksContext(): NetworksContextState {
	const [substrateNetworks, setSubstrateNetworks] = useState<
		Map<string, SubstrateNetworkParams>
	>(new Map());
	const [registries, setRegistries] = useState(new Map());
	const [registriesReady, setRegistriesReady] = useState<boolean>(false);
	const [startupAttraction, setStartupAttraction] = useState<string>('');

	const allNetworks: Map<string, NetworkParams> = useMemo(() => {
		const ethereumNetworks: Map<string, NetworkParams> = new Map(
			Object.entries(ETHEREUM_NETWORK_LIST)
		);
		return new Map([
			...ethereumNetworks,
			...substrateNetworks,
			[UnknownNetworkKeys.UNKNOWN, unknownNetworkParams]
		]);
	}, [substrateNetworks]);

	const pathIds = useMemo(() => {
		const result = Array.from(substrateNetworks.values())
			.map(n => n.pathId)
			.concat([unknownNetworkPathId]);
		return result;
	}, [substrateNetworks]);

	//all initialization of built-in and saved networks in a single place to eliminate races
	useEffect(() => {
		const initNetworksAndRegistries = async function (): Promise<void> {
			console.log('=====SIGNER STARTING=====');
			let startingString = 'Signer loading...\nLoading metadata...';
			setStartupAttraction(startingString);
			console.log('Loading metadata...');
			await populateMetadata();
			startingString = startingString + '\nLoading networks...';
			setStartupAttraction(startingString);
			console.log('Loading networks...');
			const initNetworkSpecs = await loadNetworks();
			startingString = startingString + '\nRegistering types...';
			setStartupAttraction(startingString);
			console.log('Populating registries...');
			const initRegistries = new Map();
			for (const networkKey of Array.from(initNetworkSpecs.keys())) {
				try {
					const networkParams = initNetworkSpecs.get(networkKey)!;
					console.log('Registering network:');
					console.log(networkParams.pathId);
					const metadataHandle = networkParams.metadata;
					if (metadataHandle) {
						const networkMetadataRaw = await getMetadata(metadataHandle);
						const newRegistry = new TypeRegistry();
						const overrideTypes = getOverrideTypes(
							newRegistry,
							networkParams.pathId
						);
						//const overrideTypes = getSpecTypes(newRegistry, networkParams.pathId, metadataHandle.specName, Number.MAX_SAFE_INTEGER);
						newRegistry.register(overrideTypes);
						const metadata = new Metadata(newRegistry, networkMetadataRaw);
						newRegistry.setMetadata(metadata);
						initRegistries.set(networkKey, newRegistry);
						startingString =
							startingString +
							'\nRegistered metadata ' +
							metadataHandle.specName +
							' v ' +
							metadataHandle.specVersion +
							' for network ' +
							networkParams.pathId;
						setStartupAttraction(startingString);
						console.log('Success!!!');
					} else {
						startingString =
							startingString +
							'\nRegistered network ' +
							networkParams.pathId +
							' without metadata!';
						setStartupAttraction(startingString);
						console.log('Success!!!');
					}
				} catch (e) {
					console.log('Init network registration error', e);
				}
			}
			setSubstrateNetworks(initNetworkSpecs);
			setRegistries(initRegistries);
			setRegistriesReady(true);
			console.log('====INITIALIZATION COMPLETE=====');
		};
		initNetworksAndRegistries();
	}, []);

	async function populateNetworks(): Promise<void> {
		const initNetworkSpecs = await loadNetworks();
		console.log(initNetworkSpecs);
		setSubstrateNetworks(initNetworkSpecs);
		console.log(substrateNetworks);
		console.log('networks loaded');
	}

	async function initTypeRegistry(networkKey: string): Promise<null> {
		try {
			console.log('initTypeRegistry invoked');
			const networkParams = substrateNetworks.get(networkKey)!;
			const metadataHandle = networkParams.metadata;

			const networkMetadataRaw = await getMetadata(metadataHandle);

			const newRegistries = deepCopyMap(registries);
			if (newRegistries.has(networkKey)) newRegistries.delete(networkKey)!;

			const newRegistry = new TypeRegistry();
			//const overrideTypes = getOverrideTypes(newRegistry, networkParams.pathId);
			//console.log(overrideTypes);
			//newRegistry.register(overrideTypes);
			const metadata = new Metadata(newRegistry, networkMetadataRaw);
			newRegistry.setMetadata(metadata);
			newRegistries.set(networkKey, newRegistry);
			setRegistries(newRegistries);
			return null;
		} catch (e) {
			console.log('error', e);
			return null;
		}
	}

	async function updateTypeRegistries(): Promise<void> {
		console.log('Registries update invoked');
		console.log(substrateNetworks);
		for (const networkKey of Array.from(substrateNetworks.keys())) {
			console.log('initializing network:');
			console.log(networkKey);
			await initTypeRegistry(networkKey);
			console.log(registries);
		}
		return;
	}

	function getSubstrateNetworkParams(
		networkKey: string
	): SubstrateNetworkParams {
		return substrateNetworks.get(networkKey) || dummySubstrateNetworkParams;
	}

	function getNetwork(networkKey: string): NetworkParams {
		return allNetworks.get(networkKey) || dummySubstrateNetworkParams;
	}

	function addNetwork(networkParsedData: NetworkParsedData): void {
		const newNetworkParams = generateNetworkParamsFromParsedData(
			networkParsedData
		);
		const networkKey = newNetworkParams.genesisHash;
		const newNetworksList = deepCopyNetworks(substrateNetworks);
		newNetworksList.set(networkKey, newNetworkParams);
		setSubstrateNetworks(newNetworksList);
		saveNetworks(newNetworkParams);
	}

	function getTypeRegistry(networkKey: string): TypeRegistry | null {
		try {
			if (registries.has(networkKey)) {
				return registries.get(networkKey)!;
			}
			return null;
		} catch (e) {
			console.log('error', e);
			return null;
		}
	}

	async function setMetadataVersion(
		networkKey: string,
		metadataHandle: MetadataHandle
	): Promise<void> {
		const newNetworkParams = substrateNetworks.get(networkKey);
		if (!newNetworkParams) return;
		newNetworkParams.metadata = metadataHandle;
		const newNetworksList = deepCopyNetworks(substrateNetworks);
		newNetworksList.set(networkKey, newNetworkParams);
		const newRegistries = deepCopyMap(registries);
		try {
			const networkMetadataRaw = await getMetadata(metadataHandle);
			const newRegistry = new TypeRegistry();
			const metadata = new Metadata(newRegistry, networkMetadataRaw);
			newRegistry.setMetadata(metadata);
			newRegistries.set(networkKey, newRegistry);
		} catch (e) {
			console.log('Init network registration error', e);
			return;
		}
		setSubstrateNetworks(newNetworksList);
		saveNetworks(newNetworkParams);
		setRegistries(newRegistries);
	}

	function isMetadataActive(metadataHandle: MetadataHandle): boolean {
		//weird development tool
		//console.log('-----========------');
		//console.log(dumpNetworksData());
		//console.log('-----========------');

		for (const network of substrateNetworks.entries()) {
			if (
				network[1].metadata &&
				network[1].metadata.hash === metadataHandle.hash
			) {
				console.log('Its a match');
				return true;
			}
		}
		return false;
	}

	//This is a placeholder function to emulate rust-based native db
	/*
	function dumpNetworksData(): string {
		return JSON.stringify(Array.from(substrateNetworks.entries()));
	}*/

	return {
		addNetwork,
		allNetworks,
		getNetwork,
		getSubstrateNetwork: getSubstrateNetworkParams,
		getTypeRegistry,
		initTypeRegistry,
		isMetadataActive,
		networks: substrateNetworks,
		pathIds,
		populateNetworks,
		registries,
		registriesReady,
		setMetadataVersion,
		startupAttraction,
		updateTypeRegistries
	};
}

export const NetworksContext = React.createContext({} as NetworksContextState);
