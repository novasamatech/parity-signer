import React, { ReactNode } from 'react';
import {
	ScrollView,
	ScrollViewProps,
	StyleSheet,
	ViewProps
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import containerStyles from 'styles/containerStyles';

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
		style={StyleSheet.flatten([containerStyles.background, props.style])}
		children={props.children}
	/>
);

export const SafeAreaScrollViewContainer = (
	props: SafeAreaScrollViewProps
): React.ReactElement => (
	<SafeAreaView style={containerStyles.background}>
		<ScrollView
			{...props}
			bounces={false}
			style={StyleSheet.flatten([containerStyles.background, props.style])}
			children={props.children}
		/>
	</SafeAreaView>
);
