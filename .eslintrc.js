const commonRules = {
  "no-bitwise": "off",
  "comma-dangle": ["error", "never"],
  "object-curly-spacing": ["error", "always"],
  "quotes": ["error", "single",  { "avoidEscape": true }],
  "react-native/no-inline-styles": "off",
  "sort-keys": ["error", "asc", {"caseSensitive": true, "natural": false, "minKeys": 2}],
  "import/order": ["error", {
    "newlines-between": "always"
  }]
};

module.exports = {
  extends: [
    "@react-native-community",
    "plugin:prettier/recommended",
    "plugin:import/errors",
    "plugin:import/warnings",
    "plugin:import/typescript"
   ],
  globals: { inTest: "writable" },
  overrides: [
    {
      files: ["e2e/*.spec.js", "e2e/init.js", "e2e/utils.js"],
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
        "plugin:import/typescript",
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
      plugins: ["@typescript-eslint", "react-hooks"],
      rules: {
        ...commonRules,
        "@typescript-eslint/no-explicit-any": 0,
        "@typescript-eslint/semi": ["error"],
        "@typescript-eslint/no-use-before-define": ["error", { "variables": false }], // enable defining variables after react component;
        "@typescript-eslint/no-non-null-assertion": 0,
        '@typescript-eslint/camelcase': 0,
        '@typescript-eslint/ban-ts-ignore': 0,
        "no-void": "off",
        "react-hooks/rules-of-hooks": "error",
        "react-hooks/exhaustive-deps": "warn",
        "semi": "off"
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
      },
      "typescript": {
        "alwaysTryTypes": true // always try to resolve types under `<roo/>@types` directory even it doesn't contain any source code, like `@types/unist`
      },
    },
    "import/ignore": "react-navigation",
    react: {
      "pragma": "React",  // Pragma to use, default to "React"
      "version": "16.9.0", // React version, default to the latest React stable release
    },
  },
  rules: {
    ...commonRules,
    "no-unused-vars": ["error", { "args": "none" }],
  }
};
