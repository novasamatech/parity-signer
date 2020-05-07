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

import { EthereumNetworkKeys } from 'constants/networkSpecs';
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

const { Main, PathDetail, SecurityHeader, SignedMessage, SignedTx } = testIDs;

const testSignedTx = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testUnlockPin(pinCode);
	await testVisible(SignedTx.qrView);
};

const testMultiPartExtrinsic = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testUnlockPin(pinCode);
	await testVisible(SignedMessage.qrView);
};

const testEthereumMessage = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testUnlockPin(pinCode);
	await testVisible(SignedMessage.qrView);
};

describe('Signing test', () => {
	testRecoverIdentity();

	describe('Substrate Signing Test', () => {
		it('should sign the set remarks request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkExtrinsic);
			await testSignedTx();
		});

		it('does not need sign again after pin input', async () => {
			await tapBack();
			await testTap(SecurityHeader.scanButton);
			await testVisible(SignedTx.qrView);
		});

		it('should sign transfer request', async () => {
			await launchWithScanRequest(ScanTestRequest.TransferExtrinsic);
			await testSignedTx();
		});

		it('should sign multipart request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkMultiPart);
			await testMultiPartExtrinsic();
		});
	});

	describe('Ethereum Signing Test', () => {
		it('generate Kovan account', async () => {
			await tapBack();
			const kovanNetworkButtonIndex =
				Main.networkButton + EthereumNetworkKeys.KOVAN;
			await testTap(testIDs.Main.addNewNetworkButton);
			await testScrollAndTap(
				kovanNetworkButtonIndex,
				testIDs.Main.chooserScreen
			);
			await testVisible(PathDetail.screen);
			await tapBack();
			await testExist(Main.chooserScreen);
		});

		it('should sign transactions', async () => {
			await launchWithScanRequest(ScanTestRequest.EthereumTransaction);
			await testSignedTx();
		});

		it('should sign message', async () => {
			await launchWithScanRequest(ScanTestRequest.EthereumMessage);
			await testEthereumMessage();
		});
	});
});
