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
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';
import { GenericExtrinsicPayload } from '@polkadot/types';

import { NETWORK_LIST } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { FoundAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { NavigationAccountScannerProps, NavigationProps } from 'types/props';
import colors from 'styles/colors';
import Background from 'components/Background';
import ButtonMainAction from 'components/ButtonMainAction';
import ScreenHeading from 'components/ScreenHeading';
import TxDetailsCard from 'components/TxDetailsCard';
import AccountsStore from 'stores/AccountsStore';
import ScannerStore from 'stores/ScannerStore';
import PayloadDetailsCard from 'components/PayloadDetailsCard';
import { navigateToSignedTx, unlockSeedPhrase } from 'utils/navigationHelpers';
import fontStyles from 'styles/fontStyles';
import CompatibleCard from 'components/CompatibleCard';
import { getIdentityFromSender } from 'utils/identitiesUtils';
import { Transaction } from 'utils/transaction';

export default class TxDetails extends React.PureComponent<
	NavigationProps<{}>
> {
	async onSignTx(
		scannerStore: ScannerStore,
		accountsStore: AccountsStore,
		sender: FoundAccount
	): Promise<void> {
		try {
			if (sender.isLegacy) {
				this.props.navigation.navigate('AccountUnlockAndSign', {
					next: 'SignedTx'
				});
				return;
			}
			const senderIdentity = getIdentityFromSender(
				sender,
				accountsStore.state.identities
			);
			const seedPhrase = await unlockSeedPhrase(
				this.props.navigation,
				senderIdentity
			);
			await scannerStore.signDataWithSeedPhrase(
				seedPhrase,
				NETWORK_LIST[sender.networkKey].protocol
			);
			return navigateToSignedTx(this.props.navigation);
		} catch (e) {
			scannerStore.setErrorMsg(e.message);
		}
	}

	render(): React.ReactElement {
		return (
			<Subscribe to={[ScannerStore, AccountsStore]}>
				{(
					scannerStore: ScannerStore,
					accountsStore: AccountsStore
				): React.ReactNode => {
					const txRequest = scannerStore.getTXRequest();
					const sender = scannerStore.getSender()!;
					if (txRequest) {
						const tx = scannerStore.getTx();
						return (
							<TxDetailsView
								{...{ ...this.props, ...(tx as Transaction) }}
								accounts={accountsStore}
								scannerStore={scannerStore}
								sender={sender!}
								recipient={scannerStore.getRecipient()!}
								prehash={scannerStore.getPrehashPayload()!}
								onNext={(): Promise<void> =>
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

interface ViewProps extends NavigationAccountScannerProps<{}> {
	gas: string;
	gasPrice: string;
	nonce: string;
	onNext: () => void;
	prehash: GenericExtrinsicPayload;
	recipient: FoundAccount;
	sender: FoundAccount;
	value: string;
}

export class TxDetailsView extends React.PureComponent<ViewProps> {
	render(): React.ReactElement {
		const {
			accounts,
			gas,
			gasPrice,
			prehash,
			sender,
			recipient,
			value,
			onNext
		} = this.props;

		const senderNetworkParams = NETWORK_LIST[sender.networkKey];
		const isEthereum = isEthereumNetworkParams(senderNetworkParams);

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
							accountsStore={accounts}
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
								<CompatibleCard account={recipient} accountsStore={accounts} />
							</View>
						) : (
							<PayloadDetailsCard
								style={{ marginBottom: 20 }}
								payload={prehash}
								networkKey={sender.networkKey}
							/>
						)}
					</View>
				</ScrollView>
				<ButtonMainAction
					testID={testIDs.TxDetails.signButton}
					title="Sign Transaction"
					onPress={(): any => onNext()}
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
