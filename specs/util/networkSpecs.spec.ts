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

'use strict';

import {
	defaultNetworkSpecs,
	getCompleteSubstrateNetworkSpec
} from 'utils/networkSpecsUtils';

describe('network specs', () => {
	it('should return a valid empty', () => {
		const emptyNetworkSpec = getCompleteSubstrateNetworkSpec({
			genesisHash: 'aaaaaa',
			pathId: '/test',
			title: 'CustomTestNet',
			unit: 'CTN'
		});

		expect(emptyNetworkSpec).toBeDefined();
		expect(emptyNetworkSpec.color).toBeUndefined();
		expect(emptyNetworkSpec.title).toBeUndefined();
		expect(emptyNetworkSpec.genesisHash).toBeUndefined();
		expect(emptyNetworkSpec.prefix).toBeUndefined();
		expect(emptyNetworkSpec.protocol).toBeUndefined();
		expect(emptyNetworkSpec.decimals).toBeUndefined();
	});

	it('should return valid defaults', () => {
		const defaults = defaultNetworkSpecs();

		defaults.forEach(networkSpec => {
			expect(networkSpec).toBeDefined();
			expect(networkSpec.color).toBeDefined();
			expect(networkSpec.title).toBeDefined();
			expect(networkSpec.genesisHash).toBeDefined();
			expect(networkSpec.prefix).toBeDefined();
			expect(networkSpec.protocol).toBeDefined();
			expect(networkSpec.decimals).toBeDefined();
		});
	});
});
