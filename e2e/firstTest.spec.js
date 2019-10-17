import testIds from "./testIds";

describe('Example', () => {
  beforeEach(async () => {
    // await device.reloadReactNative();
  });

  it('should have account list screen', async () => {
    await expect(element(by.id(testIds.TacScreen.tacView))).toBeVisible();
    await element(by.id(testIds.TacScreen.agreePrivacyButton)).tap();
    await element(by.id(testIds.TacScreen.agreeTacButton)).tap();
    await element(by.id(testIds.TacScreen.nextButton)).tap();
    await expect(element(by.id(testIds.AccountListScreen.accountList))).toBeVisible();
  });
});
