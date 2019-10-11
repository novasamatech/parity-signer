import React from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import colors from '../colors';
import fonts from '../fonts';

export default class HeaderLeftHome extends React.PureComponent {
	render() {
		return (
			<View
				style={{ alignItems: 'center', flexDirection: 'row', paddingLeft: 14 }}
				accessibilityComponentType="button"
				accessibilityTraits="button"
				testID="header-back"
				delayPressIn={0}
				onPress={() => this.props.onPress && this.props.onPress()}
			>
				<Image source={require('../../icon.png')} style={styles.logo} />
				<Text style={styles.headerTextLeft}>parity</Text>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	headerStyle: {
		alignItems: 'center',
		backgroundColor: colors.bg,
		borderBottomColor: colors.bg_text_sec,
		borderBottomWidth: 0.5,
		flexDirection: 'row',
		height: 60
	},
	headerTextLeft: {
		color: colors.bg_text,
		flex: 1,
		fontFamily: fonts.regular,
		fontSize: 25,
		paddingLeft: 4
	},
	logo: {
		height: 42,
		width: 42
	}
});
