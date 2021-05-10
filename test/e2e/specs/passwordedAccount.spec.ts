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
	waitAlert
} from 'e2e/utils';

const {
	DetailsTx,
	IdentityPin,
	PathDerivation,
	PathDetail,
	PathsList,
	SecurityHeader,
	SignedTx
} = testIDs;

const passwordedPath = '//passworded';
const password = 'random';

describe('passworded account test', () => {
	testRecoverIdentity();

	it('derive a passworded account', async () => {
		await testTap(PathsList.deriveButton);
		await testInput(PathDerivation.pathInput, passwordedPath);
		await testTap(PathDerivation.togglePasswordButton);
		await testInput(PathDerivation.passwordInput, password);
		//await waitAlert();
		await testExist(PathsList.pathCard + `//kusama${passwordedPath}`);
	});

	describe('Kusama exporting test', () => {
		it('is able to export a hard derived account with password', async () => {
			await testTap(`${PathsList.pathCard}//kusama${passwordedPath}`);
			await testTap(PathDetail.popupMenuButton);
			await testTap(PathDetail.exportButton);
			await testInput(IdentityPin.passwordInput, password);
			await testExist(
				'secret:0xffa534554346807099dfbf034157798cf94541b357a3fe27f37c2175594f4bf5:0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe:passworded'
			);
		});
	});

	describe('Kusama signing Test', () => {
		it('should sign the set remarks request', async () => {
			await launchWithScanRequest(ScanTestRequest.PasswordedAccountExtrinsic);
			await testTap(SecurityHeader.scanButton);
			await testTap(DetailsTx.signButton);
			await testInputWithDone(IdentityPin.unlockPinInput, pinCode);
			await testInput(IdentityPin.passwordInput, password);
			await testExist(SignedTx.qrView);
		});

		it('does only need password again in the second try', async () => {
			await tapBack();
			await tapBack();
			await testTap(SecurityHeader.scanButton);
			await testTap(DetailsTx.signButton);
			await testInput(IdentityPin.passwordInput, password);
			await testExist(SignedTx.qrView);
		});
	});
});
