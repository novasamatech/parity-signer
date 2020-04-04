import {by, element} from 'detox';
import {ScanTestRequest} from 'e2e/mockScanRequests';
import testIDs from 'e2e/testIDs';
import {
	launchWithScanRequest,
	pinCode,
	testExist,
	testInput, testInputWithDone,
	testScrollAndTap,
	testSetUpDefaultPath,
	testTap,
	testUnlockPin,
	testVisible
} from 'e2e/utils';

const {
	AccountNetworkChooser,
	IdentityNew,
	IdentityPin,
	PathDerivation,
	PathsList,
	SecurityHeader,
	SignedTx,
	TxDetails
} = testIDs;

const passwordedPath = '//passworded';
const password = '111111';

const mockIdentityName = 'mockIdentity';
const mockSeedPhrase =
	'ability cave solid soccer gloom thought response hard around minor want welcome';

describe('Load test', () => {
	it('recover a identity with seed phrase', async () => {
		await testTap(AccountNetworkChooser.recoverButton);
		await testVisible(IdentityNew.seedInput);
		await testInput(IdentityNew.nameInput, mockIdentityName);
		await element(by.id(IdentityNew.seedInput)).typeText(mockSeedPhrase);
		await element(by.id(IdentityNew.seedInput)).tapReturnKey();
		await testSetUpDefaultPath();
	});

	it('derive a passworded account', async () => {
		await testTap(PathsList.deriveButton);
		await testInput(PathDerivation.pathInput, passwordedPath);
		await testTap(PathDerivation.togglePasswordButton);
		await testInput(PathDerivation.passwordInput, password);
		await testUnlockPin(pinCode);
		await testExist(PathsList.pathCard + `//kusama${passwordedPath}`);
	});

	it('should sign the set remarks request', async () => {
		await launchWithScanRequest(ScanTestRequest.passwordedAccountExtrinsic);
		await testTap(SecurityHeader.scanButton);
		await testScrollAndTap(TxDetails.signButton, TxDetails.scrollScreen);
		await testInput(IdentityPin.unlockPinInput, pinCode);
		await testInputWithDone(IdentityPin.passwordInput, password);
		await testVisible(SignedTx.qrView);
	});
});
