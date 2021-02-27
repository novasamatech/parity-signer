// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import { CommonActions, useNavigation } from '@react-navigation/native';
import Button from 'components/Button';
import CustomScrollView from 'components/CustomScrollView';
import Markdown from 'components/Markdown';
import TouchableItem from 'components/TouchableItem';
import testIDs from 'e2e/testIDs';
import React, { useCallback, useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialCommunityIcons';
import colors from 'styles/colors';
import containerStyles from 'styles/containerStyles';
import fontStyles from 'styles/fontStyles';
import { saveTaCAndPPConfirmation } from 'utils/db';
import { migrateAccounts } from 'utils/migrationUtils';

import tac from '../../docs/terms-and-conditions.md';
import { useTac } from '../hooks/useTac';

export default function TermsAndConditions(): React.ReactElement {
	const [isPPAgreed, setPpAgreement] = useState(false);
	const [isTacAgreed, setTacAgreement] = useState(false);
	const { dispatch, navigate } = useNavigation();
	const { ppAndTaCAccepted, setPpAndTaCAccepted } = useTac();

	const onConfirm = useCallback(async () => {
		saveTaCAndPPConfirmation()
			.then(() => {
				console.log('done saving YES')
				migrateAccounts()
					.then(() => {
						console.log('done migration')
						setPpAndTaCAccepted(true);
						const resetAction = CommonActions.reset({
							index: 0,
							routes: [{ name: 'LegacyAccountList' }]
						});

						dispatch(resetAction);
					// 	setDataLoaded(true);
					})
					.catch((e) => {
						console.error('migrateAccounts error', e);
					})
			}).catch((e)=> {
				console.error('saveTaCAndPPConfirmation error', e)
			});
	}, [dispatch, setPpAndTaCAccepted]);

	return (
		<View
			style={containerStyles.background}
			testID={testIDs.TacScreen.tacView}
		>
			<CustomScrollView contentContainerStyle={{ paddingHorizontal: 16 }}>
				<Markdown>{tac}</Markdown>
			</CustomScrollView>

			{!ppAndTaCAccepted && (
				<View>
					<TouchableItem
						onPress={(): void => setTacAgreement(!isTacAgreed)}
						style={{
							alignItems: 'center',
							flexDirection: 'row',
							paddingHorizontal: 16,
							paddingVertical: 10
						}}
						testID={testIDs.TacScreen.agreeTacButton}
					>
						<Icon
							name={isTacAgreed ? 'checkbox-marked' : 'checkbox-blank-outline'}
							style={styles.icon}
						/>

						<Text style={fontStyles.t_big}>
							{'  I agree to the terms and conditions'}
						</Text>
					</TouchableItem>
					<TouchableItem
						onPress={(): void => setPpAgreement(!isPPAgreed)}
						style={{
							alignItems: 'center',
							flexDirection: 'row',
							paddingHorizontal: 16
						}}
					>
						<Icon
							name={isPPAgreed ? 'checkbox-marked' : 'checkbox-blank-outline'}
							style={styles.icon}
							testID={testIDs.TacScreen.agreePrivacyButton}
						/>

						<Text style={fontStyles.t_big}>
							<Text>{'  I agree to the '}</Text>
							<Text
								onPress={(): void => { navigate('PrivacyPolicy'); }}
								style={{ textDecorationLine: 'underline' }}
							>
								privacy policy
							</Text>
						</Text>
					</TouchableItem>

					<Button
						disabled={!isPPAgreed || !isTacAgreed}
						onPress={onConfirm}
						style={styles.nextButton}
						testID={testIDs.TacScreen.nextButton}
						title="Next"
					/>
				</View>
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	icon: {
		color: colors.text.faded,
		fontSize: 30
	},
	nextButton: {
		marginBottom: 24,
		marginTop: 16
	}
});
