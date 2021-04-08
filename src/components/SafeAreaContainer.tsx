import React, { ReactNode } from 'react';
import { ScrollView } from 'react-native';

export const SafeAreaScrollViewContainer = (props: {
	children?: ReactNode | ReactNode[];
}): React.ReactElement => (
	<ScrollView
		{...props}
		bounces={false}
		style={props.style}
		children={props.children}
	/>
);
