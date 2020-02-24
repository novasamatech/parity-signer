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

import React from 'react';
import { StyleSheet } from 'react-native';
import { default as MarkdownRender } from 'react-native-markdown-renderer';

import colors from 'styles/colors';
import fonts from 'styles/fonts';

export default class Markdown extends React.PureComponent<any> {
	render(): React.ReactElement {
		return (
			<MarkdownRender
				style={StyleSheet.create({
					listOrderedItemIcon: {
						color: colors.bg_text,
						marginRight: 3,
						marginTop: 19
					},
					listUnorderedItemIcon: {
						color: colors.card_bg,
						fontFamily: fonts.bold,
						marginRight: 3,
						marginTop: 19
					},
					text: {
						color: colors.bg_text_sec,
						fontFamily: fonts.regular,
						fontSize: 14,
						marginTop: 10
					}
				})}
				{...this.props}
			/>
		);
	}
}
