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

import { isU8a, u8aToHex } from '@polkadot/util';
import React, { useContext, useEffect, useRef } from 'react';
import { Text, View } from 'react-native';

import PayloadDetailsCard from 'modules/sign/components/PayloadDetailsCard';
import strings from 'modules/sign/strings';
import CompatibleCard from 'components/CompatibleCard';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { AccountsContext } from 'stores/AccountsContext';
import { NetworksContext } from 'stores/NetworkContext';
import { ScannerContext } from 'stores/ScannerContext';
import { FoundAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkTypes';
import { NavigationProps, NavigationScannerProps } from 'types/props';
import QrView from 'components/QrView';
import styles from 'modules/sign/styles';
import MessageDetailsCard from 'modules/sign/components/MessageDetailsCard';
import Separator from 'components/Separator';
import fontStyles from 'styles/fontStyles';

interface Props extends NavigationScannerProps<'SignedMessage'> {
	sender: FoundAccount;
	message: string;
}

export default function SignedMessage(
	props: NavigationProps<'SignedMessage'>
): React.ReactElement {
	const scannerStore = useContext(ScannerContext);
	const { sender, message } = scannerStore.state;
	const cleanup = useRef(scannerStore.cleanup);

	useEffect(() => cleanup.current, [cleanup]);

	if (sender === null || message === null) return <View />;
	return (
		<SignedMessageView
			sender={sender}
			message={message}
			scannerStore={scannerStore}
			{...props}
		/>
	);
}

function SignedMessageView({
	sender,
	message,
	scannerStore
}: Props): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { signedData, isHash, dataToSign } = scannerStore.state;
	const { getNetwork } = useContext(NetworksContext);
	const senderNetworkParams = getNetwork(sender.networkKey);
	const isEthereum = isEthereumNetworkParams(senderNetworkParams);

	return (
		<SafeAreaScrollViewContainer>
			<Text style={styles.topTitle}>Signed Message</Text>
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
			<View testID={testIDs.SignedMessage.qrView}>
				<QrView data={signedData} />
			</View>
			<CompatibleCard
				titlePrefix={'from:'}
				account={sender}
				accountsStore={accountsStore}
			/>
			{!isEthereum && dataToSign ? (
				<PayloadDetailsCard
					description={strings.INFO_MULTI_PART}
					signature={signedData.toString()}
					networkKey={sender.networkKey}
				/>
			) : null}
			<MessageDetailsCard
				isHash={isHash ?? false}
				message={message}
				data={isU8a(dataToSign) ? u8aToHex(dataToSign) : dataToSign.toString()}
				style={styles.bodyContent}
			/>
		</SafeAreaScrollViewContainer>
	);
}
