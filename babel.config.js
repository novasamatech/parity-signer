module.exports = {
	plugins: [
		[
			'rewrite-require',
			{
				aliases: {
					'@plugnet/util': '@polkadot/util',
					'@polkadot/wasm-crypto': '@plugnet/wasm-crypto-js',
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
		]
	],
	presets: ['module:metro-react-native-babel-preset']
};
