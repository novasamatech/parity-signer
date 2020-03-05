const commonConfig = require('../jestCommonConfig');

module.exports = {
	...commonConfig,
	cacheDirectory: '<rootDir>/test/.jest/e2e/cache',
	reporters: ['detox/runners/jest/streamlineReporter'],
	roots: ['<rootDir>/test/e2e/specs'],
	setupFilesAfterEnv: ['<rootDir>/test/e2e/setup.ts']
};
