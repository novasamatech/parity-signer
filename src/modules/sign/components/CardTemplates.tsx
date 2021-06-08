import React, { ReactElement } from 'react';
import { View, Text, StyleSheet, ActivityIndicator } from 'react-native';
import Identicon from '@polkadot/reactnative-identicon';
import AntIcon from 'react-native-vector-icons/AntDesign';

import { PayloadCardContent } from 'types/payloads';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import AccountPrefixedTitle from 'components/AccountPrefixedTitle';

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
		<View style={styles.errorCard}>
			<Text style={styles.errorTitleText}>ERROR! </Text>
			<Text style={styles.errorSecondaryText}>{payload}</Text>
		</View>
	);
}

export function WarningCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.warningCard}>
			<Text style={styles.warningTitleText}>Warning:</Text>
			<Text style={styles.warningSecondaryText}>{payload}</Text>
		</View>
	);
}

export function DefaultCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.card}>
			<Text style={fontStyles.t_regular}>{JSON.stringify(payload)}</Text>
		</View>
	);
}

export function AuthorCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.content}>
			<Identicon value={payload.base58} size={40} />
			<View style={{ paddingHorizontal: 10 }}>
				<View style={styles.row}>
					<Text style={styles.titleText}>From:</Text>
					<Text style={styles.secondaryText}>{payload.name} </Text>
				</View>
				<View style={styles.row}>
					<AntIcon
						name="user"
						size={fontStyles.i_small.fontSize}
						color={colors.signal.main}
					/>
					<Text style={styles.secondaryText}>{payload.derivation_path} </Text>
					{payload.has_password === 'true' ? (
						<AntIcon name="lock" style={styles.iconLock} />
					) : (
						<View />
					)}
				</View>
				<Text
					style={styles.authorAddressText}
					numberOfLines={1}
					adjustFontSizeToFit
				>
					{payload.base58}
				</Text>
			</View>
		</View>
	);
}

export function BalanceCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.card}>
			<Text style={styles.titleText}>{payload.amount} </Text>
			<Text style={styles.titleText}>{payload.units}</Text>
		</View>
	);
}

export function BlockHashCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.cardExtrinsic}>
			<Text style={styles.titleText}> Block </Text>
			<Text style={styles.secondaryText}>{payload}</Text>
		</View>
	);
}

export function CallCard({ payload }: CardProps): ReactElement {
	return (
		<View>
			<Text style={styles.label}>
				{payload.pallet}::{payload.method}
			</Text>
		</View>
	);
}

export function EraNonceCard({ payload }: CardProps): ReactElement {
	if (payload.era === 'Mortal') {
		return (
			<View style={styles.cardExtrinsic}>
				<View style={styles.cardExtrinsicColumn}>
					<Text style={styles.label}> Period </Text>
					<Text style={styles.titleText}>{payload.period}</Text>
				</View>
				<View style={styles.cardExtrinsicColumn}>
					<Text style={styles.label}> Phase </Text>
					<Text style={styles.titleText}>{payload.phase}</Text>
				</View>
				<View style={styles.cardExtrinsicColumn}>
					<Text style={styles.label}> Nonce </Text>
					<Text style={styles.titleText}>{payload.nonce}</Text>
				</View>
			</View>
		);
	} else {
		return (
			<View style={styles.cardExtrinsic}>
				<View style={styles.cardExtrinsicColumn}>
					<Text style={styles.label}> Immortal Era </Text>
				</View>
				<View style={styles.cardExtrinsicColumn}>
					<Text style={styles.label}> Nonce </Text>
					<Text style={styles.titleText}>{payload.nonce}</Text>
				</View>
			</View>
		);
	}
}

export function IdCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.card}>
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

export function TipCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.cardExtrinsic}>
			<Text style={styles.label}> Tip </Text>
			<Text style={styles.titleText}> {payload.amount}</Text>
			<Text style={styles.titleText}> {payload.units}</Text>
		</View>
	);
}

export function TxSpecCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.cardExtrinsic}>
			<View style={styles.cardExtrinsicColumn}>
				<Text style={styles.label}> Network </Text>
				<Text style={styles.titleText}> {payload.network} </Text>
			</View>
			<View style={styles.cardExtrinsicColumn}>
				<Text style={styles.label}> Spec version </Text>
				<Text style={styles.titleText}> {payload.version} </Text>
			</View>
			<View style={styles.cardExtrinsicColumn}>
				<Text style={styles.label}> tx version </Text>
				<Text style={styles.titleText}> {payload.tx_version} </Text>
			</View>
		</View>
	);
}

export function VariableNameCard({ payload }: CardProps): ReactElement {
	return (
		<View style={styles.card}>
			<Text style={fontStyles.t_regular}>{payload}</Text>
		</View>
	);
}

const styles = StyleSheet.create({
	authorAddressText: {
		...fontStyles.t_codeS,
		color: colors.text.faded,
		fontSize: 10
	},
	body: {
		borderBottomWidth: 1,
		borderColor: colors.background.app,
		borderTopWidth: 1
	},
	card: {
		backgroundColor: colors.background.card,
		borderBottomColor: colors.signal.main,
		borderBottomWidth: 1,
		borderLeftColor: colors.signal.main,
		borderLeftWidth: 1,
		flexDirection: 'row',
		padding: 2
	},
	cardExtrinsic: {
		backgroundColor: colors.background.card,
		flex: 1,
		flexDirection: 'row',
		padding: 10,
		textAlign: 'center'
	},
	cardExtrinsicColumn: {
		alignItems: 'center',
		flex: 1
	},
	content: {
		alignItems: 'center',
		backgroundColor: colors.background.card,
		flexDirection: 'row',
		paddingLeft: 8,
		paddingVertical: 8
	},
	desc: {
		flex: 1,
		paddingHorizontal: 16
	},
	errorCard: {
		alignItems: 'center',
		backgroundColor: colors.signal.error,
		flexDirection: 'row',
		paddingVertical: 20
	},
	errorSecondaryText: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.error,
		color: colors.text.warning,
		flex: 1,
		textAlign: 'center'
	},
	errorTitleText: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.error,
		color: colors.text.warning,
		flex: 0.2,
		textAlign: 'center'
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
		paddingHorizontal: 10,
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
		color: colors.text.main
	},
	warningCard: {
		flexDirection: 'row'
	},
	warningSecondaryText: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.warning,
		color: colors.text.warning,
		flex: 1,
		textAlign: 'left'
	},
	warningTitleText: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.warning,
		color: colors.text.warning,
		flex: 0.2
	}
});
