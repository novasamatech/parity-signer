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

import PropTypes from 'prop-types';
import React from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';

import colors from '../colors';
import {
	NETWORK_LIST,
	NetworkProtocols,
	SUBSTRATE_NETWORK_LIST
} from '../constants';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import TxDetailsCard from '../components/TxDetailsCard';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import PayloadDetailsCard from '../components/PayloadDetailsCard';
import { navigateToSignedTx, unlockSeed } from '../util/navigationHelpers';
import { GenericExtrinsicPayload } from '@polkadot/types';
import testIDs from '../../e2e/testIDs';
import fontStyles from '../fontStyles';

export default class TxDetails extends React.PureComponent {
	render() {
		return (
			<Subscribe to={[ScannerStore, AccountsStore]}>
				{scannerStore => {
					const txRequest = scannerStore.getTXRequest();
					const sender = scannerStore.getSender();
					if (txRequest) {
						const tx = scannerStore.getTx();

						return (
							<TxDetailsView
								{...{ ...this.props, ...tx }}
								scannerStore={scannerStore}
								sender={sender}
								recipient={scannerStore.getRecipient()}
								// dataToSign={scannerStore.getDataToSign()}
								prehash={scannerStore.getPrehashPayload()}
								onNext={async () => {
									try {
										if (sender.isLegacy) {
											return this.props.navigation.navigate(
												'AccountUnlockAndSign',
												{
													next: 'SignedTx'
												}
											);
										}
										const seed = await unlockSeed(this.props.navigation);
										await scannerStore.signDataWithSeed(
											seed,
											NETWORK_LIST[sender.networkKey].protocol
										);
										return navigateToSignedTx(this.props.navigation);
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

export class TxDetailsView extends React.PureComponent {
	static propTypes = {
		// dataToSign: PropTypes.oneOfType([PropTypes.string, PropTypes.object])
		// .isRequired,
		gas: PropTypes.string,
		gasPrice: PropTypes.string,
		nonce: PropTypes.string,
		onNext: PropTypes.func.isRequired,
		prehash: PropTypes.instanceOf(GenericExtrinsicPayload),
		recipient: PropTypes.object.isRequired,
		sender: PropTypes.object.isRequired,
		value: PropTypes.string
	};

	render() {
		const {
			// dataToSign,
			gas,
			gasPrice,
			prehash,
			recipient,
			sender,
			value,
			onNext
		} = this.props;

		const isEthereum =
			NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM;
		const prefix =
			!isEthereum && SUBSTRATE_NETWORK_LIST[sender.networkKey].prefix;

		return (
			<ScrollView
				style={styles.body}
				contentContainerStyle={{ paddingBottom: 40 }}
				testID={testIDs.TxDetails.scrollScreen}
			>
				<Background />
				<Text style={styles.topTitle}>Sign Transaction</Text>
				<View style={styles.bodyContent}>
					<Text style={styles.title}>From Account</Text>
				</View>
				<AccountCard
					title={sender.name}
					address={sender.address}
					networkKey={sender.networkKey}
				/>
				<Text style={styles.title}>Transaction Details</Text>

				{isEthereum ? (
					<View>
						<TxDetailsCard
							style={{ marginBottom: 20 }}
							description="You are about to send the following amount"
							value={value}
							gas={gas}
							gasPrice={gasPrice}
						/>
						<Text style={styles.title}>Recipient</Text>
						<AccountCard
							title={recipient.name}
							address={recipient.address}
							networkKey={recipient.networkKey || ''}
						/>
					</View>
				) : (
					<PayloadDetailsCard
						style={{ marginBottom: 20 }}
						description="You are about to confirm sending the following extrinsic"
						payload={prehash}
						prefix={prefix}
					/>
				)}

				<Button
					buttonStyles={{ height: 60 }}
					testID={testIDs.TxDetails.signButton}
					title="Sign Transaction"
					onPress={() => onNext()}
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
		alignContent: 'flex-start',
		backgroundColor: colors.bg,
		flex: 1,
		paddingHorizontal: 20,
		paddingTop: 24
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
