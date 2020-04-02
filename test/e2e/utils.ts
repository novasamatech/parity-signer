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

import { expect, element, by, device } from 'detox';

import testIDs from './testIDs';
const { IdentityPin, AccountNetworkChooser, PathDetail, PathsList } = testIDs;

export const pinCode = '000000';
const substrateNetworkButtonIndex =
	AccountNetworkChooser.networkButton + 'kusama';

export const testTap = async (buttonId: string): Promise<Detox.Actions<any>> =>
	await element(by.id(buttonId)).tap();

export const testVisible = async (componentId: string): Promise<void> =>
	await expect(element(by.id(componentId))).toBeVisible();

export const testExist = async (componentId: string): Promise<void> =>
	await expect(element(by.id(componentId))).toExist();

export const testNotExist = async (componentId: string): Promise<void> =>
	await expect(element(by.id(componentId))).toNotExist();

export const testNotVisible = async (componentId: string): Promise<void> =>
	await expect(element(by.id(componentId))).toBeNotVisible();

export const tapBack = async (): Promise<void> => {
	if (device.getPlatform() === 'ios') {
		await element(by.id(testIDs.Header.headerBackButton)).atIndex(0).tap();
	} else {
		await device.pressBack();
	}
};

export const testInput = async (
	inputId: string,
	inputText: string
): Promise<void> => {
	await element(by.id(inputId)).typeText(inputText);
	await element(by.id(inputId)).tapReturnKey();
};

export const testInputWithDone = async (
	inputId: string,
	inputText: string
): Promise<void> => {
	await element(by.id(inputId)).typeText(inputText);
	if (device.getPlatform() === 'ios') {
		await element(by.label('Done')).atIndex(0).tap();
	} else {
		await element(by.id(inputId)).tapReturnKey();
	}
};

export const testScrollAndTap = async (
	buttonId: string,
	screenId: string
): Promise<void> => {
	await waitFor(element(by.id(buttonId)))
		.toBeVisible()
		.whileElement(by.id(screenId))
		.scroll(100, 'down');
	await testTap(buttonId);
};

export const testUnlockPin = async (inputPin: string): Promise<void> => {
	await testInputWithDone(IdentityPin.unlockPinInput, inputPin);
};

export const testSetUpDefaultPath = async (): Promise<void> => {
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

export const launchWithScanRequest = async (
	txRequest: number
): Promise<void> => {
	await device.launchApp({
		launchArgs: { scanRequest: txRequest.toString() },
		newInstance: true,
		permissions: { camera: 'YES' }
	});
};
