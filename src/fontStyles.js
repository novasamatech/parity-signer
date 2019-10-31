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

import fonts from './fonts';
import colors from './colors';
export default {
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
	t_codeS: {
		color: colors.bg_text_sec,
		fontFamily: 'monospace',
		fontSize: 11
	},
	t_important: {
		color: colors.bg_text,
		fontFamily: fonts.robotoBold,
		fontSize: 12
	},
	t_prefix: {
		color: colors.bg_text,
		fontFamily: fonts.roboto,
		fontSize: 14,
		textTransform: 'uppercase'
	},
	t_regular: {
		color: colors.bg_text,
		fontFamily: fonts.robotoRegular,
		fontSize: 12
	}
};
