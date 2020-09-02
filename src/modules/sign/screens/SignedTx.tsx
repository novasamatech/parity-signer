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

import React, { useContext, useEffect, useRef } from 'react';
import { Text, View } from 'react-native';

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
import QrView from 'components/QrView';
import fontStyles from 'styles/fontStyles';
import CompatibleCard from 'components/CompatibleCard';
import { Transaction } from 'utils/transaction';
import styles from 'modules/sign/styles';
import Separator from 'components/Separator';

function SignedTx(props: NavigationProps<'SignedTx'>): React.ReactElement {
	const scannerStore = useContext(ScannerContext);
	const { recipient, sender } = scannerStore.state;
	const cleanup = useRef(scannerStore.cleanup);

	useEffect(() => cleanup.current, [cleanup]);

	if (sender === null || recipient === null) return <View />;
	return (
		<SignedTxView
			sender={sender}
			recipient={recipient}
			scannerStore={scannerStore}
			{...props}
		/>
	);
}

interface Props extends NavigationScannerProps<'SignedTx'> {
	sender: FoundAccount;
	recipient: FoundAccount;
}

function SignedTxView({
	sender,
	recipient,
	scannerStore
}: Props): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { getNetwork } = useContext(NetworksContext);
	const { signedData, tx } = scannerStore.state;
	const senderNetworkParams = getNetwork(sender.networkKey);
	const isEthereum = isEthereumNetworkParams(senderNetworkParams);
	const { value, gas, gasPrice } = tx as Transaction;

	return (
		<SafeAreaScrollViewContainer>
			<Text style={styles.topTitle}>Signed extrinsic</Text>
			<Separator
				shadow={true}
				style={{
					height: 0,
					marginVertical: 20
				}}
			/>
			<Text style={[fontStyles.h_subheading, { paddingHorizontal: 16 }]}>
				{'Scan to publish'}
			</Text>
			<View style={styles.qr} testID={testIDs.SignedTx.qrView}>
				<QrView data={signedData} />
			</View>
			<CompatibleCard
				account={sender}
				accountsStore={accountsStore}
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
					<CompatibleCard account={recipient} accountsStore={accountsStore} />
				</View>
			)}
		</SafeAreaScrollViewContainer>
	);
}

export default SignedTx;
