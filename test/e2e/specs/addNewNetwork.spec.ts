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

import {
	launchWithScanRequest,
	pinCode,
	tapBack,
	testExist,
	testRecoverIdentity,
	testScrollAndTap,
	testTap,
	testUnlockPin,
	testVisible
} from 'e2e/utils';
import {
	addNetworkGenesisHash,
	addNetworkPathId,
	ScanTestRequest
} from 'e2e/mockScanRequests';
import testIDs from 'e2e/testIDs';

const {
	DetailsTx,
	Main,
	PathDetail,
	PathsList,
	QrScanner,
	NetworkSettings,
	SecurityHeader,
	SignedTx
} = testIDs;

const testSignedTx = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testScrollAndTap(DetailsTx.signButton, DetailsTx.detailsScreen);
	await testUnlockPin(pinCode);
	await testExist(SignedTx.qrView);
};

describe('Signing ane exporting test', () => {
	testRecoverIdentity();

	it('is able to import a new network', async () => {
		await launchWithScanRequest(ScanTestRequest.AddNewNetwork);
		await testTap(SecurityHeader.scanButton);
		await testTap(QrScanner.networkAddSuccessButton);
		await testExist(NetworkSettings.networkCard + addNetworkGenesisHash);
	});

	it('derive a new account from the path list', async () => {
		await tapBack();
		const addedNetworkButtonIndex = Main.networkButton + addNetworkPathId;
		await testTap(testIDs.Main.addNewNetworkButton);
		await testTap(addedNetworkButtonIndex);
		await testUnlockPin(pinCode);
		await testVisible(PathDetail.screen);
		await tapBack();
		await testExist(PathsList.pathCard + `//${addNetworkPathId}`);
	});

	it('is able to sign a signing request', async () => {
		await launchWithScanRequest(ScanTestRequest.AddedNetworkRemarkExtrinsic);
		await testSignedTx();
	});
});
