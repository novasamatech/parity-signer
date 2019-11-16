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

'use strict';

import React, { useEffect } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import colors from '../colors';
import fonts from '../fonts';
import QrView from '../components/QrView';
import { hexToAscii, isAscii } from '../util/strings';
import { withScannerStore } from '../util/HOC';

export function SignedMessage({ scanner }) {
	const data = scanner.getSignedTxData();
	const isHash = scanner.getIsHash();
	const message = scanner.getMessage();

	useEffect(
		() =>
			function() {
				scanner.cleanup();
			},
		[scanner]
	);

	return (
		<ScrollView style={styles.body}>
			<Text style={styles.topTitle}>SCAN SIGNATURE</Text>
			<View style={styles.qr}>
				<QrView data={data} />
			</View>
			<Text style={styles.title}>{!isHash && 'MESSAGE'}</Text>
			{isHash ? (
				<Text style={styles.title}>HASH</Text>
			) : (
				<Text style={styles.title}>MESSAGE</Text>
			)}
			<Text style={styles.message}>
				{isHash ? message : isAscii(message) ? hexToAscii(message) : data}
			</Text>
		</ScrollView>
	);
}

export default withScannerStore(SignedMessage);

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		overflow: 'hidden'
	},
	message: {
		backgroundColor: colors.card_bg,
		fontFamily: fonts.regular,
		fontSize: 20,
		lineHeight: 26,
		marginBottom: 20,
		marginHorizontal: 20,
		minHeight: 120,
		padding: 10
	},
	qr: {
		marginBottom: 20
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		marginHorizontal: 20,
		paddingBottom: 20
	},
	topTitle: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
