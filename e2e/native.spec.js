import testIDs from "./testIDs";

describe('First test', () => {
	beforeEach(async () => {
		// await device.clearKeychain();
	});

	it('should pass all the eccrypto function', async () => {
		await expect(element(by.id(testIDs.TacScreen.tacView))).toBeVisible();
		await element(by.id(testIDs.TacScreen.nativeModuleTestButton)).tap();
		await expect(element(by.id(testIDs.NativeTestScreen.nativeTestView))).toBeVisible();
		await expect(element(by.id(testIDs.NativeTestScreen.succeedView))).toNotExist();
		await element(by.id(testIDs.NativeTestScreen.startButton)).tap();
		await expect(element(by.id(testIDs.NativeTestScreen.succeedView))).toExist();
	})
});
