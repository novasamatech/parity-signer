import { ScanTestRequest } from 'e2e/mockScanRequests';
import testIDs from 'e2e/testIDs';
import {
	launchWithScanRequest,
	pinCode,
	testExist,
	testInput,
	testInputWithDone,
	testRecoverIdentity,
	testScrollAndTap,
	testTap,
	testUnlockPin,
	testVisible
} from 'e2e/utils';

const {
	IdentityPin,
	PathDerivation,
	PathsList,
	SecurityHeader,
	SignedTx,
	TxDetails
} = testIDs;

const passwordedPath = '//passworded';
const password = '111111';

describe('Load test', () => {
	testRecoverIdentity();

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
