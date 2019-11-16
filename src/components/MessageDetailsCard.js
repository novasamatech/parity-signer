import React from 'react';
import { StyleSheet, Text } from 'react-native';

import fontStyles from '../fontStyles';
import { hexToAscii, isAscii } from '../util/strings';
import colors from '../colors';

export default function MessageDetailsCard({ isHash, message, data }) {
	return (
		<>
			<Text style={fontStyles.t_label}>{isHash ? 'Hash' : 'Message'}</Text>
			{isHash ? (
				<Text style={styles.hashText}>{message}</Text>
			) : (
				<Text style={styles.messageText}>
					{isAscii(message) ? hexToAscii(message) : data}
				</Text>
			)}
		</>
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
	messageText: {
		...fontStyles.t_code,
		color: colors.label_text,
		lineHeight: 26,
		marginBottom: 20,
		minHeight: 120,
		padding: 10
	}
});
