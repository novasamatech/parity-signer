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

import { useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import Button from 'components/Button';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import styles from 'modules/main/styles';
import React from 'react';
import { Text, View } from 'react-native';
import fontStyles from 'styles/fontStyles';
import { RootStackParamList } from 'types/routes';

export default function OnBoardingView({ hasLegacyAccount }: {hasLegacyAccount: boolean}): React.ReactElement {
	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();

	function TextButton({ isRecover, text }: {
		text: string;
		isRecover: boolean;
	}): React.ReactElement {
		return (
			<Text
				onPress={(): void => navigation.navigate('RecoverAccount', { isRecover })}
				style={[fontStyles.quote, { textDecorationLine: 'underline' }]}
				testID={
					isRecover ? testIDs.Main.recoverButton : testIDs.Main.createButton
				}
			>
				{text}
			</Text>
		);
	}

	return (
		<SafeAreaScrollViewContainer
			contentContainerStyle={styles.scrollContent}
			testID={testIDs.Main.noAccountScreen}
		>
			<View style={styles.onboardingWrapper}>
				<TextButton isRecover={false}
					text="Create" />
				<Text style={fontStyles.quote}> or </Text>
				<TextButton isRecover={true}
					text="recover" />
				<Text style={fontStyles.quote}>your identity to get started.</Text>
				{hasLegacyAccount && (
					<Button
						onPress={(): void => navigation.navigate('LegacyAccountList')}
						onlyText={true}
						small={true}
						style={{ marginLeft: 0 }}
						title="Show Legacy Accounts"
					/>
				)}
			</View>
		</SafeAreaScrollViewContainer>
	);
}
