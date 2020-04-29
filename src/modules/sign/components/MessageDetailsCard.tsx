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

import fontStyles from 'styles/fontStyles';
import { hexToAscii, isAscii } from 'utils/strings';
import colors from 'styles/colors';

export default function MessageDetailsCard({
	isHash,
	message,
	data,
	style
}: {
	isHash: boolean;
	message: string;
	data: string;
	style?: ViewStyle;
}): React.ReactElement {
	return (
		<View style={[styles.messageContainer, style]}>
			<Text style={fontStyles.t_label}>{isHash ? 'Hash' : 'Message'}</Text>
			{isHash ? (
				<Text style={styles.hashText}>{message}</Text>
			) : (
				<Text style={styles.messageText}>
					{isAscii(message) ? hexToAscii(message) : data}
				</Text>
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	hashText: {
		...fontStyles.t_codeS,
		backgroundColor: colors.label_text,
		color: colors.bg,
		marginBottom: 20,
		paddingHorizontal: 8
	},
	messageContainer: {
		marginTop: 16
	},
	messageText: {
		...fontStyles.t_code,
		color: colors.label_text,
		lineHeight: 26,
		marginBottom: 20,
		minHeight: 120,
		padding: 10
	}
});
