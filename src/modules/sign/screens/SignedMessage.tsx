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

import { isU8a, u8aToHex } from '@polkadot/util';
import React, { useEffect } from 'react';
import { Text, View } from 'react-native';

import CompatibleCard from 'components/CompatibleCard';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { FoundAccount } from 'types/identityTypes';
import { NavigationAccountScannerProps } from 'types/props';
import QrView from 'components/QrView';
import { withAccountAndScannerStore } from 'utils/HOC';
import styles from 'modules/sign/styles';
import MessageDetailsCard from 'modules/sign/components/MessageDetailsCard';

interface Props extends NavigationAccountScannerProps<'SignedMessage'> {
	sender: FoundAccount;
	message: string;
}

function SignedMessage(
	props: NavigationAccountScannerProps<'SignedMessage'>
): React.ReactElement {
	const { scannerStore } = props;
	const sender = scannerStore.getSender();
	const message = scannerStore.getMessage();
	if (sender === null || message === null) return <View />;
	return <SignedMessageView sender={sender} message={message} {...props} />;
}

function SignedMessageView({
	sender,
	message,
	accounts,
	scannerStore
}: Props): React.ReactElement {
	const data = scannerStore.getSignedTxData();
	const isHash = scannerStore.getIsHash();
	const dataToSign = scannerStore.getDataToSign();

	useEffect(
		(): (() => void) =>
			function (): void {
				scannerStore.cleanup();
			},
		[scannerStore]
	);

	return (
		<SafeAreaScrollViewContainer>
			<Text style={styles.topTitle}>Signed Message</Text>
			<View testID={testIDs.SignedMessage.qrView}>
				<QrView data={data} />
			</View>
			<View style={styles.bodyContent}>
				<Text style={styles.title}>From Account</Text>
				<CompatibleCard account={sender} accountsStore={accounts} />
				<MessageDetailsCard
					isHash={isHash ?? false}
					message={message}
					data={
						isU8a(dataToSign) ? u8aToHex(dataToSign) : dataToSign.toString()
					}
				/>
			</View>
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountAndScannerStore(SignedMessage);
