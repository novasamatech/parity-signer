export default '';

jest.mock('react-native-crypto', () => {
	const apiMock = {
		randomBytes: jest.fn((length, _cn) => {
			let data = new Uint8Array(length);
			data = data.map(() => Math.floor(Math.random() * 90) + 10);
			return data;
		})
	};
	return apiMock;
});
