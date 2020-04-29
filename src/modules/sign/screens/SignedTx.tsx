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

import React, { useEffect } from 'react';
import { Text, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NETWORK_LIST } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { NavigationAccountScannerProps } from 'types/props';
import PayloadDetailsCard from 'components/PayloadDetailsCard';
import TxDetailsCard from 'modules/sign/components/TxDetailsCard';
import QrView from 'components/QrView';
import { withAccountAndScannerStore } from 'utils/HOC';
import fontStyles from 'styles/fontStyles';
import CompatibleCard from 'components/CompatibleCard';
import { Transaction } from 'utils/transaction';
import styles from 'modules/sign/styles';

function SignedTx({
	scannerStore,
	accounts
}: NavigationAccountScannerProps<'SignedTx'>): React.ReactElement {
	const data = scannerStore.getSignedTxData();
	const recipient = scannerStore.getRecipient()!;
	const prehash = scannerStore.getPrehashPayload();
	const txRequest = scannerStore.getTXRequest();

	const tx = scannerStore.getTx();
	const sender = scannerStore.getSender()!;
	const senderNetworkParams = NETWORK_LIST[sender.networkKey];
	// if it is legacy account
	const isEthereum = isEthereumNetworkParams(senderNetworkParams);
	const { value, gas, gasPrice } = tx as Transaction;

	useEffect(
		() =>
			function (): void {
				scannerStore.cleanup();
			},
		[scannerStore]
	);

	return (
		<SafeAreaScrollViewContainer
			contentContainerStyle={{ paddingBottom: 40 }}
			style={styles.body}
		>
			<Text style={styles.topTitle}>Scan Signature</Text>
			<View style={styles.qr} testID={testIDs.SignedTx.qrView}>
				<QrView data={data} />
			</View>

			<Text style={[fontStyles.t_big, styles.bodyContent]}>
				{`You are about to sending the following ${
					isEthereum ? 'transaction' : 'extrinsic'
				}`}
			</Text>
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
						payload={prehash!}
						signature={data}
						networkKey={sender.networkKey}
					/>
				)}
			</View>
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountAndScannerStore(SignedTx);

const TX_DETAILS_MSG = 'After signing and publishing you will have sent';
