// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import { useState } from 'react';
import { Platform } from 'react-native';

import { scanRequestDataMap, ScanTestRequest } from 'e2e/mockScanRequests';

type AndroidAppArgs = {
	scanRequest?: number;
};

type iOSAppArgs = Array<string>;

const timeout = (ms: number): Promise<void> =>
	new Promise(resolve => setTimeout(resolve, ms));

export interface AppProps {
	launchArgs?: AndroidAppArgs | iOSAppArgs;
}

export const getLaunchArgs = (props: AppProps): void => {
	const { launchArgs } = props;
	if (Platform.OS === 'ios') {
		if (Array.isArray(launchArgs) && launchArgs.includes('-detoxServer')) {
			global.inTest = true;
			const argsIndex = launchArgs.indexOf('-scanRequest');
			if (argsIndex !== -1)
				global.scanRequest = parseInt(launchArgs[argsIndex + 1], 10);
			return;
		}
	} else {
		if (launchArgs && launchArgs.hasOwnProperty('detoxServer')) {
			global.inTest = true;
			global.scanRequest = (launchArgs as AndroidAppArgs)?.scanRequest;
			return;
		}
	}
	global.inTest = false;
};

/*
const buildSignRequest = (rawData: string, data = ''): TxRequestData => ({
	bounds: {
		bounds: [
			{ x: '50', y: '50' },
			{ x: '100', y: '100' }
		],
		height: 1440,
		width: 1920
	},
	data,
	rawData,
	target: 319,
	type: Platform.OS === 'ios' ? 'org.iso.QRCode' : 'QR_CODE'
});
*/

/*
export function useInjectionQR(): [
	number,
	(
		txRequest: ScanTestRequest,
		onBarCodeRead: (tx: TxRequestData) => void
	) => Promise<void>
] {
	const [mockIndex, setMockIndex] = useState(0);

	const onMockBarCodeRead = async (
		txRequest: ScanTestRequest,
		onBarCodeRead: (tx: TxRequestData) => void
	): Promise<void> => {
		const scanRequest = scanRequestDataMap[txRequest];
		if (typeof scanRequest === 'string') {
			await timeout(200);
			await onBarCodeRead(buildSignRequest(scanRequest));
		} else if (Array.isArray(scanRequest)) {
			await timeout(200);
			if (mockIndex < scanRequest.length) {
				await onBarCodeRead(buildSignRequest(scanRequest[mockIndex]));
				setMockIndex(mockIndex + 1);
			}
		} else if (typeof scanRequest === 'object') {
			await timeout(200);
			await onBarCodeRead(
				buildSignRequest(scanRequest.rawData, scanRequest.data)
			);
		}
	};

	return [mockIndex, onMockBarCodeRead];
}*/
