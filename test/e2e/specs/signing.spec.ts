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

import { by, element } from 'detox';

import { EthereumNetworkKeys } from 'constants/networkSpecs';
import {
	launchWithScanRequest,
	pinCode,
	tapBack,
	testExist,
	testInput,
	testScrollAndTap,
	testSetUpDefaultPath,
	testTap,
	testUnlockPin,
	testVisible
} from 'e2e/utils';
import { ScanTestRequest } from 'e2e/mockScanRequests';
import testIDs from 'e2e/testIDs';

const {
	TacScreen,
	AccountNetworkChooser,
	IdentityNew,
	PathDetail,
	SecurityHeader,
	TxDetails,
	SignedMessage,
	SignedTx,
	MessageDetails
} = testIDs;

const mockIdentityName = 'mockIdentity';
const mockSeedPhrase =
	'ability cave solid soccer gloom thought response hard around minor want welcome';

const testSignedTx = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testScrollAndTap(TxDetails.signButton, TxDetails.scrollScreen);
	await testUnlockPin(pinCode);
	await testVisible(SignedTx.qrView);
};

const testMultiPartExtrinsic = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testScrollAndTap(
		MessageDetails.signButton,
		MessageDetails.scrollScreen
	);
	await element(by.text('I understand the risks')).atIndex(0).tap();
	await testUnlockPin(pinCode);
	await testVisible(SignedMessage.qrView);
};

const testEthereumMessage = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testScrollAndTap(
		MessageDetails.signButton,
		MessageDetails.scrollScreen
	);
	await testUnlockPin(pinCode);
	await testVisible(SignedMessage.qrView);
};

describe('Signing test', () => {
	it('should have account list screen', async () => {
		await testVisible(TacScreen.tacView);
		await testTap(TacScreen.agreePrivacyButton);
		await testTap(TacScreen.agreeTacButton);
		await testTap(TacScreen.nextButton);
		await testVisible(AccountNetworkChooser.noAccountScreen);
	});

	it('recover a identity with seed phrase', async () => {
		await testTap(AccountNetworkChooser.recoverButton);
		await testVisible(IdentityNew.seedInput);
		await testInput(IdentityNew.nameInput, mockIdentityName);
		await element(by.id(IdentityNew.seedInput)).typeText(mockSeedPhrase);
		await element(by.id(IdentityNew.seedInput)).tapReturnKey();
		await testSetUpDefaultPath();
	});

	describe('Substrate Signing Test', () => {
		it('should sign the set remarks request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkExtrinsic);
			await testSignedTx();
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
				AccountNetworkChooser.networkButton + EthereumNetworkKeys.KOVAN;
			await testTap(testIDs.AccountNetworkChooser.addNewNetworkButton);
			await testScrollAndTap(
				kovanNetworkButtonIndex,
				testIDs.AccountNetworkChooser.chooserScreen
			);
			await testUnlockPin(pinCode);
			await testVisible(PathDetail.screen);
			await tapBack();
			await testExist(AccountNetworkChooser.chooserScreen);
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
