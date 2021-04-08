// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import { StyleSheet } from 'react-native';

export const colors = {
	background: {
		accent: '#5d7ff3',
		accentDark: '#4d6fe3',
		accentLight: '#eff2fe',
		accentMedium: '#E1E5F8',
		app: '#fff',
		button: '#f7f9ff',
		dark: '#000000',
		light: '#ddd',
		medium: '#999',
		negative: '#DF3535'
	},
	border: {
		dark: '#000000',
		light: '#ccc',
		signal: '#8E1F40'
	},
	navText: {
		// used for bottom navbar
		disabled: '#777',
		faded: '#9A9A9A',
		main: '#fff'
	},
	text: {
		accent: '#5d7ff3',
		cursor: '#1d99f1',
		dark: '#111',
		disabled: '#aaa',
		error: '#D73400',
		light: '#ccc',
		white: '#fff'
	}
};

export const fonts = {
	bold: 'Montserrat-Bold',
	light: 'Montserrat-Light',
	regular: 'Montserrat-Regular'
};

export const components = {
	app: {
		backgroundColor: colors.background.white
	},
	button: {
		alignSelf: 'center',
		backgroundColor: colors.background.accent,
		borderRadius: 60,
		height: 42,
		justifyContent: 'center',
		marginVertical: 8,
		paddingHorizontal: 28
	},
	buttonDisabled: {
		backgroundColor: colors.background.accentLight
	},
	buttonFluid: {
		textAlign: 'center',
		width: '100%'
	},
	buttonText: {
		color: colors.text.white,
		fontFamily: fonts.regular,
		fontSize: 17
	},
	link: {
		color: colors.text.accent,
		fontFamily: fonts.regular,
		textDecorationLine: 'underline'
	},
	linkSmall: {
		color: colors.text.accent,
		fontFamily: fonts.regular,
		fontSize: 14,
		textDecorationLine: 'underline'
	},
	page: {
		backgroundColor: colors.text.white,
		flex: 1,
		paddingHorizontal: 20,
		paddingTop: 34
	},
	pageWide: {
		backgroundColor: colors.text.white,
		flex: 1,
		paddingTop: 24
	},
	pageWideFullBleed: {
		backgroundColor: colors.text.white,
		flex: 1
	},
	textBlock: {
		fontFamily: fonts.regular,
		fontSize: 18,
		lineHeight: 24,
		marginBottom: 8
	},
	textBlockPreformatted: {
		backgroundColor: colors.background.accentDark,
		borderRadius: 10,
		marginVertical: 16,
		minHeight: 100,
		paddingHorizontal: 18,
		paddingVertical: 12
	},
	textBlockPreformattedText: {
		color: colors.text.white,
		fontFamily: fonts.regular,
		fontSize: 18,
		minHeight: 140
	},
	textInput: {
		marginVertical: 12
	},
	textInputLabel: {
		display: 'flex',
		flexDirection: 'row',
		height: 18,
		justifyContent: 'space-between',
		marginBottom: 8,
		paddingRight: 6
	},
	textInputLabelLeft: {
		flex: 1,
		fontFamily: fonts.regular
	},
	textInputLabelRight: {
		flex: 1,
		fontFamily: fonts.regular
	},
	textInputSuffix: {
		borderWidth: 0,
		color: colors.text.dark,
		flex: 0,
		paddingRight: 2,
		paddingTop: 13,
		position: 'absolute',
		right: 14
	},
	textInputText: {
		borderColor: colors.border.light,
		borderRadius: 10,
		borderWidth: 0.8,
		flex: 1,
		fontFamily: fonts.regular,
		fontSize: 18,
		paddingBottom: 10,
		paddingHorizontal: 14,
		paddingTop: 12
	},
	textInputTextError: {
		borderBottomColor: colors.text.error
	}
};

export const fontStyles = StyleSheet.create({
	a_text: {
		color: colors.text.dark,
		fontFamily: fonts.regular,
		fontSize: 12,
		letterSpacing: 0.4,
		textTransform: 'uppercase'
	},
	h2: {
		color: colors.text.dark,
		fontFamily: fonts.bold,
		fontSize: 18
	},
	h_subheading: {
		color: colors.text.dark,
		fontFamily: fonts.regular,
		fontSize: 14,
		textTransform: 'uppercase'
	},
	quote: {
		color: colors.text.dark,
		fontFamily: fonts.light,
		fontSize: 28
	},
	t_big: {
		color: colors.text.dark,
		fontFamily: fonts.regular,
		fontSize: 16
	},
	t_code: {
		color: colors.text.dark,
		fontFamily: fonts.regular,
		fontSize: 15
	},
	t_codeS: {
		color: colors.text.dark,
		fontFamily: fonts.regular,
		fontSize: 11,
		letterSpacing: 0.2
	},
	t_important: {
		color: colors.text.dark,
		fontFamily: fonts.bold,
		fontSize: 13
	},
	t_label: {
		backgroundColor: colors.text.accent,
		color: colors.text.accent,
		fontFamily: fonts.regular,
		fontSize: 13
	},
	t_prefix: {
		color: colors.text.dark,
		fontFamily: fonts.regular,
		fontSize: 14,
		textTransform: 'uppercase'
	}
});
