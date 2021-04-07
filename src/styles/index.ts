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
		accentLight: '#eff2fe',
		app: '#fff',
		button: '#f7f9ff',
		card: '#eee',
		dark: '#000000',
		light: '#ddd'
	},
	border: {
		dark: '#000000',
		light: '#666666',
		signal: '#8E1F40'
	},
	navText: {
		// used for bottom navbar
		disabled: '#777',
		faded: '#9A9A9A',
		main: '#fff'
	},
	text: {
		accent: '#7535d4',
		disabled: '#aaa',
		error: '#D73400',
		faded: '#9A9A9A',
		main: '#111',
		white: '#fff'
	}
};

export const fonts = {
	bold: 'Montserrat-Bold',
	light: 'Montserrat-Light',
	regular: 'Montserrat-Regular'
};

export const components = {
	button: {
		alignSelf: 'center',
		backgroundColor: colors.background.accent,
		borderRadius: 60,
		height: 40,
		justifyContent: 'center',
		marginVertical: 8,
		paddingHorizontal: 28
	},
	buttonDisabled: {
		backgroundColor: colors.background.card
	},
	buttonFluid: {
		textAlign: 'center',
		width: '100%'
	},
	buttonText: {
		color: colors.text.white,
		fontFamily: fonts.regular,
		fontSize: 16
	}
};

export const fontStyles = StyleSheet.create({
	a_text: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 12,
		letterSpacing: 0.4,
		textTransform: 'uppercase'
	},
	h2: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 18
	},
	h_subheading: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 14,
		textTransform: 'uppercase'
	},
	quote: {
		color: colors.text.main,
		fontFamily: fonts.light,
		fontSize: 28
	},
	t_big: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 16
	},
	t_code: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 15
	},
	t_codeS: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 11,
		letterSpacing: 0.2
	},
	t_important: {
		color: colors.text.main,
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
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 14,
		textTransform: 'uppercase'
	},
	t_seed: {
		borderColor: colors.background.card,
		borderWidth: 0.8,
		color: colors.text.accent,
		fontFamily: fonts.light,
		fontSize: 18,
		letterSpacing: 0.7,
		lineHeight: 23,
		minHeight: 100,
		paddingHorizontal: 16,
		paddingVertical: 10
	}
});
