// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import AccountCard from 'components/AccountCard';
import QrView from 'components/QrView';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import Separator from 'components/Separator';
import testIDs from 'e2e/testIDs';
import MessageDetailsCard from 'modules/sign/components/MessageDetailsCard';
import PayloadDetailsCard from 'modules/sign/components/PayloadDetailsCard';
import strings from 'modules/sign/strings';
import styles from 'modules/sign/styles';
import React, { useContext, useEffect, useRef } from 'react';
import { Text, View } from 'react-native';
import fontStyles from 'styles/fontStyles';
import { isEthereumNetwork } from 'types/networkTypes';
import { NavigationProps, NavigationScannerProps } from 'types/props';

import { isU8a, u8aToHex } from '@polkadot/util';

import { AccountsContext, NetworksContext, ScannerContext } from '../../../context';

interface Props extends NavigationScannerProps<'SignedMessage'> {
	sender: string;
	message: string;
}

function SignedMessageView({ message, sender: senderAddress }: Props): React.ReactElement {
	const { getAccountByAddress } = useContext(AccountsContext);
	const sender = getAccountByAddress(senderAddress);
	const { state: { dataToSign, isHash, signedData } } = useContext(ScannerContext);
	const { getNetwork } = useContext(NetworksContext);

	if (!sender) {
		console.error('no sender')

		return<View/>;
	}

	const senderNetworkParams = getNetwork(sender.networkKey);
	const isEthereum = !!senderNetworkParams && isEthereumNetwork(senderNetworkParams);

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
			<AccountCard
				address={senderAddress}
				titlePrefix={'from:'}
			/>
			{!isEthereum && dataToSign ? (
				<PayloadDetailsCard
					description={strings.INFO_MULTI_PART}
					networkKey={sender.networkKey}
					signature={signedData.toString()}
				/>
			) : null}
			<MessageDetailsCard
				data={isU8a(dataToSign) ? u8aToHex(dataToSign) : dataToSign.toString()}
				isHash={isHash ?? false}
				message={message}
				style={styles.bodyContent}
			/>
		</SafeAreaScrollViewContainer>
	);
}

export default function SignedMessage(props: NavigationProps<'SignedMessage'>): React.ReactElement {
	const { cleanup, state } = useContext(ScannerContext);
	const { message, senderAddress: sender } = state;
	const clean = useRef(cleanup);

	useEffect(() => clean.current, [clean]);

	if (sender === null || message === null) return <View />;

	return (
		<SignedMessageView
			message={message}
			sender={sender}
			{...props}
		/>
	);
}
