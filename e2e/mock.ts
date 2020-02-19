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

import {
	SUBSTRATE_NETWORK_LIST,
	SubstrateNetworkKeys
} from 'constants/networkSpecs';
import { TxRequestData } from 'types/scannerTypes';

export const signingTestIdentityPath = `//${
	SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].pathId
}//default`;

const setRemarkExtrinsicKusama =
	'47900000100005301021e169bcc4cdb062f1c85f971be770b6aea1bd32ac1bf7877aa54ccd73309014a20180000010c11111145030000fe030000b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafeaf2df518f7017e0442a1d01b0f175e0fa5d427470014c1c3ab2131e6250072a70ec';

export const createMockSignRequest = (): TxRequestData => ({
	bounds: {
		bounds: [
			{ x: '50', y: '50' },
			{ x: '100', y: '100' }
		],
		height: 1440,
		width: 1920
	},
	data: '',
	rawData: setRemarkExtrinsicKusama,
	target: 319,
	type: 'qr'
});
