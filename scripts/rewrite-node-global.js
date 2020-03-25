const pathLib = require('path');

module.exports = function ({ types: t }) {
	return {
		name: 'rewrite node global __dirname',
		visitor: {
			Identifier: function (path, state) {
				if (path.node.name === '__dirname') {
					const fallbackPath = `${state.cwd}/node_modules/@polkadot`;
					const fileName = state.file.opts.filename;
					path.replaceWith(
						t.stringLiteral(
							fileName !== undefined ? pathLib.dirname(fileName) : fallbackPath
						)
					);
				}
			}
		}
	};
};
