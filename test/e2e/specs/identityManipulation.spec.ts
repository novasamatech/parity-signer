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

import testIDs from 'e2e/testIDs';
import {
	tapBack,
	testExist,
	testInput,
	testNotExist,
	testNotVisible,
	testScrollAndTap,
	testTap,
	testUnlockPin,
	testVisible,
	testSetUpDefaultPath,
	pinCode
} from 'e2e/utils';
import { EthereumNetworkKeys } from 'constants/networkSpecs';

const {
	TacScreen,
	AccountNetworkChooser,
	IdentitiesSwitch,
	IdentityManagement,
	IdentityNew,
	IdentityBackup,
	PathDerivation,
	PathDetail,
	PathsList
} = testIDs;

const defaultPath = '//default';
const customPath = '//sunny_day/1';

describe('Load test', () => {
	it('should have account list screen', async () => {
		await testVisible(TacScreen.tacView);
		await testTap(TacScreen.agreePrivacyButton);
		await testTap(TacScreen.agreeTacButton);
		await testTap(TacScreen.nextButton);
		await testVisible(AccountNetworkChooser.noAccountScreen);
	});

	it('create a new identity with default substrate account', async () => {
		await testTap(AccountNetworkChooser.createButton);
		await testNotVisible(IdentityNew.seedInput);
		await testTap(IdentityNew.createButton);
		await testVisible(IdentityBackup.seedText);
		await testTap(IdentityBackup.nextButton);
		await element(by.text('Proceed')).tap();
		await testSetUpDefaultPath();
	});

	it('derive a new key', async () => {
		await testTap(PathsList.deriveButton);
		await testInput(PathDerivation.pathInput, defaultPath);
		await testInput(PathDerivation.nameInput, 'first one');
		await testUnlockPin(pinCode);
		await testExist(PathsList.pathCard + `//kusama${defaultPath}`);
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

	it('delete a path', async () => {
		await tapBack();
		await testTap(AccountNetworkChooser.networkButton + 'kusama');
		await testTap(PathsList.pathCard + `//kusama${defaultPath}`);
		await testTap(PathDetail.popupMenuButton);
		await testTap(PathDetail.deleteButton);
		await element(by.text('Delete')).tap();
		await testUnlockPin(pinCode);
		await testNotExist(PathsList.pathCard + `//kusama${defaultPath}`);
	});

	it('is able to create ethereum account', async () => {
		await tapBack();
		const ethereumNetworkButtonIndex =
			AccountNetworkChooser.networkButton + EthereumNetworkKeys.FRONTIER;
		await testTap(testIDs.AccountNetworkChooser.addNewNetworkButton);
		await testScrollAndTap(
			ethereumNetworkButtonIndex,
			testIDs.AccountNetworkChooser.chooserScreen
		);
		await testUnlockPin(pinCode);
		await testVisible(PathDetail.screen);
		await tapBack();
		await testExist(AccountNetworkChooser.chooserScreen);
	});

	it('is able to delete it', async () => {
		//'1' is frontier network chainId defined in networkSpecs.ts
		await testTap(AccountNetworkChooser.networkButton + 1);
		await testVisible(PathDetail.screen);
		await testTap(PathDetail.popupMenuButton);
		await testTap(PathDetail.deleteButton);
		await element(by.text('Delete')).tap();
		await testUnlockPin(pinCode);
		await testNotExist(AccountNetworkChooser.networkButton + 1);
	});

	it('delete identity', async () => {
		await element(by.id(IdentitiesSwitch.toggleButton)).atIndex(0).tap();
		await testTap(IdentitiesSwitch.manageIdentityButton);
		await testTap(IdentityManagement.popupMenuButton);
		await testTap(IdentityManagement.deleteButton);
		await element(by.text('Delete')).tap();
		await testUnlockPin(pinCode);
		await testVisible(AccountNetworkChooser.noAccountScreen);
	});
});
