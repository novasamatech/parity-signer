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
	UnknownNetworkKeys,
	unknownNetworkParams,
	unknownNetworkPathId,
	SUBSTRATE_NETWORK_LIST
} from 'constants/networkSpecs';
import { SubstrateNetworkParams, NetworkParams } from 'types/networkTypes';
import {
	loadNetworks,
	saveNetworks,
	getMetadata,
	populateMetadata,
	dumpNetworks
} from 'utils/db';
import {
	deepCopyNetworks,
	generateNetworkParamsFromParsedData
} from 'utils/networksUtils';
import { MetadataHandle } from 'types/metadata';
import { rustTest, dbInit } from 'utils/native';

// https://github.com/polkadot-js/ui/blob/f2f36e2db07f5faec14ee43cf4295f5e8a6f3cfa/packages/reactnative-identicon/src/icons/Polkadot.tsx#L37.

// we will need the generate function to be standardized to take an ss58 check address and isSixPoint boolean flag and returns a Circle https://github.com/polkadot-js/ui/blob/ff351a0f3160552f38e393b87fdf6e85051270de/packages/ui-shared/src/polkadotIcon.ts#L12.

export type NetworksContextState = {
	registriesReady: boolean;
	startupAttraction: string;
	};

export function useNetworksContext(): NetworksContextState {
	const [registriesReady, setRegistriesReady] = useState<boolean>(false);
	const [startupAttraction, setStartupAttraction] = useState<string>('');

	//all initialization of built-in and saved networks in a single place to eliminate races
	useEffect(() => {
		const initNetworksAndRegistries = async function (): Promise<void> {
			console.log('=====SIGNER STARTING=====');
			let startingString = 'Signer loading...\nLoading metadata...';
			setStartupAttraction(startingString);
			console.log('Loading metadata...');
			await populateMetadata();
			console.log('Loading networks...');
			await dbInit();
			setRegistriesReady(true);
			console.log('====INITIALIZATION COMPLETE=====');
		};
		initNetworksAndRegistries();
	}, []);

	return {
		registriesReady,
		startupAttraction,
	};
}

export const NetworksContext = React.createContext({} as NetworksContextState);
