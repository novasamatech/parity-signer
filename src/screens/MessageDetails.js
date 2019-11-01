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

import { isU8a, u8aToHex } from '@polkadot/util';
import PropTypes from 'prop-types';
import React from 'react';
import { Alert, ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fonts from '../fonts';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import { hexToAscii, isAscii } from '../util/message';

export default class MessageDetails extends React.PureComponent {
	render() {
		return (
			<Subscribe to={[ScannerStore, AccountsStore]}>
				{(scannerStore, accounts) => {
					const dataToSign = scannerStore.getDataToSign();
					const message = scannerStore.getMessage();

					if (dataToSign) {
						return (
							<MessageDetailsView
								{...this.props}
								scannerStore={scannerStore}
								sender={scannerStore.getSender()}
								message={isU8a(message) ? u8aToHex(message) : message}
								dataToSign={
									isU8a(dataToSign) ? u8aToHex(dataToSign) : dataToSign
								}
								isHash={scannerStore.getIsHash()}
								onNext={async () => {
									try {
										this.props.navigation.navigate('AccountUnlockAndSign', {
											next: 'SignedMessage'
										});
									} catch (e) {
										scannerStore.setErrorMsg(e.message);
									}
								}}
							/>
						);
					} else {
						return null;
					}
				}}
			</Subscribe>
		);
	}
}

export class MessageDetailsView extends React.PureComponent {
	static propTypes = {
		dataToSign: PropTypes.string.isRequired,
		isHash: PropTypes.bool,
		message: PropTypes.string.isRequired,
		onNext: PropTypes.func.isRequired,
		sender: PropTypes.object.isRequired
	};

	render() {
		const { dataToSign, isHash, message, onNext, sender } = this.props;

		return (
			<ScrollView
				contentContainerStyle={styles.bodyContent}
				style={styles.body}
			>
				<Background />
				<Text style={styles.topTitle}>SIGN MESSAGE</Text>
				<Text style={styles.title}>FROM ACCOUNT</Text>
				<AccountCard
					title={sender.name}
					address={sender.address}
					networkKey={sender.networkKey}
				/>
				<Text style={styles.title}>MESSAGE</Text>
				<Text style={styles.message}>
					{isHash
						? message
						: isAscii(message)
						? hexToAscii(message)
						: dataToSign}
				</Text>
				<Button
					buttonStyles={{ height: 60 }}
					title="Sign Message"
					onPress={() => {
						isHash
							? Alert.alert(
									'Warning',
									'The payload of the transaction you are signing is too big to be decoded. Not seeing what you are signing is inherently unsafe. If possible, contact the developer of the application generating the transaction to ask for multipart support.',
									[
										{
											onPress: () => onNext(),
											text: 'I take the risk'
										},
										{
											style: 'cancel',
											text: 'Cancel'
										}
									]
							  )
							: onNext();
					}}
				/>
			</ScrollView>
		);
	}
}

const styles = StyleSheet.create({
	actionButtonContainer: {
		flex: 1
	},
	actionsContainer: {
		flex: 1,
		flexDirection: 'row'
	},
	address: {
		flex: 1
	},
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		overflow: 'hidden',
		padding: 20
	},
	bodyContent: {
		paddingBottom: 40
	},
	changePinText: {
		color: 'green',
		textAlign: 'left'
	},
	deleteText: {
		textAlign: 'right'
	},
	message: {
		backgroundColor: colors.card_bg,
		fontFamily: fonts.regular,
		fontSize: 20,
		lineHeight: 26,
		marginBottom: 20,
		minHeight: 120,
		padding: 10
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	},
	topTitle: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	},
	transactionDetails: {
		backgroundColor: colors.card_bg,
		flex: 1
	},
	wrapper: {
		borderRadius: 5
	}
});
