export const testTap = async buttonId => await element(by.id(buttonId)).tap();

export const testVisible = async componentId =>
	await expect(element(by.id(componentId))).toBeVisible();

export const testExist = async componentId =>
	await expect(element(by.id(componentId))).toExist();

export const testNotVisible = async componentId =>
	await expect(element(by.id(componentId))).toBeNotVisible();

// export const timeout = m => new Promise(r => setTimeout(r, m));

export const testInput = async (inputId, inputText) =>
	await element(by.id(inputId)).typeText(inputText);
