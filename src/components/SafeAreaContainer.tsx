import React, { ReactNode } from 'react';
import {
	ScrollView,
	ScrollViewProps,
	StyleSheet,
	ViewProps
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import { colors } from 'styles';

interface SafeAreaContainerProps extends ViewProps {
	children?: ReactNode | ReactNode[];
}

interface SafeAreaScrollViewProps extends ScrollViewProps {
	children?: ReactNode | ReactNode[];
}

export const SafeAreaViewContainer = (
	props: SafeAreaContainerProps
): React.ReactElement => (
	<SafeAreaView
		{...props}
		style={StyleSheet.flatten([styles.background, props.style])}
		children={props.children}
	/>
);

export const SafeAreaScrollViewContainer = (
	props: SafeAreaScrollViewProps
): React.ReactElement => (
	<SafeAreaView style={styles.background}>
		<ScrollView
			{...props}
			bounces={false}
			style={StyleSheet.flatten([styles.background, props.style])}
			children={props.children}
		/>
	</SafeAreaView>
);

const styles = StyleSheet.create({
	background: {
		backgroundColor: colors.background.app,
		flex: 1,
		flexDirection: 'column',
		overflow: 'hidden',
		paddingBottom: 0
	}
});
