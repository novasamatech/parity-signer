// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import React from 'react';
import { StyleSheet, Text, View, ViewStyle } from 'react-native';

import colors from 'styles/colors';
import fonts from 'styles/fonts';

const WEI_IN_ETH = 1000000000000000000;

interface Props {
	value: string;
	description: string;
	gas: string;
	gasPrice: string;
	style: ViewStyle;
}
export default class TxDetailsCard extends React.PureComponent<Props> {
	render(): React.ReactNode {
		const { value, description, gas, gasPrice, style } = this.props;

		return (
			<View style={[styles.body, style]}>
				<Text style={styles.titleText}>{description}</Text>
				<Amount
					style={{ marginTop: 10 }}
					value={value}
					gas={gas}
					gasPrice={gasPrice}
				/>
			</View>
		);
	}
}

interface AmountProps {
	value: string;
	gas: string;
	gasPrice: string;
	style: ViewStyle;
}

function Amount({
	style,
	value,
	gas,
	gasPrice
}: AmountProps): React.ReactElement<AmountProps> {
	const fee = (parseInt(gas, 10) * parseInt(gasPrice, 10)) / WEI_IN_ETH;
	return (
		<View style={[{ alignItems: 'center', justifyContent: 'center' }, style]}>
			<View>
				<View
					style={{ backgroundColor: colors.bg, padding: 5, paddingVertical: 2 }}
				>
					<Text
						style={{ fontSize: 20, fontWeight: '800', textAlign: 'center' }}
					>
						<Text style={{ color: colors.bg_text }}>{value}</Text>
						<Text style={{ color: colors.bg_text_sec }}> ETH</Text>
					</Text>
				</View>
				<View style={{ backgroundColor: colors.bg_text_sec, padding: 5 }}>
					<Text
						style={{
							color: colors.bg_text,
							fontSize: 12,
							fontWeight: '800',
							textAlign: 'center'
						}}
					>
						fee: {fee} ETH
					</Text>
				</View>
			</View>
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.card_bg,
		flexDirection: 'column',
		padding: 20,
		paddingTop: 10
	},
	content: {},
	footer: {
		backgroundColor: '#977CF6',
		flexDirection: 'row-reverse',
		padding: 5
	},
	footerText: {
		color: colors.card_bg,
		fontFamily: fonts.bold
	},
	icon: {
		height: 47,
		width: 47
	},
	secondaryText: {
		color: colors.card_bg_text_sec,
		fontFamily: fonts.semiBold,
		fontSize: 12,
		textAlign: 'center'
	},
	titleText: {
		color: colors.card_bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 14,
		textAlign: 'center'
	}
});
