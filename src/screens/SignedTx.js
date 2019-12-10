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

'use strict';

import React, { useEffect } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';

import colors from '../colors';
import PayloadDetailsCard from '../components/PayloadDetailsCard';
import TxDetailsCard from '../components/TxDetailsCard';
import QrView from '../components/QrView';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import testIDs from '../../e2e/testIDs';
import { withAccountAndScannerStore } from '../util/HOC';
import fontStyles from '../fontStyles';
import CompatibleCard from '../components/CompatibleCard';

export function SignedTx({ scanner, accounts }) {
	const { gas, gasPrice, value } = scanner.getTx();
	const data = scanner.getSignedTxData();
	const recipient = scanner.getRecipient();
	const sender = scanner.getSender();

	useEffect(
		() =>
			function() {
				scanner.cleanup();
			},
		[scanner]
	);

	return (
		<ScrollView
			contentContainerStyle={{ paddingBottom: 40 }}
			style={styles.body}
		>
			<Text style={styles.topTitle}>Scan Signature</Text>
			<View style={styles.qr} testID={testIDs.SignedTx.qrView}>
				<QrView data={data} />
			</View>

			<Text style={styles.title}>Transaction Details</Text>
			<View style={{ marginBottom: 20, marginHorizontal: 20 }}>
				{NETWORK_LIST[sender.networkKey].protocol ===
				NetworkProtocols.ETHEREUM ? (
					<React.Fragment>
						<TxDetailsCard
							style={{ marginBottom: 20 }}
							description={TX_DETAILS_MSG}
							value={value}
							gas={gas}
							gasPrice={gasPrice}
						/>
						<Text style={styles.title}>Recipient</Text>
						<CompatibleCard account={recipient} accountsStore={accounts} />
					</React.Fragment>
				) : (
					<PayloadDetailsCard
						style={{ marginBottom: 20 }}
						description={TX_DETAILS_MSG}
						protocol={NETWORK_LIST[sender.networkKey].protocol}
						prefix={NETWORK_LIST[sender.networkKey].prefix}
						signature={data}
					/>
				)}
			</View>
		</ScrollView>
	);
}

export default withAccountAndScannerStore(SignedTx);

const TX_DETAILS_MSG = 'After signing and publishing you will have sent';

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start',
		backgroundColor: colors.bg,
		flex: 1,
		paddingTop: 24
	},
	qr: {
		marginBottom: 20
	},
	title: {
		...fontStyles.h2,
		marginHorizontal: 20,
		paddingBottom: 20
	},
	topTitle: {
		...fontStyles.h1,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
