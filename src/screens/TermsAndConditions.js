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
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialCommunityIcons';
import toc from '../../docs/terms-and-conditions.md';
import colors from '../colors';
import fonts from '../fonts';
import Button from '../components/Button';
import Markdown from '../components/Markdown';
import TouchableItem from '../components/TouchableItem';
import { saveToCAndPPConfirmation } from '../util/db';
import testIDs from '../../e2e/testIDs';

export default class TermsAndConditions extends React.PureComponent {
	state = {
		ppAgreement: false,
		tocAgreement: false
	};

	render() {
		const { navigation } = this.props;
		const { tocAgreement, ppAgreement } = this.state;
		return (
			<View style={styles.body} testID={testIDs.TacScreen.tacView}>
				<ScrollView contentContainerStyle={{}}>
					<Markdown>{toc}</Markdown>
				</ScrollView>

				<TouchableItem
					testID={testIDs.TacScreen.agreeTacButton}
					style={{
						alignItems: 'center',
						flexDirection: 'row'
					}}
					onPress={() => {
						this.setState({ tocAgreement: !tocAgreement });
					}}
				>
					<Icon
						name={tocAgreement ? 'checkbox-marked' : 'checkbox-blank-outline'}
						style={[styles.text, { fontSize: 30 }]}
					/>

					<Text style={[styles.text, { fontSize: 16 }]}>
						{'  I agree to the terms and conditions'}
					</Text>
				</TouchableItem>
				<TouchableItem
					style={{
						alignItems: 'center',
						flexDirection: 'row'
					}}
					onPress={() => {
						this.setState({ ppAgreement: !ppAgreement });
					}}
				>
					<Icon
						testID={testIDs.TacScreen.agreePrivacyButton}
						name={ppAgreement ? 'checkbox-marked' : 'checkbox-blank-outline'}
						style={[styles.text, { fontSize: 30 }]}
					/>

					<Text style={[styles.text, { fontSize: 16 }]}>
						<Text>{'  I agree to the '}</Text>
						<Text
							style={{ textDecorationLine: 'underline' }}
							onPress={() => {
								navigation.navigate('PrivacyPolicy');
							}}
						>
							privacy policy
						</Text>
					</Text>
				</TouchableItem>

				<Button
					buttonStyles={{ height: 60, marginTop: 10 }}
					testID={testIDs.TacScreen.nextButton}
					title="Next"
					disabled={!ppAgreement || !tocAgreement}
					onPress={async () => {
						const firstScreenActions = navigation.getParam(
							'firstScreenActions'
						);
						await saveToCAndPPConfirmation();
						navigation.dispatch(firstScreenActions);
					}}
				/>
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
		paddingBottom: 20,
		textAlign: 'center'
	},
	top: {
		flex: 1
	}
});
