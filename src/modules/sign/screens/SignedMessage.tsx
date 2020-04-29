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

import { isU8a, u8aToHex } from '@polkadot/util';
import React, { useEffect } from 'react';
import { StyleSheet, Text, View } from 'react-native';

import CompatibleCard from 'components/CompatibleCard';
import PayloadDetailsCard from 'components/PayloadDetailsCard';
import { NETWORK_LIST } from 'constants/networkSpecs';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import {
	NavigationAccountScannerProps,
} from 'types/props';
import QrView from 'components/QrView';
import { withAccountAndScannerStore } from 'utils/HOC';
import styles from 'modules/sign/styles';
import MessageDetailsCard from 'modules/sign/components/MessageDetailsCard';

function SignedMessage({
	accounts,
	scannerStore
}: NavigationAccountScannerProps<'SignedMessage'>): React.ReactElement {
	const data = scannerStore.getSignedTxData();
	const isHash = scannerStore.getIsHash();
	const message = scannerStore.getMessage()!;
	const prehash = scannerStore.getPrehashPayload();
	const dataToSign = scannerStore.getDataToSign()!;

	const sender = scannerStore.getSender()!;
	const senderNetworkParams = NETWORK_LIST[sender.networkKey];
	// if it is legacy account
	const isEthereum = isEthereumNetworkParams(senderNetworkParams);

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
				{!isEthereum && prehash ? (
					<PayloadDetailsCard
						description="You are about to sending the following extrinsic. We will sign the hash of the payload as it is oversized."
						payload={prehash}
						signature={data}
						networkKey={sender.networkKey}
					/>
				) : null}
				<MessageDetailsCard
					isHash={isHash ?? false}
					message={message}
					data={isU8a(dataToSign) ? u8aToHex(dataToSign) : dataToSign.toString()}
				/>
			</View>
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountAndScannerStore(SignedMessage);
