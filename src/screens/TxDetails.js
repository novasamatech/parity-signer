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
import Background from '../components/Background';
import ButtonMainAction from '../components/ButtonMainAction';
import ScreenHeading from '../components/ScreenHeading';
import TxDetailsCard from '../components/TxDetailsCard';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import PayloadDetailsCard from '../components/PayloadDetailsCard';
import { navigateToSignedTx, unlockSeed } from '../util/navigationHelpers';
import { GenericExtrinsicPayload } from '@polkadot/types';
import testIDs from '../../e2e/testIDs';
import fontStyles from '../fontStyles';
import CompatibleCard from '../components/CompatibleCard';
import { getIdentityFromSender } from '../util/identitiesUtils';

export default class TxDetails extends React.PureComponent {
	async onSignTx(scannerStore, accountsStore, sender) {
		if (
			scannerStore.getSender().biometricEnabled &&
			(await scannerStore.signDataBiometric(sender.isLegacy))
		) {
			return navigateToSignedTx(this.props.navigation);
		} else {
			try {
				if (sender.isLegacy) {
					return this.props.navigation.navigate('AccountUnlockAndSign', {
						next: 'SignedTx'
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
				return navigateToSignedTx(this.props.navigation);
			} catch (e) {
				scannerStore.setErrorMsg(e.message);
			}
		}
	}

	render() {
		return (
			<Subscribe to={[ScannerStore, AccountsStore]}>
				{(scannerStore, accountsStore) => {
					const txRequest = scannerStore.getTXRequest();
					const sender = scannerStore.getSender();
					if (txRequest) {
						const tx = scannerStore.getTx();

						return (
							<TxDetailsView
								{...{ ...this.props, ...tx }}
								accountsStore={accountsStore}
								scannerStore={scannerStore}
								sender={sender}
								recipient={scannerStore.getRecipient()}
								// dataToSign={scannerStore.getDataToSign()}
								prehash={scannerStore.getPrehashPayload()}
								onNext={() =>
									this.onSignTx(scannerStore, accountsStore, sender)
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
			accountsStore,
			gas,
			gasPrice,
			prehash,
			sender,
			recipient,
			value,
			onNext
		} = this.props;

		const isEthereum =
			NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM;
		const prefix =
			!isEthereum && SUBSTRATE_NETWORK_LIST[sender.networkKey].prefix;

		return (
			<View style={styles.body}>
				<ScreenHeading
					title="Sign Transaction"
					subtitle="step 1/2 – verify and sign"
				/>
				<ScrollView
					contentContainerStyle={{ paddingBottom: 120 }}
					testID={testIDs.TxDetails.scrollScreen}
				>
					<Text style={[fontStyles.t_big, styles.bodyContent]}>
						{`You are about to confirm sending the following ${
							isEthereum ? 'transaction' : 'extrinsic'
						}`}
					</Text>
					<Background />
					<View style={styles.bodyContent}>
						<CompatibleCard
							account={sender}
							accountsStore={accountsStore}
							titlePrefix={'from: '}
						/>
						{isEthereum ? (
							<View style={{ marginTop: 16 }}>
								<TxDetailsCard
									style={{ marginBottom: 20 }}
									description="You are about to send the following amount"
									value={value}
									gas={gas}
									gasPrice={gasPrice}
								/>
								<Text style={styles.title}>Recipient</Text>
								<CompatibleCard
									account={recipient}
									accountsStore={accountsStore}
								/>
							</View>
						) : (
							<PayloadDetailsCard
								style={{ marginBottom: 20 }}
								payload={prehash}
								prefix={prefix}
							/>
						)}
					</View>
				</ScrollView>
				<ButtonMainAction
					testID={testIDs.TxDetails.signButton}
					title="Sign Transaction"
					onPress={() => onNext()}
				/>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start',
		backgroundColor: colors.bg,
		flex: 1
	},
	bodyContent: {
		marginVertical: 16,
		paddingHorizontal: 20
	},
	marginBottom: {
		marginBottom: 16
	},
	title: {
		...fontStyles.t_regular,
		paddingBottom: 8
	}
});
