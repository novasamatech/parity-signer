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

import React, { useContext, useEffect, useRef } from 'react';
import { Text, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { AccountsContext } from 'stores/AccountsContext';
import { ScannerContext } from 'stores/ScannerContext';
import { FoundAccount } from 'types/identityTypes';
import { NavigationProps, NavigationScannerProps } from 'types/props';
import QrView from 'components/QrView';
import fontStyles from 'styles/fontStyles';
import CompatibleCard from 'components/CompatibleCard';
import styles from 'modules/sign/styles';
import Separator from 'components/Separator';

function SignedTx(props: NavigationProps<'SignedTx'>): React.ReactElement {
	const scannerStore = useContext(ScannerContext);
	const { recipient, sender } = scannerStore.state;
	const cleanup = useRef(scannerStore.cleanup);

	useEffect(() => cleanup.current, [cleanup]);

	if (sender === null || recipient === null) return <View />;
	return (
		<SignedTxView sender={sender} scannerStore={scannerStore} {...props} />
	);
}

interface Props extends NavigationScannerProps<'SignedTx'> {
	sender: FoundAccount;
}

function SignedTxView({ sender, scannerStore }: Props): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { signedData } = scannerStore.state;

	return (
		<SafeAreaScrollViewContainer>
			<Text style={styles.topTitle}>Signed extrinsic</Text>
			<CompatibleCard
				account={sender}
				accountsStore={accountsStore}
				titlePrefix={'from:'}
			/>
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
		</SafeAreaScrollViewContainer>
	);
}

export default SignedTx;
