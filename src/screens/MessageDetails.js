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

import { GenericExtrinsicPayload } from '@polkadot/types';
import { isU8a, u8aToHex } from '@polkadot/util';
import PropTypes from 'prop-types';
import React from 'react';
import { ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import {
	NETWORK_LIST,
	NetworkProtocols,
	SUBSTRATE_NETWORK_LIST
} from '../constants';
import Background from '../components/Background';
import Button from '../components/Button';
import PayloadDetailsCard from '../components/PayloadDetailsCard';
import ScannerStore from '../stores/ScannerStore';
import AccountsStore from '../stores/AccountsStore';
import { navigateToSignedMessage, unlockSeed } from '../util/navigationHelpers';
import fontStyles from '../fontStyles';
import MessageDetailsCard from '../components/MessageDetailsCard';
import { alertMultipart } from '../util/alertUtils';
import CompatibleCard from '../components/CompatibleCard';
import { getIdentityFromSender } from '../util/identitiesUtils';

export default class MessageDetails extends React.PureComponent {
	async onSignMessage(scannerStore, accountsStore, sender) {
		try {
			if (sender.isLegacy) {
				return this.props.navigation.navigate('AccountUnlockAndSign', {
					next: 'SignedMessage'
				});
			}
			const senderIdentity = getIdentityFromSender(
				sender,
				accountsStore.state.identities
			);
			const seed = await unlockSeed(this.props.navigation, senderIdentity);
			await scannerStore.signDataWithSeed(
				seed,
				NETWORK_LIST[sender.networkKey].protocol
			);
			return navigateToSignedMessage(this.props.navigation);
		} catch (e) {
			scannerStore.setErrorMsg(e.message);
		}
	}

	render() {
		return (
			<Subscribe to={[ScannerStore, AccountsStore]}>
				{(scannerStore, accountsStore) => {
					const dataToSign = scannerStore.getDataToSign();
					const message = scannerStore.getMessage();
					const sender = scannerStore.getSender();

					if (dataToSign) {
						return (
							<MessageDetailsView
								{...this.props}
								scannerStore={scannerStore}
								accountsStore={accountsStore}
								sender={sender}
								message={isU8a(message) ? u8aToHex(message) : message}
								dataToSign={
									isU8a(dataToSign) ? u8aToHex(dataToSign) : dataToSign
								}
								prehash={scannerStore.getPrehashPayload()}
								isHash={scannerStore.getIsHash()}
								onNext={() =>
									this.onSignMessage(scannerStore, accountsStore, sender)
								}
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
		prehash: PropTypes.instanceOf(GenericExtrinsicPayload),
		sender: PropTypes.object.isRequired
	};

	render() {
		const {
			accountsStore,
			dataToSign,
			isHash,
			message,
			onNext,
			prehash,
			sender
		} = this.props;

		const isEthereum =
			NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM;
		const prefix =
			!isEthereum && SUBSTRATE_NETWORK_LIST[sender.networkKey].prefix;

		return (
			<ScrollView
				contentContainerStyle={styles.bodyContent}
				style={styles.body}
			>
				<Background />
				<Text style={styles.topTitle}>Sign Message</Text>
				<Text style={styles.title}>From Account</Text>
				<CompatibleCard account={sender} accountsStore={accountsStore} />
				{!isEthereum && prehash && prefix ? (
					<PayloadDetailsCard
						style={{ marginBottom: 20 }}
						description="You are about to confirm sending the following extrinsic. We will sign the hash of the payload as it is oversized."
						payload={prehash}
						prefix={prefix}
					/>
				) : null}
				<MessageDetailsCard
					isHash={isHash}
					message={message}
					data={dataToSign}
				/>
				<Button
					buttonStyles={{ height: 60 }}
					title="Sign Message"
					onPress={() => {
						isHash ? alertMultipart(onNext) : onNext();
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

	title: {
		...fontStyles.h2,
		paddingBottom: 20
	},
	topTitle: {
		...fontStyles.h1,
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
