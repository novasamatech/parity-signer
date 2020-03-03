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
import { StyleSheet, View } from 'react-native';

import privacyPolicy from '../../docs/privacy-policy.md';

import colors from 'styles/colors';
import Markdown from 'components/Markdown';
import CustomScrollview from 'components/CustomScrollView';

export default class PrivacyPolicy extends React.PureComponent {
	render(): React.ReactElement {
		return (
			<View style={styles.body}>
				<CustomScrollview contentContainerStyle={{ paddingHorizontal: 16 }}>
					<Markdown>{privacyPolicy}</Markdown>
				</CustomScrollview>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		overflow: 'hidden'
	}
});
