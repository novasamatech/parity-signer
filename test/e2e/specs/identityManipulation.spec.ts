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
	pinCode,
	waitAlert
} from 'e2e/utils';

const {
	Alert,
	Main,
	IdentitiesSwitch,
	IdentityManagement,
	IdentityNew,
	IdentityBackup,
	PathDerivation,
	PathDetail,
	PathsList
} = testIDs;

const defaultPath = '//default';
const childPath = '/funding';
const customPath = '//sunny_day/1';
const secondPath = '/2';

describe('Load test', () => {
	it('create a new identity with default substrate account', async () => {
		await testTap(Main.createButton);
		await testNotVisible(IdentityNew.seedInput);
		await testTap(IdentityNew.createButton);
		await testVisible(IdentityBackup.seedText);
		await testTap(IdentityBackup.nextButton);
		await testTap(Alert.backupDoneButton);
		await testSetUpDefaultPath();
	});

	it('derive a new account from the path list', async () => {
		await testTap(PathsList.deriveButton);
		await testInput(PathDerivation.pathInput, defaultPath);
		await testInput(PathDerivation.nameInput, 'default');
		//await waitAlert();
		await testExist(PathsList.pathCard + `//kusama${defaultPath}`);
	});

	it('derive a new account from the derived account', async () => {
		await testTap(PathsList.pathCard + `//kusama${defaultPath}`);
		await testTap(PathDetail.deriveButton);
		await testInput(PathDerivation.pathInput, childPath);
		await testInput(PathDerivation.nameInput, 'child');
		//await waitAlert();
		await testExist(PathsList.pathCard + `//kusama${defaultPath}${childPath}`);
	});

	it('derive new account with quick derivation button', async () => {
		await tapBack();
		const deriveButtonId = `${PathsList.pathsGroup}${defaultPath}`;
		await testExist(deriveButtonId);
		await testTap(deriveButtonId);
		await testExist(PathsList.pathCard + `//kusama${defaultPath}//0`);
	});

	it('need pin after application go to the background', async () => {
		await device.sendToHome();
		await device.launchApp({ newInstance: false });
		await testTap(PathsList.deriveButton);
		await testUnlockPin(pinCode);
		await testInput(PathDerivation.pathInput, secondPath);
		await testInput(PathDerivation.nameInput, 'second');
		//await waitAlert();
		await testExist(PathsList.pathCard + `//kusama${secondPath}`);
	});

	it('is able to create custom path', async () => {
		await tapBack();
		await testTap(Main.addNewNetworkButton);
		await testTap(Main.addCustomNetworkButton);
		await testInput(PathDerivation.pathInput, customPath);
		await testInput(PathDerivation.nameInput, 'custom');
		//await waitAlert();
		await testVisible(Main.chooserScreen);
	});

	it('delete a path', async () => {
		await testTap(Main.backButton);
		await testTap(Main.networkButton + 'kusama');
		await testTap(PathsList.pathCard + `//kusama${defaultPath}`);
		await testTap(PathDetail.popupMenuButton);
		await testTap(PathDetail.deleteButton);
		await testTap(Alert.deleteAccount);
		await testNotExist(PathsList.pathCard + `//kusama${defaultPath}`);
	});

	it('delete identity', async () => {
		await element(by.id(IdentitiesSwitch.toggleButton)).atIndex(0).tap();
		await testTap(IdentitiesSwitch.manageIdentityButton);
		await testTap(IdentityManagement.popupMenuButton);
		await testTap(IdentityManagement.deleteButton);
		await testTap(Alert.deleteIdentity);
		await testUnlockPin(pinCode);
		await testVisible(Main.noAccountScreen);
	});
});
