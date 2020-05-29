// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import strings from 'modules/sign/strings';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NETWORK_LIST } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { FoundAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { NavigationAccountScannerProps } from 'types/props';
import TxDetailsCard from 'modules/sign/components/TxDetailsCard';
import QrView from 'components/QrView';
import { withAccountAndScannerStore } from 'utils/HOC';
import fontStyles from 'styles/fontStyles';
import CompatibleCard from 'components/CompatibleCard';
import { Transaction } from 'utils/transaction';
import styles from 'modules/sign/styles';
import Separator from 'components/Separator';

function SignedTx(
	props: NavigationAccountScannerProps<'SignedTx'>
): React.ReactElement {
	const { scannerStore } = props;
	const recipient = scannerStore.getRecipient();
	const sender = scannerStore.getSender();
	if (sender === null || recipient === null) return <View />;
	return <SignedTxView sender={sender} recipient={recipient} {...props} />;
}

interface Props extends NavigationAccountScannerProps<'SignedTx'> {
	sender: FoundAccount;
	recipient: FoundAccount;
}

function SignedTxView({
	sender,
	recipient,
	accounts,
	scannerStore
}: Props): React.ReactElement {
	const data = scannerStore.getSignedTxData();
	const tx = scannerStore.getTx();
	const senderNetworkParams = NETWORK_LIST[sender.networkKey];
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
		<SafeAreaScrollViewContainer style={styles.body}>
			<Text style={[styles.topTitle, { marginBottom: 16 }]}>
				Signed {isEthereum ? 'transaction' : 'extrinsic'}
			</Text>
			<CompatibleCard
				account={sender}
				accountsStore={accounts}
				titlePrefix={'from:'}
			/>
			{isEthereum && (
				<View style={[styles.bodyContent, { marginTop: 16 }]}>
					<TxDetailsCard
						style={{ marginBottom: 20 }}
						description={strings.INFO_ETH_TX}
						value={value}
						gas={gas}
						gasPrice={gasPrice}
					/>
					<Text style={styles.title}>Recipient</Text>
					<CompatibleCard account={recipient} accountsStore={accounts} />
				</View>
			)}
			<Separator
				shadow={true}
				style={{
					height: 0,
					marginVertical: 24
				}}
			/>
			<Text style={[fontStyles.h_subheading, { paddingHorizontal: 16 }]}>
				{'Scan to publish'}
			</Text>
			<View style={styles.qr} testID={testIDs.SignedTx.qrView}>
				<QrView data={data} />
			</View>
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountAndScannerStore(SignedTx);
