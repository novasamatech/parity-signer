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

'use strict';

import React from 'react';
import { ScrollView, StyleSheet, View } from 'react-native';
import privacyPolicy from '../../docs/privacy-policy.md';
import colors from '../colors';
import fonts from '../fonts';
import Markdown from '../components/Markdown';

export default class PrivacyPolicy extends React.PureComponent {
	static navigationOptions = {
		headerBackTitle: 'Back',
		title: 'Privacy policy'
	};

	render() {
		return (
			<View style={styles.body}>
				<ScrollView contentContainerStyle={{}}>
					<Markdown>{privacyPolicy}</Markdown>
				</ScrollView>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		overflow: 'hidden',
		padding: 20
	},
	bottom: {
		flexBasis: 50,
		paddingBottom: 15
	},
	text: {
		color: colors.card_bg,
		fontFamily: fonts.regular,
		fontSize: 14,
		marginTop: 10
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	},
	titleTop: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		fontWeight: 'bold',
		textAlign: 'center'
	},
	top: {
		flex: 1
	}
});
