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

import { by, device, element } from 'detox';

import testIDs from './testIDs';
import {
	tapBack,
	testExist,
	testInput,
	testInputWithDone,
	testNotExist,
	testNotVisible,
	testScrollAndTap,
	testTap,
	testUnlockPin,
	testVisible
} from './e2eUtils';

const {
	TacScreen,
	AccountNetworkChooser,
	IdentitiesSwitch,
	IdentityManagement,
	IdentityNew,
	IdentityBackup,
	IdentityPin,
	PathDerivation,
	PathDetail,
	PathsList,
	SecurityHeader,
	SignedTx,
	TxDetails
} = testIDs;

const pinCode = '123456';
const mockIdentityName = 'mockIdentity';
const substrateNetworkButtonIndex = AccountNetworkChooser.networkButton + '3'; //Need change if network list changes
const defaultPath = '//default',
	customPath = '//sunny_day/1';
const mockSeedPhrase =
	'split cradle example drum veteran swear cruel pizza guilt surface mansion film grant benefit educate marble cargo ignore bind include advance grunt exile grow';

const testSetUpDefaultPath = async (): Promise<void> => {
	await testInput(IdentityPin.setPin, pinCode);
	await testInputWithDone(IdentityPin.confirmPin, pinCode);
	await testVisible(AccountNetworkChooser.chooserScreen);
	await testScrollAndTap(
		substrateNetworkButtonIndex,
		testIDs.AccountNetworkChooser.chooserScreen
	);
	await testUnlockPin(pinCode);
	await testVisible(PathDetail.screen);
	await tapBack();
	await testExist(PathsList.screen);
};

describe('Load test', async () => {
	beforeAll(async () => {
		if (device.getPlatform() === 'ios') {
			await device.clearKeychain();
		}
		await device.launchApp({ permissions: { camera: 'YES' } });
	});

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

	it('derive a new key', async () => {
		await testTap(PathsList.deriveButton);
		await testInput(PathDerivation.pathInput, defaultPath);
		await testInput(PathDerivation.nameInput, 'first one');
		await testUnlockPin(pinCode);
		await testExist(PathsList.pathCard + `//kusama${defaultPath}`);
	});

	it('create a new identity with default substrate account', async () => {
		await element(by.id(IdentitiesSwitch.toggleButton))
			.atIndex(0)
			.tap();
		await testTap(IdentitiesSwitch.addIdentityButton);
		await testNotVisible(IdentityNew.seedInput);
		await testTap(IdentityNew.createButton);
		await testVisible(IdentityBackup.seedText);
		await testTap(IdentityBackup.nextButton);
		await element(by.text('Proceed')).tap();
		await testSetUpDefaultPath();
	});

	it('is able to create custom path', async () => {
		await tapBack();
		await testTap(testIDs.AccountNetworkChooser.addNewNetworkButton);
		await testScrollAndTap(
			testIDs.AccountNetworkChooser.addCustomNetworkButton,
			testIDs.AccountNetworkChooser.chooserScreen
		);
		await testInput(PathDerivation.pathInput, customPath);
		await testInput(PathDerivation.nameInput, 'custom network');
		await testUnlockPin(pinCode);
		await testExist(PathsList.pathCard + customPath);
	});

	it('should sign the transaction', async () => {
		await tapBack();
		await testTap(SecurityHeader.scanButton);
		await testScrollAndTap(TxDetails.signButton, TxDetails.scrollScreen);
		await testUnlockPin(pinCode);
		await testVisible(SignedTx.qrView);
	});

	it('delete a path', async () => {
		await tapBack();
		await testTap(AccountNetworkChooser.networkButton + '0');
		await testTap(PathsList.pathCard + `//kusama${defaultPath}`);
		await testTap(PathDetail.popupMenuButton);
		await testTap(PathDetail.deleteButton);
		await element(by.text('Delete')).tap();
		await testUnlockPin(pinCode);
		await testNotExist(PathsList.pathCard + `//kusama${defaultPath}`);
	});

	it('delete identity', async () => {
		await element(by.id(IdentitiesSwitch.toggleButton))
			.atIndex(0)
			.tap();
		await testTap(IdentitiesSwitch.manageIdentityButton);
		await testTap(IdentityManagement.popupMenuButton);
		await testTap(IdentityManagement.deleteButton);
		await element(by.text('Delete')).tap();
		await testUnlockPin(pinCode);
		await testVisible(IdentitiesSwitch.modal);
	});
});
