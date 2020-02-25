// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

// @flow

import { Container } from 'unstated';

import {
	SubstrateNetworkParams,
	SubstrateNetworkBasics
} from 'types/networkSpecsTypes';
import { getNetworkSpecs, saveNetworkSpecs } from 'utils/db';
import {
	getCompleteSubstrateNetworkSpec,
	checkNewNetworkSpec
} from 'utils/networkSpecsUtils';

// https://github.com/polkadot-js/ui/blob/f2f36e2db07f5faec14ee43cf4295f5e8a6f3cfa/packages/reactnative-identicon/src/icons/Polkadot.tsx#L37.

// we will need the generate function to be standardized to take an ss58 check address and isSixPoint boolean flag and returns a Circle https://github.com/polkadot-js/ui/blob/ff351a0f3160552f38e393b87fdf6e85051270de/packages/ui-shared/src/polkadotIcon.ts#L12.

type State = {
	networkSpecs: Array<SubstrateNetworkParams>;
	newNetworkSpec: SubstrateNetworkBasics | null;
	selectedSpec: SubstrateNetworkParams | null;
};

const deepCopy = (
	networkSpecs: Array<SubstrateNetworkParams>
): Array<SubstrateNetworkParams> => JSON.parse(JSON.stringify(networkSpecs));

export default class NetworksStore extends Container<State> {
	state: State = {
		networkSpecs: [],
		newNetworkSpec: null,
		selectedSpec: null
	};

	constructor() {
		super();
		this.refreshList();
	}

	async refreshList(): Promise<void> {
		const networkSpecs = await getNetworkSpecs();
		await this.setState({ networkSpecs });
	}

	async select(networkKey: string): Promise<void> {
		const selectedSpec = this.state.networkSpecs.find(
			networkSpec => networkSpec.genesisHash === networkKey
		);
		await this.setState({ selectedSpec });
	}

	async submitNewNetworkSpec(): Promise<void> {
		const { networkSpecs, newNetworkSpec } = this.state;
		if (newNetworkSpec === null)
			throw new Error('NetworkKey is not initialized.');

		checkNewNetworkSpec(newNetworkSpec);
		const updatedNetworkSpecs = deepCopy(networkSpecs);
		const networkIndex = updatedNetworkSpecs.findIndex(
			networkSpec => networkSpec.genesisHash === newNetworkSpec.genesisHash
		);
		const completeNetworkSpec = getCompleteSubstrateNetworkSpec(newNetworkSpec);
		if (networkIndex === -1) {
			updatedNetworkSpecs.push(completeNetworkSpec);
		} else {
			updatedNetworkSpecs.splice(networkIndex, 1, completeNetworkSpec);
		}

		await this.setState({
			networkSpecs: updatedNetworkSpecs,
			newNetworkSpec: null
		});

		try {
			await saveNetworkSpecs(updatedNetworkSpecs);
		} catch (e) {
			//TODO give feedback to UI
			console.error(e);
		}
	}

	async deleteNetwork(networkKey: string): Promise<void> {
		const { networkSpecs } = this.state;
		const updatedNetworkSpecs = deepCopy(networkSpecs);
		const networkIndex = updatedNetworkSpecs.findIndex(
			networkSpec => networkSpec.genesisHash === networkKey
		);
		if (networkIndex === -1) return;

		updatedNetworkSpecs.splice(networkIndex, 1);
		await this.setState({
			networkSpecs: updatedNetworkSpecs,
			selectedSpec: null
		});
		try {
			await saveNetworkSpecs(updatedNetworkSpecs);
		} catch (e) {
			//TODO give feedback to UI
			console.error(e);
		}
	}

	getNew(): SubstrateNetworkBasics | null {
		return this.state.newNetworkSpec;
	}

	getSelected(): SubstrateNetworkBasics | null {
		return this.state.selectedSpec;
	}

	getNetworkSpecs(): SubstrateNetworkBasics[] {
		return this.state.networkSpecs;
	}
}
