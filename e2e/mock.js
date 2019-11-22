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

'use strict';

import { NETWORK_LIST, SubstrateNetworkKeys } from '../src/constants';

export const signingTestIdentityPath = `//${NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].pathID}//default`;

const setRemarkExtrinsicKusama =
	'47900000100005301023c36776005aec2f32a34c109dc791a82edef980eec3be80da938ac9bcc68217220170000010c11111165030000fa030000e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636dbb5aefb451e26bd64faf476301f980437d87c0d88dec1a8c7a3eb3cc82e9bbb0ec';

export const createMockSignRequest = () => ({
	bounds: {
		height: 1440,
		origin: [],
		width: 1920
	},
	data: '',
	rawData: setRemarkExtrinsicKusama,
	target: 319,
	type: 'QR_CODE'
});
