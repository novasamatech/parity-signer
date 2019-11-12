import React from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import colors from '../colors';
import fonts from '../fonts';

export default class HeaderLeftHome extends React.PureComponent {
	render() {
		return (
			<View
				style={{ alignItems: 'center', flexDirection: 'row', paddingLeft: 16 }}
				accessibilityComponentType="button"
				accessibilityTraits="button"
				testID="header-back"
				delayPressIn={0}
				onPress={() => this.props.onPress && this.props.onPress()}
			>
				<Image source={require('../../icon.png')} style={styles.logo} />
				<Text style={[styles.headerTextLeft, styles.t_bold, { fontSize: 13 }]}>
					parity
				</Text>
				<Text style={[styles.headerTextLeft, { fontSize: 13 }]}>signer</Text>
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
		marginRight: 3,
		marginTop: 15
	},
	logo: {
		height: 21,
		width: 21
	},
	t_bold: {
		fontFamily: fonts.semiBold
	}
});
