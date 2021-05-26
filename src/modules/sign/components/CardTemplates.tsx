import React, { ReactElement } from 'react';
import { View, Text, StyleSheet } from 'react-native';

import { PayloadCardContent } from 'types/payload';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import Identicon from '@polkadot/reactnative-identicon';

type CardProps = {
	payload?: PayloadCardContent;
};

export function DefaultCard({
	payload
}: CardProps): ReactElement {
	return (
		<View>
			<Text style={fontStyles.t_regular}>{JSON.stringify(payload)}</Text>
		</View>
	);
}

export function BlockHashCard({
	payload
}: CardProps): ReactElement {
	return (
		<View style={styles.content}>
			<Text style={styles.titleText} > Block </Text> 
			<Text style={styles.secondaryText}>{payload}</Text>
		</View>
	);
}

export function CallCard({
	payload
}: CardProps): ReactElement {
	return (
		<View>
			<Text
				style={styles.titleText}
			>
				{payload.method}
				<Text style={styles.secondaryText}> from </Text>
				{payload.pallet}
			</Text>

		</View>
	);
}

export function EraNonceTipCard({
	payload
}: CardProps): ReactElement {
	if (payload.era === 'Mortal') {
		return (
			<View style={styles.content}>
				<View>
					<Text style={styles.secondaryText}> Period </Text>
					<Text style={styles.titleText}>{payload.period}</Text>
				</View>
				<View>
					<Text style={styles.secondaryText}> Phase </Text>
					<Text style={styles.titleText}>{payload.phase}</Text>
				</View>
				<View>
					<Text style={styles.secondaryText}> Nonce </Text>
					<Text style={styles.titleText}>{payload.nonce}</Text>
				</View>
				<View>
					<Text style={styles.secondaryText}> Tip </Text>
					<Text style={styles.titleText}>{payload.tip}</Text>
				</View>
			</View>
		);
	} else {
		return(
			<View>
				<Text
					style={styles.titleText}
				>
					Immortal Era
					<Text style={styles.secondaryText}> Nonce: </Text>
					{payload.nonce}
					<Text style={styles.secondaryText}> Tip: </Text>
					{payload.tip}
				</Text>
			</View>
		);
	}
}

export function IdCard({
	payload
}: CardProps): ReactElement {
	return (
		<View style={styles.content}>
			<Identicon 
				value={payload}
				size={40}
			/>
			<View style={{paddingHorizontal: 10}}>
				<Text style={fontStyles.t_codeS}>{payload}</Text>
			</View>
		</View>
	);
}

export function TxSpecCard({
	payload
}: CardProps): ReactElement {
	return (
		<View>
			<Text
				style={styles.titleText}
			>
				<Text style={styles.secondaryText}> Network </Text>
				{payload.chain}
				<Text style={styles.secondaryText}> Spec version: </Text>
				{payload.version}
				<Text style={styles.secondaryText}> tx version: </Text>
				{payload.tx_version}
			</Text>

		</View>
	);
}

export function VariableNameCard({
	payload
}: CardProps): ReactElement {
	return (
		<View>
			<Text style={fontStyles.t_regular}>{payload}</Text>
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		borderBottomWidth: 1,
		borderColor: colors.background.app,
		borderTopWidth: 1
	},
	content: {
		alignItems: 'center',
		backgroundColor: colors.background.card,
		flexDirection: 'row',
		paddingLeft: 16,
		paddingVertical: 8
	},
	desc: {
		flex: 1,
		paddingHorizontal: 16
	},
	footer: {
		height: 80,
		marginLeft: 8,
		width: 4
	},
	iconLock: {
		marginLeft: 4,
		...fontStyles.h2
	},
	identicon: {
		height: 40,
		width: 40
	},
	row: {
		alignItems: 'flex-end',
		flexDirection: 'row'
	},
	secondaryText: {
		...fontStyles.t_codeS,
		color: colors.signal.main,
		paddingHorizontal: 8,
		textAlign: 'left'
	},
	titleText: {
		...fontStyles.t_codeS,
		color: colors.text.main,
		paddingHorizontal: 16
	}

});
