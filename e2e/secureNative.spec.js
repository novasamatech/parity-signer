'use strict';

//import testIDs from './testIDs';
//import {
//	tapBack,
//	testExist,
//	testInput,
//	testNotExist,
//	testNotVisible,
//	testScrollAndTap,
//	testTap,
//	testUnlockPin,
//	testVisible
//} from './e2eUtils';
import {
	secureContains,
	secureDelete,
	//	secureEthkeySign,
	secureGet,
	securePut
	//	secureSubstrateSign
} from '../src/util/native.js';

const testKey = 'test_key';
const testPin = '424242';

describe('secure-native test', () => {
	beforeEach(async () => {
		if (device.getPlatform() === 'ios') {
			await device.clearKeychain();
		}
		await device.launchApp({ newInstance: true });
	});

	it('should store, retrieve, and delete a keychain item', async () => {
		//                await expect(element(by.id(testIDs.TacScreen.tacView))).toBeVisible();
		//                await element(by.id(testIDs.TacScreen.nativeModuleTestButton)).tap();
		//                await expect(element(by.id(testIDs.SecureNativeTest.nativeTestView))).toBeVisible();
		//                await testTap(testIDs.SecureNativeTest.startButton);
		//                await expect(element(by.id(testIDs.SecureNativeTest.succeedView))).toExist();
		await securePut(testKey, testPin, 0);
		expect(await secureContains(testKey)).toEqual(true);
		expect(await secureGet(testKey)).toEqual(testPin);
		await secureDelete(testKey);
		expect(await secureContains(testKey)).toEqual(false);
	});
});
