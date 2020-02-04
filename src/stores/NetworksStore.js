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

import { getNetworkSpecs, saveNetworkSpecs } from '../util/db';
import { empty } from '../util/networkSpecs';

// https://github.com/polkadot-js/ui/blob/f2f36e2db07f5faec14ee43cf4295f5e8a6f3cfa/packages/reactnative-identicon/src/icons/Polkadot.tsx#L37.

// we will need the generate function to be standardized to take an ss58 check address and isSixPoint boolean flag and returns a Circle https://github.com/polkadot-js/ui/blob/ff351a0f3160552f38e393b87fdf6e85051270de/packages/ui-shared/src/polkadotIcon.ts#L12.

type NetworkSpec = {};

type State = {
	networkSpecs: Array<NetworkSpec>,
	newNetworkSpec: NetworkSpec,
	selectedKey: NetworkSpec
};

const deepCopy = networkSpecs => JSON.parse(JSON.stringify(networkSpecs));

export default class NetworksStore extends Container<State> {
	state = {
		networkSpecs: [],
		newNetworkSpec: empty(),
		selectedSpec: null
	};

	constructor(props) {
		super(props);
		this.refreshList();
	}

	async refreshList() {
		const networkSpecs = await getNetworkSpecs();
		await this.setState({ networkSpecs });
	}

	async addNewNetwork(newNetworkSpec) {
		//TODO give feedback to UI
		if (!newNetworkSpec.genesisHash) {
			throw new Error('Must supply a network key to add new network spec.');
		}

		if (!newNetworkSpec.prefix) {
			throw new Error('Network spec must include prefix to be valid.');
		}
		const updatedNetworkSpecs = deepCopy(this.state.networkSpecs);
		updatedNetworkSpecs.push({ [newNetworkSpec.genesisHash]: newNetworkSpec });
		try {
			await saveNetworkSpecs(updatedNetworkSpecs);
		} catch (e) {
			//TODO give feedback to UI
			console.error(e);
		}
	}

	async select(networkKey) {
		const selectedSpec = this.state.networkSpecs.find(
			networkSpec => networkSpec.genesisHash === networkKey
		);
		await this.setState({ selectedSpec });
	}

	async submitNewNetworkSpec() {
		const { networkSpecs, newNetworkSpec } = this.state;

		//TODO give feedback to UI
		if (!newNetworkSpec.genesisHash) {
			throw new Error('Must supply a network key to add new network spec.');
		}

		if (!newNetworkSpec.prefix) {
			throw new Error('Network spec must include prefix to be valid.');
		}
		const updatedNetworkSpecs = deepCopy(networkSpecs);
		const networkIndex = updatedNetworkSpecs.findIndex(
			networkSpec => networkSpec.genesisHash === newNetworkSpec.genesisHash
		);
		if (networkIndex === -1) {
			updatedNetworkSpecs.push(newNetworkSpec);
		} else {
			updatedNetworkSpecs.splice(networkIndex, 1, newNetworkSpec);
		}

		await this.setState({
			networkSpecs: updatedNetworkSpecs,
			newNetworkSpec: empty()
		});

		try {
			await saveNetworkSpecs(updatedNetworkSpecs);
		} catch (e) {
			//TODO give feedback to UI
			console.error(e);
		}
	}

	async deleteNetwork(networkKey) {
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

	getNew() {
		return this.state.newNetworkSpec;
	}

	getSelected() {
		return this.state.selectedSpec;
	}

	getNetworkByKey() {
		return this.state.selectedKey;
	}

	getNetworkSpecs() {
		return this.state.networkSpecs;
	}
}
