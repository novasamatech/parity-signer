import { useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import React from 'react';
import { Text, View } from 'react-native';

import Button from 'components/Button';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import styles from 'modules/main/styles';
import fontStyles from 'styles/fontStyles';
import { RootStackParamList } from 'types/routes';

export default function OnBoardingView({
	hasLegacyAccount
}: {
	hasLegacyAccount: boolean;
}): React.ReactElement {
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
				onPress={(): void => navigation.navigate('IdentityNew', { isRecover })}
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
				{hasLegacyAccount && (
					<Button
						title="Show Legacy Accounts"
						onPress={(): void => navigation.navigate('LegacyAccountList')}
						small={true}
						onlyText={true}
						style={{ marginLeft: 0 }}
					/>
				)}
			</View>
		</SafeAreaScrollViewContainer>
	);
}
