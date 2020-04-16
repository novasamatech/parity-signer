import React from 'react';
import { Text, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import styles from 'modules/main/styles';
import fontStyles from 'styles/fontStyles';

export default function NoCurrentIdentity(): React.ReactElement {
	return (
		<SafeAreaScrollViewContainer contentContainerStyle={styles.scrollContent}>
			<View style={styles.onboardingWrapper}>
				<Text style={fontStyles.quote}>
					Select one of your identity to get started.
				</Text>
			</View>
		</SafeAreaScrollViewContainer>
	);
}
