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

import React, { useContext, useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialCommunityIcons';

import toc from '../../docs/terms-and-conditions.md';

import containerStyles from 'styles/containerStyles';
import { GlobalState, GlobalStateContext } from 'stores/globalStateContext';
import testIDs from 'e2e/testIDs';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import Button from 'components/Button';
import Markdown from 'components/Markdown';
import TouchableItem from 'components/TouchableItem';
import { saveToCAndPPConfirmation } from 'utils/db';
import CustomScrollView from 'components/CustomScrollView';

export default function TermsAndConditions(
	props: NavigationProps<'TermsAndConditions'>
): React.ReactElement {
	const [ppAgreement, setPpAgreement] = useState<boolean>(false);
	const [tocAgreement, setTocAgreement] = useState<boolean>(false);

	const { setPolicyConfirmed, policyConfirmed } = useContext<GlobalState>(
		GlobalStateContext
	);
	const { navigation } = props;
	const onConfirm = async (): Promise<void> => {
		await saveToCAndPPConfirmation();
		setPolicyConfirmed(true);
	};

	return (
		<View style={containerStyles.background} testID={testIDs.TacScreen.tacView}>
			<CustomScrollView contentContainerStyle={{ paddingHorizontal: 16 }}>
				<Markdown>{toc}</Markdown>
			</CustomScrollView>

			{!policyConfirmed && (
				<View>
					<TouchableItem
						testID={testIDs.TacScreen.agreeTacButton}
						style={{
							alignItems: 'center',
							flexDirection: 'row',
							paddingHorizontal: 16,
							paddingVertical: 10
						}}
						onPress={(): void => setTocAgreement(!tocAgreement)}
					>
						<Icon
							name={tocAgreement ? 'checkbox-marked' : 'checkbox-blank-outline'}
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
						onPress={(): void => setPpAgreement(!ppAgreement)}
					>
						<Icon
							testID={testIDs.TacScreen.agreePrivacyButton}
							name={ppAgreement ? 'checkbox-marked' : 'checkbox-blank-outline'}
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

const styles = StyleSheet.create({
	icon: {
		color: colors.bg_text_sec,
		fontSize: 30
	}
});
