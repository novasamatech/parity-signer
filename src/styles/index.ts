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

export const headerHeight = 40;

export const colors = {
	background: {
		alert: '#444444',
		app: '#151515',
		button: 'C0C0C0',
		card: '#262626',
		os: '#000000'
	},
	border: {
		dark: '#000000',
		light: '#666666',
		signal: '#8E1F40'
	},
	signal: {
		error: '#D73400',
		main: '#FF4077'
	},
	text: {
		disabled: '#2F2F2F',
		faded: '#9A9A9A',
		main: '#C0C0C0'
	}
};

export const fonts = {
	bold: 'Montserrat-Bold',
	light: 'Montserrat-Light',
	regular: 'Montserrat-Regular'
};

export const fontStyles = StyleSheet.create({
	a_button: {
		color: colors.background.app,
		fontFamily: fonts.regular,
		fontSize: 20
	},
	a_text: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 12,
		textTransform: 'uppercase',
		letterSpacing: 0.4
	},
	h1: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 22
	},
	h2: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 18
	},
	h_subheading: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 14,
		textTransform: 'uppercase'
	},
	i_large: {
		fontSize: 22
	},
	i_medium: {
		fontSize: 18
	},
	i_small: {
		fontSize: 10
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
		backgroundColor: colors.signal.main,
		color: colors.signal.main,
		fontFamily: fonts.regular,
		fontSize: 13
	},
	t_prefix: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 14,
		textTransform: 'uppercase'
	},
	t_regular: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 12
	},
	t_seed: {
		borderColor: colors.background.card,
		borderWidth: 0.8,
		color: colors.signal.main,
		fontFamily: fonts.light,
		fontSize: 18,
		letterSpacing: 0.7,
		lineHeight: 23,
		minHeight: 100,
		paddingHorizontal: 16,
		paddingVertical: 10
	}
});

export const containerStyles = StyleSheet.create({
	background: {
		backgroundColor: colors.background.app,
		flex: 1,
		flexDirection: 'column',
		overflow: 'hidden',
		paddingBottom: 0
	}
});
