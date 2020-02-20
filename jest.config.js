// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

const { defaults: tsjPreset } = require('ts-jest/presets');

module.exports = {
	...tsjPreset,
	cacheDirectory: '.jest/cache',
	globals: {
		'ts-jest': {
			babelConfig: true
		}
	},
	moduleNameMapper: {
		'^constants/(.*)$': '<rootDir>/src/constants/$1',
		'^styles/(.*)$': '<rootDir>/src/styles/$1',
		'^utils/(.*)$': '<rootDir>/src/utils/$1'
	},
	preset: 'react-native',
	roots: ['<rootDir>/specs'],
	setupFiles: ['<rootDir>/jest-setup.js'],
	testEnvironment: 'node',
	testPathIgnorePatterns: ['/node_modules/'],
	transform: {
		...tsjPreset.transform,
		'\\.js$': '<rootDir>/node_modules/react-native/jest/preprocessor.js'
	},
	verbose: true
};
