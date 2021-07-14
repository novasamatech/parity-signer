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
import { ScanTestRequest } from 'e2e/mockScanRequests';
import testIDs from 'e2e/testIDs';

const {
	DetailsTx,
	Main,
	PathDetail,
	PathsList,
	SecurityHeader,
	SignedMessage,
	SignedTx
} = testIDs;

const testSignedTx = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testScrollAndTap(DetailsTx.signButton, DetailsTx.detailsScreen);
	await testUnlockPin(pinCode);
	await testExist(SignedTx.qrView);
};

const testSignedMessage = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testUnlockPin(pinCode);
	await testVisible(SignedMessage.qrView);
};

describe('Signing ane exporting test', () => {
	testRecoverIdentity();

	describe('Kusama Signing Test', () => {
		it('is able to export the signing account', async () => {
			await testTap(PathsList.pathCard + '//kusama');
			await testTap(PathDetail.popupMenuButton);
			await testTap(PathDetail.exportButton);
			await testExist(
				'secret:0xdf46d55a2d98695e9342b67edae6669e5c0b4e1a3895f1adf85989565b9ab827:0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe:Kusama root'
			);
			await tapBack();
			await testVisible(PathDetail.screen);
		});

		it('should sign the set remarks request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkExtrinsicKusama);
			await testSignedTx();
		});

		it('does not need sign again after pin input', async () => {
			await tapBack();
			await tapBack();
			await testTap(SecurityHeader.scanButton);
			await testTap(DetailsTx.signButton);
			await testExist(SignedTx.qrView);
		});

		it('should sign transfer request', async () => {
			await launchWithScanRequest(ScanTestRequest.TransferExtrinsicKusama);
			await testSignedTx();
		});

		it('should sign multipart request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkMultiPartKusama);
			await testSignedMessage();
		});

		it('should sign extrinsic hashes', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkHashKusama);
			await testSignedMessage();
		});
	});

	describe('Polkadot Signing Test', () => {
		it('generate Polkadot account', async () => {
			await tapBack();
			await tapBack();
			const PolkadotNetworkButtonIndex =
				Main.networkButton;
			await testTap(testIDs.Main.addNewNetworkButton);
			await testScrollAndTap(
				PolkadotNetworkButtonIndex,
				testIDs.Main.chooserScreen
			);
			await testVisible(PathDetail.screen);
		});

		it('should sign the set remarks request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkExtrinsicPolkadot);
			await testSignedTx();
		});

		it('should sign transfer request', async () => {
			await launchWithScanRequest(ScanTestRequest.TransferExtrinsicPolkadot);
			await testSignedTx();
		});

		it('should sign multipart request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkMultiPartPolkadot);
			await testSignedMessage();
		});
	});
});
