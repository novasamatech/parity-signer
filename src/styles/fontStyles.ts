// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import { StyleSheet } from 'react-native';

import fonts from './fonts';
import colors from './colors';

export default StyleSheet.create({
	h1: {
		color: colors.bg_text,
		fontFamily: fonts.robotoBold,
		fontSize: 22
	},
	h2: {
		color: colors.bg_text,
		fontFamily: fonts.robotoMedium,
		fontSize: 18
	},
	quote: {
		color: colors.bg_text,
		fontFamily: fonts.robotoLight,
		fontSize: 28
	},
	t_big: {
		color: colors.bg_text,
		fontFamily: fonts.roboto,
		fontSize: 16
	},
	t_code: {
		color: colors.bg_text_sec,
		fontFamily: fonts.robotoMono,
		fontSize: 18
	},
	t_codeS: {
		color: colors.bg_text_sec,
		fontFamily: fonts.robotoMono,
		fontSize: 11
	},
	t_important: {
		color: colors.bg_text,
		fontFamily: fonts.robotoBold,
		fontSize: 13
	},
	t_label: {
		backgroundColor: colors.label_text,
		color: colors.card_text,
		fontFamily: fonts.robotoMedium,
		fontSize: 13
	},
	t_prefix: {
		color: colors.bg_text,
		fontFamily: fonts.roboto,
		fontSize: 14,
		textTransform: 'uppercase'
	},
	t_regular: {
		color: colors.bg_text,
		fontFamily: fonts.roboto,
		fontSize: 12
	},
	t_seed: {
		borderColor: colors.card_bg,
		borderWidth: 0.8,
		color: colors.bg_button,
		fontFamily: fonts.light,
		fontSize: 20,
		letterSpacing: 0.1,
		lineHeight: 26,
		minHeight: 140,
		paddingHorizontal: 16,
		paddingVertical: 10
	}
});
