// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

// This screen shows Tx-type payload details and asks for signing confirmation

import React, { useContext, useEffect, useRef } from 'react';
import { Text, View } from 'react-native';

import { usePayloadDetails } from 'modules/sign/hooks';
import PayloadDetailsCard from 'modules/sign/components/PayloadDetailsCard';
import strings from 'modules/sign/strings';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { AccountsContext } from 'stores/AccountsContext';
import { NetworksContext } from 'stores/NetworkContext';
import { ScannerContext } from 'stores/ScannerContext';
import { FoundAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkTypes';
import { NavigationProps, NavigationScannerProps } from 'types/props';
import TxDetailsCard from 'modules/sign/components/TxDetailsCard';
import CompatibleCard from 'components/CompatibleCard';
import { Transaction } from 'utils/transaction';
import styles from 'modules/sign/styles';
import Separator from 'components/Separator';
import Button from 'components/Button';

function DetailsTx({
	route,
	navigation
}: NavigationProps<'DetailsTx'>): React.ReactElement {
	const scannerStore = useContext(ScannerContext);
	const { recipient, sender } = scannerStore.state;
	const cleanup = useRef(scannerStore.cleanup);

	useEffect(() => cleanup.current, [cleanup]);

	if (sender === null || recipient === null) return <View />;
	return (
		<UnsignedTxView
			sender={sender}
			recipient={recipient}
			scannerStore={scannerStore}
			route={route}
			navigation={navigation}
		/>
	);
}

interface Props extends NavigationScannerProps<'DetailsTx'> {
	sender: FoundAccount;
	recipient: FoundAccount;
}

function UnsignedTxView({
	sender,
	recipient,
	scannerStore,
	route
}: Props): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { getNetwork } = useContext(NetworksContext);
	const { tx, rawPayload } = scannerStore.state;
	const senderNetworkParams = getNetwork(sender.networkKey);
	const isEthereum = isEthereumNetworkParams(senderNetworkParams);
	const { value, gas, gasPrice } = tx as Transaction;
	const [isProcessing, payload] = usePayloadDetails(
		rawPayload,
		sender.networkKey
	);

	function renderPayloadDetails(): React.ReactNode {
		if (isEthereum) {
			return (
				<View style={[styles.bodyContent, { marginTop: 16 }]}>
					<TxDetailsCard
						style={{ marginBottom: 20 }}
						description={strings.INFO_ETH_TX}
						value={value}
						gas={gas}
						gasPrice={gasPrice}
					/>
					<Text style={styles.title}>Recipient</Text>
					<CompatibleCard account={recipient} accountsStore={accountsStore} />
				</View>
			);
		} else {
			if (!isProcessing && payload !== null) {
				return (
					<PayloadDetailsCard
						networkKey={sender.networkKey}
						payload={payload}
					/>
				);
			} else {
				return <View />;
			}
		}
	}

	const approveTransaction = (): void => {
		const resolve = route.params.resolve;
		resolve();
	};

	return (
		<SafeAreaScrollViewContainer testID={testIDs.DetailsTx.detailsScreen}>
			<Text style={styles.topTitle}>Signed extrinsic</Text>
			<CompatibleCard
				account={sender}
				accountsStore={accountsStore}
				titlePrefix={'from:'}
			/>
			{renderPayloadDetails()}
			<Separator
				shadow={true}
				style={{
					height: 0,
					marginVertical: 20
				}}
			/>
			<Text style={styles.topTitle}>
				You are about to sign this transaction, please verify with care
			</Text>
			<Button
				onPress={approveTransaction}
				title="SIGN"
				testID={testIDs.DetailsTx.signButton}
			/>
		</SafeAreaScrollViewContainer>
	);
}

export default DetailsTx;
