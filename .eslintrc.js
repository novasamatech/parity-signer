module.exports = {
  extends: ["@react-native-community", "plugin:prettier/recommended"],
  parserOptions: {
    "ecmaVersion": 6,
    "sourceType": "module",
    "ecmaFeatures": {
      "jsx": true
    },
  },
  settings: {
    react: {
      "pragma": "React",  // Pragma to use, default to "React"
      "version": "16.9.0", // React version, default to the latest React stable release
    },
  },
  rules: {
  	"no-bitwise": "off",
    "comma-dangle": ["error", "never"],
    "object-curly-spacing": ["error", "always"],
    "quotes": ["error", "single",  { "avoidEscape": true }],
    "no-unused-vars": ["error", { "args": "none" }],
    "react-native/no-inline-styles": "off",
    "sort-keys": ["error", "asc", {"caseSensitive": true, "natural": false, "minKeys": 2}]
  }
};
