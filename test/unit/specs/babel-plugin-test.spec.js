const path = require('path');

const pluginTester = require('babel-plugin-tester').default;

const identifierReversePlugin = require('../../../scripts/rewrite-node-global');

const dirName = path.join(process.cwd(), 'node_modules/@polkadot');
pluginTester({
	formatResult: r => r,
	plugin: identifierReversePlugin,
	tests: {
		'changes this code': {
			code: 'var hello = __dirname',
			output: `var hello = "${dirName}";`
		}
	}
});
