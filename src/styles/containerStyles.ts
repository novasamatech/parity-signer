import { StyleSheet } from 'react-native';

import colors from 'styles/colors';

export const headerHeight = 40;
export const horizontalPadding = 16;

export default StyleSheet.create({
	background: {
		backgroundColor: colors.background.app,
		flex: 1,
		flexDirection: 'column',
		overflow: 'hidden'
	}
});
