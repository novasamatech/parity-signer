import { ScanTestRequest } from 'e2e/mockScanRequests';
import testIDs from 'e2e/testIDs';
import {
	launchWithScanRequest,
	pinCode,
	tapBack,
	testExist,
	testInput,
	testInputWithDone,
	testRecoverIdentity,
	testTap,
	testVisible
} from 'e2e/utils';

const {
	IdentityPin,
	PathDerivation,
	PathsList,
	SecurityHeader,
	SignedTx
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
		await testExist(PathsList.pathCard + `//kusama${passwordedPath}`);
	});

	it('should sign the set remarks request', async () => {
		await launchWithScanRequest(ScanTestRequest.passwordedAccountExtrinsic);
		await testTap(SecurityHeader.scanButton);
		await testInput(IdentityPin.unlockPinInput, pinCode);
		await testInputWithDone(IdentityPin.passwordInput, password);
		await testVisible(SignedTx.qrView);
	});

	it('does only need password again in the second try', async () => {
		await tapBack();
		await testTap(SecurityHeader.scanButton);
		await testInputWithDone(IdentityPin.passwordInput, password);
		await testVisible(SignedTx.qrView);
	});
});
