export default '';

jest.mock('react-native-randombytes', () => {
	const apiMock = {
		randomBytes: jest.fn(length => {
			let data = new Uint8Array(length);
			data = data.map(() => Math.floor(Math.random() * 90) + 10);
			return data;
		})
	};
	return apiMock;
});
