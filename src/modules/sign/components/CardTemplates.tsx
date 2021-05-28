import React, { ReactElement } from 'react';
import { View, Text, StyleSheet, ActivityIndicator } from 'react-native';
import Identicon from '@polkadot/reactnative-identicon';

import { PayloadCardContent } from 'types/payloads';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';

type CardProps = {
	payload?: PayloadCardContent;
};

export function LoadingCard(): ReactElement {
	return (
		<View>
			<ActivityIndicator
				animating={true}
				color="red"
				size="large"
				style={styles.indicator}
			/>
			<Text style={styles.titleText}>Parsing transaction, please wait</Text>
		</View>
	);
}

export function ErrorCard({ payload }: CardProps): ReactElement {
	return (
		<View>
			<Text style={styles.titleText}>ERROR! {payload}</Text>
		</View>
	);
}

export function DefaultCard({ payload }: CardProps): ReactElement {
	return (
		<View>
			<Text style={fontStyles.t_regular}>{JSON.stringify(payload)}</Text>
		</View>
	);
}

export function AuthorCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.content}>
			<Identicon value={payload.base58} size={40} />
			<View style={{ paddingHorizontal: 10 }}>
				<Text style={styles.titleText}>From:</Text>
				<Text style={fontStyles.t_codeS}>{payload.base58}</Text>
			</View>
		</View>
	);
}

export function BlockHashCard({ payload }: CardProps): ReactElement {
	return (
		<View>
			<Text style={styles.titleText}> Block </Text>
			<Text style={styles.secondaryText}>{payload}</Text>
		</View>
	);
}

export function CallCard({ payload }: CardProps): ReactElement {
	return (
		<View>
			<Text style={styles.titleText}>
				{payload.method}
				<Text style={styles.secondaryText}> from </Text>
				{payload.pallet}
			</Text>
		</View>
	);
}

export function EraNonceTipCard({ payload }: CardProps): ReactElement {
	if (payload.era === 'Mortal') {
		return (
			<View style={styles.content}>
				<View>
					<Text style={styles.label}> Period </Text>
					<Text style={styles.titleText}>{payload.period}</Text>
				</View>
				<View>
					<Text style={styles.label}> Phase </Text>
					<Text style={styles.titleText}>{payload.phase}</Text>
				</View>
				<View>
					<Text style={styles.label}> Nonce </Text>
					<Text style={styles.titleText}>{payload.nonce}</Text>
				</View>
				<View>
					<Text style={styles.label}> Tip </Text>
					<Text style={styles.titleText}>{payload.tip}</Text>
				</View>
			</View>
		);
	} else {
		return (
			<View>
				<Text style={styles.titleText}>
					Immortal Era
					<Text style={styles.label}> Nonce: </Text>
					{payload.nonce}
					<Text style={styles.label}> Tip: </Text>
					{payload.tip}
				</Text>
			</View>
		);
	}
}

export function IdCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.content}>
			<Identicon value={payload} size={40} />
			<View style={{ paddingHorizontal: 10 }}>
				<Text style={fontStyles.t_codeS}>{payload.substr(0, 12)}</Text>
				<Text style={fontStyles.t_codeS}>{payload.substr(12, 12)}</Text>
				<Text style={fontStyles.t_codeS}>{payload.substr(24, 12)}</Text>
				<Text style={fontStyles.t_codeS}>{payload.substr(36, 12)}</Text>
			</View>
		</View>
	);
}

export function TxSpecCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.content}>
			<View>
				<Text style={styles.label}> Network </Text>
				<Text style={styles.titleText}> {payload.chain} </Text>
			</View>
			<View>
				<Text style={styles.label}> Spec version </Text>
				<Text style={styles.titleText}> {payload.version} </Text>
			</View>
			<View>
				<Text style={styles.label}> tx version </Text>
				<Text style={styles.titleText}> {payload.tx_version} </Text>
			</View>
		</View>
	);
}

export function VariableNameCard({ payload }: CardProps): ReactElement {
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
	indicator: {
		margin: 15
	},
	label: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.main,
		color: colors.background.app,
		marginBottom: 10,
		paddingHorizontal: 8,
		textAlign: 'center'
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
