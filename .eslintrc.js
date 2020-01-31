module.exports = {
  extends: [
    "@react-native-community",
    "plugin:prettier/recommended",
    "plugin:import/errors",
    "plugin:import/warnings",
   ],
  globals: { inTest: "readonly" },
  overrides: [
    {
      files: ["e2e/*.spec.js", "e2e/init.js", "e2e/e2eUtils.js"],
      rules: {
        "no-undef": "off"
      }
    },
    {
      files: ["**/*.ts", "**/*.tsx"],
      env: { "browser": true, "es6": true, "node": true },
      extends: [
        "@react-native-community",
        "plugin:prettier/recommended",
        "plugin:import/errors",
        "plugin:import/warnings",
        "plugin:@typescript-eslint/eslint-recommended",
        "plugin:@typescript-eslint/recommended"
      ],
      parser: "@typescript-eslint/parser",
      parserOptions: {
        ecmaFeatures: { "jsx": true },
        ecmaVersion: 2018,
        sourceType: "module",
        project: "./tsconfig.json"
      },
      plugins: ["@typescript-eslint"],
      rules: {
        "indent": ["error", 2, { "SwitchCase": 1 }],
        "linebreak-style": ["error", "unix"],
        "quotes": ["error", "single"],
        "comma-dangle": ["error", "always-multiline"],
        "@typescript-eslint/no-explicit-any": 0
      }
    }
  ],
  parserOptions: {
    "ecmaVersion": 6,
    "sourceType": "module",
    "ecmaFeatures": {
      "jsx": true
    },
  },
  settings: {
    "import/resolver": {
      "node": {
        "extensions": [".js", ".jsx", ".ts", ".tsx"]
      }
    },
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
