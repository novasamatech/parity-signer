import testIDs from './testIDs';
import {
	testExist,
	testInput,
	testNotVisible,
	testTap,
	testVisible
} from './e2eUtils';

const {
	TacScreen,
	AccountNetworkChooser,
	IdentityNew,
	IdentityBackup,
	IdentityPin,
	PathList
} = testIDs;

describe('Load test', async () => {
	beforeAll(async () => {
		if (device.getPlatform() === 'ios') {
			await device.clearKeychain();
			await device.launchApp();
		}
	});

	it('should have account list screen', async () => {
		await testVisible(TacScreen.tacView);
		await testTap(TacScreen.agreePrivacyButton);
		await testTap(TacScreen.agreeTacButton);
		await testTap(TacScreen.nextButton);
		await testVisible(AccountNetworkChooser.noAccountScreen);
	});

	it('create a new identity with default substrate account', async () => {
		const pinCode = '123456';
		const substrateNetworkButtonIndex =
			AccountNetworkChooser.networkButton + '5'; //Need change if network list changes

		await testTap(AccountNetworkChooser.createButton);
		await testNotVisible(IdentityNew.seedInput);
		await testTap(IdentityNew.createButton);
		await testVisible(IdentityBackup.seedText);
		await testTap(IdentityBackup.nextButton);
		await testInput(IdentityPin.setPin, pinCode);
		await testInput(IdentityPin.confirmPin, pinCode);
		await testTap(IdentityPin.submitButton);
		await testVisible(AccountNetworkChooser.chooserScreen);
		await waitFor(element(by.id(substrateNetworkButtonIndex)))
			.toBeVisible()
			.whileElement(by.id(testIDs.AccountNetworkChooser.chooserScreen))
			.scroll(100, 'down');
		await testTap(substrateNetworkButtonIndex);
		await testInput(IdentityPin.unlockPinInput, pinCode);
		await testTap(IdentityPin.unlockPinButton);
		await testExist(PathList.pathCard + '//kusama_CC2//default');
	});
});
