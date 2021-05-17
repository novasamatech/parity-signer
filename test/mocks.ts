export default '';

jest.mock('@react-native-community/async-storage', () => {
	// a map/dict/kvs of types to return - the leaves of the def
	// are jest functions
	const apiMock = {
		getItem: jest.fn(() => {
			// mock's AsyncStorage.getItem()
			return JSON.stringify('some mock data');
		})
	};
	return apiMock;
});

jest.mock('react-native-secure-storage', () => {
	// a map/dict/kvs of types to return - the leaves of the def
	// are jest functions
	const apiMock = {
		getItem: jest.fn(() => {
			return JSON.stringify('some mock data');
		})
	};
	return apiMock;
});

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
