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
import { StyleSheet, Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialCommunityIcons';

import toc from '../../docs/terms-and-conditions.md';

import testIDs from 'e2e/testIDs';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import Button from 'components/Button';
import Markdown from 'components/Markdown';
import TouchableItem from 'components/TouchableItem';
import { saveToCAndPPConfirmation } from 'utils/db';
import CustomScrollview from 'components/CustomScrollView';

interface State {
	tocAgreement: boolean;
	ppAgreement: boolean;
}

export default class TermsAndConditions extends React.PureComponent<
	NavigationProps<{ disableButtons?: boolean }>,
	State
> {
	state: State = {
		ppAgreement: false,
		tocAgreement: false
	};

	render(): React.ReactElement {
		const { navigation } = this.props;
		const { tocAgreement, ppAgreement } = this.state;
		const disableButtons = navigation.getParam('disableButtons', false);

		const onConfirm = async (): Promise<void> => {
			await saveToCAndPPConfirmation();
			navigation.navigate('Welcome');
		};

		return (
			<View style={styles.body} testID={testIDs.TacScreen.tacView}>
				<CustomScrollview
					containerStyle={styles.scrollView}
					contentContainerStyle={{ paddingHorizontal: 16 }}
				>
					<Markdown>{toc}</Markdown>
				</CustomScrollview>

				{!disableButtons && (
					<View>
						<TouchableItem
							testID={testIDs.TacScreen.agreeTacButton}
							style={{
								alignItems: 'center',
								flexDirection: 'row',
								paddingHorizontal: 16,
								paddingVertical: 10
							}}
							onPress={(): void => {
								this.setState({ tocAgreement: !tocAgreement });
							}}
						>
							<Icon
								name={
									tocAgreement ? 'checkbox-marked' : 'checkbox-blank-outline'
								}
								style={styles.icon}
							/>

							<Text style={fontStyles.t_big}>
								{'  I agree to the terms and conditions'}
							</Text>
						</TouchableItem>
						<TouchableItem
							style={{
								alignItems: 'center',
								flexDirection: 'row',
								paddingHorizontal: 16
							}}
							onPress={(): void => {
								this.setState({ ppAgreement: !ppAgreement });
							}}
						>
							<Icon
								testID={testIDs.TacScreen.agreePrivacyButton}
								name={
									ppAgreement ? 'checkbox-marked' : 'checkbox-blank-outline'
								}
								style={styles.icon}
							/>

							<Text style={fontStyles.t_big}>
								<Text>{'  I agree to the '}</Text>
								<Text
									style={{ textDecorationLine: 'underline' }}
									onPress={(): void => {
										navigation.navigate('PrivacyPolicy');
									}}
								>
									privacy policy
								</Text>
							</Text>
						</TouchableItem>

						<Button
							testID={testIDs.TacScreen.nextButton}
							title="Next"
							disabled={!ppAgreement || !tocAgreement}
							onPress={onConfirm}
						/>
					</View>
				)}
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
	},
	icon: {
		color: colors.bg_text_sec,
		fontSize: 30
	},
	scrollView: {
		flex: 1
	}
});
