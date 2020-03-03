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

import { Platform } from 'react-native';

import { rawDataMap, ScanTestRequest } from 'e2e/mock';
import { TxRequestData } from 'types/scannerTypes';

type AndroidAppArgs = {
	scanRequest?: number;
};

type iOSAppArgs = Array<string>;

export interface AppProps {
	launchArgs?: AndroidAppArgs | iOSAppArgs;
}

export const getLaunchArgs = (props: AppProps): void => {
	console.log(props.launchArgs, typeof props.launchArgs);
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

const buildSignRequest = (rawData: string): TxRequestData => ({
	bounds: {
		bounds: [
			{ x: '50', y: '50' },
			{ x: '100', y: '100' }
		],
		height: 1440,
		width: 1920
	},
	data: '',
	rawData,
	target: 319,
	type: 'qr'
});

export const onMockBarCodeRead = (
	txRequest: ScanTestRequest,
	onBarCodeRead: (tx: any) => void
): void => {
	if (typeof rawDataMap[txRequest] === 'string') {
		onBarCodeRead(buildSignRequest(rawDataMap[txRequest] as string));
	} else if (Array.isArray(rawDataMap[txRequest])) {
		(rawDataMap[txRequest] as string[]).forEach((rawData: string): void => {
			onBarCodeRead(buildSignRequest(rawData));
		});
	}
};
