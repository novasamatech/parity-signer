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
	getNetworkSpecByKey,
	getAllNetworkSpecs,
	addNetworkSpec
} from '../util/db';
import { empty } from '../util/networkSpecs';

// https://github.com/polkadot-js/ui/blob/f2f36e2db07f5faec14ee43cf4295f5e8a6f3cfa/packages/reactnative-identicon/src/icons/Polkadot.tsx#L37.

// we will need the generate function to be standardized to take an ss58 check address and isSixPoint boolean flag and returns a Circle https://github.com/polkadot-js/ui/blob/ff351a0f3160552f38e393b87fdf6e85051270de/packages/ui-shared/src/polkadotIcon.ts#L12.

type NetworkSpec = {};

type State = {
	networkSpecs: Array<NetworkSpec>,
	newNetworkSpec: NetworkSpec,
	selectedKey: NetworkSpec
};

export default class NetworksStore extends Container<State> {
	state = {
		networkSpecs: [],
		newNetworkSpec: empty(),
		selectedSpec: {}
	};

	constructor(props) {
		super(props);
		this.refreshList();
	}

	async refreshList() {
		const networkSpecs = await getAllNetworkSpecs();

		this.setState({ networkSpecs });
	}

	async addNewNetwork(newNetworkSpec) {
		try {
			await addNetworkSpec(newNetworkSpec.genesisHash, newNetworkSpec);
		} catch (e) {
			console.error(e);
		}
	}

	async select(networkKey) {
		const selectedSpec = await getNetworkSpecByKey(`network_${networkKey}`);

		this.setState({ selectedSpec: JSON.parse(selectedSpec) });
	}

	async submitNew() {
		const network = this.state.newNetwork;

		if (network.networkKey) {
			const networkSpec = await getNetworkSpecByKey(network.networkKey);

			await addNetworkSpec(network.networkKey, networkSpec);

			this.setState({
				networks: this.state.networks.set(network.networkKey, network),
				newNetworkSpec: empty()
			});
		}
	}

	updateNetworkSpec(networkKey, updatedNetworkSpec) {}

	// async deleteNetwork(networkKey) {
	// 	const { networkSpecs } = this.state;

	// 	networkSpecs.delete(networkKey);
	// 	this.setState({ networkSpecs, selectedKey: '' });
	// 	await deleteDbNetwork(networkKey);
	// }

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
