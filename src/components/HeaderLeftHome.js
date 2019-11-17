import React from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import colors from '../colors';
import fonts from '../fonts';

export default class HeaderLeftHome extends React.PureComponent {
	render() {
		return (
			<View
				style={[
					{
						alignItems: 'center',
						flexDirection: 'row',
						marginTop: -10,
						paddingLeft: 12
					},
					this.props.style
				]}
			>
				<Image source={require('../../icon.png')} style={styles.logo} />
				<Text style={[styles.headerTextLeft, styles.t_bold]}>parity</Text>
				<Text style={styles.headerTextLeft}>signer</Text>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	headerTextLeft: {
		color: colors.bg_text,
		fontFamily: fonts.light,
		fontSize: 14,
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
