module.exports = {
	plugins: [
		[
			'rewrite-require',
			{
				aliases: {
					_stream_duplex: 'readable-stream/duplex',
					_stream_passthrough: 'readable-stream/passthrough',
					_stream_readable: 'readable-stream/readable',
					_stream_transform: 'readable-stream/transform',
					_stream_writable: 'readable-stream/writable',
					crypto: 'react-native-crypto',
					stream: 'readable-stream',
					vm: 'vm-browserify'
				}
			}
		],
		[
			'module-resolver',
			{
				alias: {
					components: './src/components',
					constants: './src/constants',
					e2e: './test/e2e',
					res: './res',
					screens: './src/screens',
					stores: './src/stores',
					styles: './src/styles',
					types: './src/types',
					utils: './src/utils'
				},
				root: ['.']
			}
		],
		['./scripts/rewrite-node-global']
	],
	presets: ['module:metro-react-native-babel-preset']
};
