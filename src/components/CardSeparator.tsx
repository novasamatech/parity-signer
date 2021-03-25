import React, { ReactElement } from 'react';
//import { StyleSheet, Text, View, ViewStyle } from 'react-native';

import Separator from 'components/Separator';
//import colors from 'styles/colors';

export const CardSeparator = (): ReactElement => (
	<Separator
		shadow={true}
		style={{
			backgroundColor: 'transparent',
			height: 0,
			marginVertical: 0
		}}
	/>
);
