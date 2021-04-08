// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import React, { useContext, useEffect, useRef } from 'react';
import { Text, View } from 'react-native';

import { components } from 'styles/index';
import testIDs from 'e2e/testIDs';
import { AccountsContext } from 'stores/AccountsContext';
import { ScannerContext } from 'stores/ScannerContext';
import { FoundAccount } from 'types/identityTypes';
import { NavigationProps, NavigationScannerProps } from 'types/props';
import QrView from 'components/QrView';
import CompatibleCard from 'components/CompatibleCard';
import styles from 'modules/sign/styles';
import Separator from 'components/Separator';

function SignTransactionFinish(
	props: NavigationProps<'SignTransactionFinish'>
): React.ReactElement {
	const scannerStore = useContext(ScannerContext);
	const { recipient, sender } = scannerStore.state;
	const cleanup = useRef(scannerStore.cleanup);

	useEffect(() => cleanup.current, [cleanup]);

	if (sender === null || recipient === null) return <View />;
	return (
		<SignTransactionFinishView
			sender={sender}
			scannerStore={scannerStore}
			{...props}
		/>
	);
}

interface Props extends NavigationScannerProps<'SignTransactionFinish'> {
	sender: FoundAccount;
}

function SignTransactionFinishView({
	sender,
	scannerStore
}: Props): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { signedData } = scannerStore.state;

	return (
		<View style={components.pageWide}>
			<Text style={styles.topTitle}>Signed extrinsic</Text>
			<CompatibleCard account={sender} accountsStore={accountsStore} />
			<Separator
				shadow={true}
				style={{
					height: 0,
					marginVertical: 20
				}}
			/>
			<Text style={[{ paddingHorizontal: 16 }]}>{'Scan to publish'}</Text>
			<View style={styles.qr} testID={testIDs.SignTransactionFinish.qrView}>
				<QrView data={signedData} />
			</View>
		</View>
	);
}

export default SignTransactionFinish;
