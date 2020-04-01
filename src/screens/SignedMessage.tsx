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
import { StyleSheet, Text, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { NavigationScannerProps } from 'types/props';
import QrView from 'components/QrView';
import { withScannerStore } from 'utils/HOC';
import fontStyles from 'styles/fontStyles';
import MessageDetailsCard from 'components/MessageDetailsCard';

function SignedMessage({
	scannerStore
}: NavigationScannerProps<'SignedMessage'>): React.ReactElement {
	const data = scannerStore.getSignedTxData();
	const isHash = scannerStore.getIsHash();
	const message = scannerStore.getMessage();

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
			<MessageDetailsCard
				isHash={isHash}
				message={message ?? ''}
				data={data}
				style={styles.messageDetail}
			/>
		</SafeAreaScrollViewContainer>
	);
}

export default withScannerStore(SignedMessage);

const styles = StyleSheet.create({
	messageDetail: {
		paddingHorizontal: 20
	},
	topTitle: {
		...fontStyles.h1,
		textAlign: 'center'
	}
});
