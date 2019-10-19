import testIDs from "./testIDs";

describe('Load test', () => {
  beforeEach(async () => {
    await device.reloadReactNative();
  });

  it('should have account list screen', async () => {
    await expect(element(by.id(testIDs.TacScreen.tacView))).toBeVisible();
    await element(by.id(testIDs.TacScreen.agreePrivacyButton)).tap();
    await element(by.id(testIDs.TacScreen.agreeTacButton)).tap();
    await element(by.id(testIDs.TacScreen.nextButton)).tap();
    await expect(element(by.id(testIDs.AccountListScreen.accountList))).toBeVisible();
  });
});
