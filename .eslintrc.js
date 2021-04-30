const commonRules = {
	'comma-dangle': ['error', 'never'],
	'import/order': [
		'error',
		{
			'newlines-between': 'always'
		}
	],
	'no-bitwise': 'off',
	'object-curly-spacing': ['error', 'always'],
	quotes: ['error', 'single', { avoidEscape: true }],
	'react-native/no-inline-styles': 'off',
	'sort-keys': [
		'error',
		'asc',
		{ caseSensitive: true, minKeys: 2, natural: false }
	]
};

module.exports = {
	extends: [
		'@react-native-community',
		'plugin:prettier/recommended',
		'plugin:import/errors',
		'plugin:import/warnings',
		'plugin:import/typescript'
	],
	globals: { inTest: 'writable' },
	ignorePatterns: ['**/node_modules/*'],
	overrides: [
		{
			files: ['e2e/*.spec.js', 'e2e/init.js', 'e2e/utils.js'],
			rules: {
				'no-undef': 'off'
			}
		},
		{
			env: { browser: true, es6: true, node: true },
			extends: [
				'@react-native-community',
				'plugin:@typescript-eslint/eslint-recommended',
				'plugin:@typescript-eslint/recommended',
				'plugin:import/errors',
				'plugin:import/typescript',
				'plugin:import/warnings',
				'plugin:prettier/recommended'
			],
			files: ['**/*.ts', '**/*.tsx'],
			parser: '@typescript-eslint/parser',
			parserOptions: {
				ecmaFeatures: { jsx: true },
				ecmaVersion: 2018,
				project: './tsconfig.json',
				sourceType: 'module'
			},
			plugins: ['@typescript-eslint', 'react-hooks'],
			rules: {
				...commonRules,
				'@typescript-eslint/ban-ts-comment': 'warn',
				'@typescript-eslint/camelcase': 0,
				'@typescript-eslint/no-explicit-any': 0,
				'@typescript-eslint/no-non-null-assertion': 0,
				'@typescript-eslint/no-use-before-define': 0, // ["error", { "variables": false }], // enable defining variables after react component;
				'@typescript-eslint/semi': ['error'],
				'no-void': 'off',
				'react-hooks/exhaustive-deps': 'warn',
				'react-hooks/rules-of-hooks': 'error',
				// deprecated: https://palantir.github.io/tslint/rules/no-use-before-declare/
				semi: 'off'
			}
		}
	],
	parserOptions: {
		ecmaFeatures: {
			jsx: true
		},
		ecmaVersion: 6,
		sourceType: 'module'
	},
	rules: {
		...commonRules,
		'no-unused-vars': ['error', { args: 'none' }]
	},
	settings: {
		'import/ignore': 'react-navigation',
		'import/resolver': {
			node: {
				extensions: ['.js', '.jsx', '.ts', '.tsx']
			},
			typescript: {
				alwaysTryTypes: true // always try to resolve types under `<roo/>@types` directory even it doesn't contain any source code, like `@types/unist`
			}
		},
		react: {
			pragma: 'React', // Pragma to use, default to "React"
			version: '16.9.0' // React version, default to the latest React stable release
		}
	}
};
