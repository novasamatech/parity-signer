const { defaults: tsjPreset } = require('ts-jest/presets');

module.exports = {
	...tsjPreset,
	globals: {
		'ts-jest': {
			babelConfig: true
		}
	},
	moduleNameMapper: {
		'\\.(jpg|ico|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga)$':
			'<rootDir>/test/mocks.ts',
		'^components/(.*)$': '<rootDir>/src/components/$1',
		'^constants/(.*)$': '<rootDir>/src/constants/$1',
		'^e2e/(.*)$': '<rootDir>/test/e2e/$1',
		'^res/(.*)$': '<rootDir>/res/$1',
		'^screens/(.*)$': '<rootDir>/src/screens/$1',
		'^stores/(.*)$': '<rootDir>/src/stores/$1',
		'^styles/(.*)$': '<rootDir>/src/styles/$1',
		'^types/(.*)$': '<rootDir>/src/types/$1',
		'^utils/(.*)$': '<rootDir>/src/utils/$1'
	},
	rootDir: './../../',
	testEnvironment: 'node',
	testPathIgnorePatterns: ['/node_modules/'],
	transform: {
		...tsjPreset.transform,
		'\\.js$': '<rootDir>/node_modules/react-native/jest/preprocessor.js'
	},
	verbose: true
};
