export const testTap = async buttonId => await element(by.id(buttonId)).tap();

export const testVisible = async componentId =>
	await expect(element(by.id(componentId))).toBeVisible();

export const testExist = async componentId =>
	await expect(element(by.id(componentId))).toExist();

export const testNotVisible = async componentId =>
	await expect(element(by.id(componentId))).toBeNotVisible();

// export const timeout = m => new Promise(r => setTimeout(r, m));

export const testInput = async (inputId, inputText) => {
	await element(by.id(inputId)).typeText(inputText);
	if (device.getPlatform() !== 'ios') {
		await device.pressBack();
	}
};

export const testScrollAndTap = async (buttonId, screenId) => {
	await waitFor(element(by.id(buttonId)))
		.toBeVisible()
		.whileElement(by.id(screenId))
		.scroll(200, 'down');
	await testTap(buttonId);
};
