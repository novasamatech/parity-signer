module.exports = {
  "presets": ["module:metro-react-native-babel-preset"],
  "plugins": [
    [
      "rewrite-require",
      {
        "aliases": {
          "crypto": "react-native-crypto",
          "stream": "readable-stream",
          "_stream_duplex": "readable-stream/duplex",
          "_stream_passthrough": "readable-stream/passthrough",
          "_stream_readable": "readable-stream/readable",
          "_stream_transform": "readable-stream/transform",
          "_stream_writable": "readable-stream/writable",
          "@polkadot/wasm-crypto": "@plugnet/wasm-crypto-js",
          "@plugnet/util": "@polkadot/util",
          "vm": "vm-browserify"
        }
      }
    ]
  ]
} 