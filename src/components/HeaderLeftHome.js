import React from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import colors from '../colors';
import fonts from '../fonts';

export default class HeaderLeftHome extends React.PureComponent {
	render() {
		return (
			<View
				style={{
					alignItems: 'center',
					flexDirection: 'row',
					marginTop: -10,
					paddingLeft: 12
				}}
				accessibilityComponentType="button"
				accessibilityTraits="button"
				testID="header-back"
				delayPressIn={0}
				onPress={() => this.props.onPress && this.props.onPress()}
			>
				<Image source={require('../../icon.png')} style={styles.logo} />
				<Text style={[styles.headerTextLeft, styles.t_bold, { fontSize: 14 }]}>
					parity
				</Text>
				<Text style={[styles.headerTextLeft, { fontSize: 14 }]}>signer</Text>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	headerTextLeft: {
		color: colors.bg_text,
		flex: 1,
		fontFamily: fonts.light,
		fontSize: 16,
		marginRight: 2,
		marginTop: 15
	},
	logo: {
		height: 24,
		width: 24
	},
	t_bold: {
		fontFamily: fonts.semiBold
	}
});
