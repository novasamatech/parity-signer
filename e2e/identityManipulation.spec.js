import testIDs from './testIDs';
import {
	testExist,
	testInput,
	testNotExist,
	testNotVisible,
	testScrollAndTap,
	testTap,
	testVisible
} from './e2eUtils';

const {
	TacScreen,
	AccountNetworkChooser,
	IdentityNew,
	IdentityBackup,
	IdentityPin,
	PathDerivation,
	PathDetail,
	PathsList
} = testIDs;

const pinCode = '123456';

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
		const substrateNetworkButtonIndex =
			AccountNetworkChooser.networkButton + '2'; //Need change if network list changes

		await testTap(AccountNetworkChooser.createButton);
		await testNotVisible(IdentityNew.seedInput);
		await testTap(IdentityNew.createButton);
		await testVisible(IdentityBackup.seedText);
		await testTap(IdentityBackup.nextButton);
		await testInput(IdentityPin.setPin, pinCode);
		await testInput(IdentityPin.confirmPin, pinCode);
		await testTap(IdentityPin.submitButton);
		await testVisible(AccountNetworkChooser.chooserScreen);
		await testScrollAndTap(
			substrateNetworkButtonIndex,
			testIDs.AccountNetworkChooser.chooserScreen
		);
		await testInput(IdentityPin.unlockPinInput, pinCode);
		await testTap(IdentityPin.unlockPinButton);
		await testExist(PathsList.pathCard + '//kusama_CC2//default');
	});

	const fundingPath = '//funding/0';

	it('derive a new key', async () => {
		await testTap(PathsList.deriveButton);
		await testInput(PathDerivation.nameInput, 'first one');
		await testInput(PathDerivation.pathInput, fundingPath);
		await testTap(PathDerivation.deriveButton);
		await testInput(IdentityPin.unlockPinInput, pinCode);
		await testTap(IdentityPin.unlockPinButton);
		await testExist(PathsList.pathCard + `//kusama_CC2${fundingPath}`);
	});

	it('delete a path', async () => {
		await testTap(PathsList.pathCard + `//kusama_CC2${fundingPath}`);
		await testTap(PathDetail.popupMenuButton);
		await testTap(PathDetail.deleteButton);
		await element(by.text('Delete')).tap();
		await testNotExist(PathsList.pathCard + `//kusama_CC2${fundingPath}`);
	});
});
