// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import { useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import React from 'react';
import { Text, View, StyleSheet } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import fontStyles from 'styles/fontStyles';
import { RootStackParamList } from 'types/routes';

export default function OnBoardingView(): React.ReactElement {
	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();

	function TextButton({
		text,
		isRecover
	}: {
		text: string;
		isRecover: boolean;
	}): React.ReactElement {
		return (
			<Text
				style={[fontStyles.quote, { textDecorationLine: 'underline' }]}
				testID={
					isRecover ? testIDs.Main.recoverButton : testIDs.Main.createButton
				}
				onPress={(): void => navigation.navigate('RootSeedNew', { isRecover })}
			>
				{text}
			</Text>
		);
	}

	return (
		<SafeAreaScrollViewContainer
			testID={testIDs.Main.noAccountScreen}
			contentContainerStyle={styles.scrollContent}
		>
			<View style={styles.onboardingWrapper}>
				<TextButton text="Create" isRecover={false} />
				<Text style={fontStyles.quote}> or </Text>
				<TextButton text="recover" isRecover={true} />
				<Text style={fontStyles.quote}>your identity to get started.</Text>
			</View>
		</SafeAreaScrollViewContainer>
	);
}

const styles = StyleSheet.create({
	onboardingWrapper: {
		alignItems: 'center',
		flexDirection: 'row',
		flexWrap: 'wrap'
	},
	scrollContent: {
		flex: 1,
		justifyContent: 'center',
		padding: 16,
		paddingBottom: 100
	}
});
