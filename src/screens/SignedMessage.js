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

import { GenericExtrinsicPayload } from '@polkadot/types';
import PropTypes from 'prop-types';
import React from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fonts from '../fonts';
import PayloadDetailsCard from '../components/PayloadDetailsCard';
import QrView from '../components/QrView';
import {
	NETWORK_LIST,
	NetworkProtocols,
	SUBSTRATE_NETWORK_LIST
} from '../constants';
import ScannerStore from '../stores/ScannerStore';
import { hexToAscii, isAscii } from '../util/strings';

export default class SignedMessage extends React.PureComponent {
	render() {
		return (
			<Subscribe to={[ScannerStore]}>
				{scannerStore => {
					return (
						<SignedMessageView
							data={scannerStore.getSignedTxData()}
							isHash={scannerStore.getIsHash()}
							message={scannerStore.getMessage()}
							prehash={scannerStore.getPrehashPayload()}
						/>
					);
				}}
			</Subscribe>
		);
	}
}

export class SignedMessageView extends React.PureComponent {
	static propTypes = {
		data: PropTypes.string.isRequired, // post sign
		isHash: PropTypes.bool,
		message: PropTypes.string, // pre sign
		prehash: PropTypes.instanceOf(GenericExtrinsicPayload)
	};

	render() {
		const { data, isHash, message, prehash } = this.props;

		let prefix;
		if (prehash) {
			const isEthereum =
				NETWORK_LIST[prehash.genesisHash.toString()].protocol ===
				NetworkProtocols.ETHEREUM;
			prefix =
				!isEthereum &&
				SUBSTRATE_NETWORK_LIST[prehash.genesisHash.toString()].prefix;
		}

		return (
			<ScrollView style={styles.body} contentContainerStyle={{ padding: 20 }}>
				<Text style={styles.topTitle}>SCAN SIGNATURE</Text>
				<View style={styles.qr}>
					<QrView data={data} />
				</View>
				<Text style={styles.title}>{!isHash && 'MESSAGE'}</Text>
				{!isEthereum && prehash ? (
					<PayloadDetailsCard
						style={{ marginBottom: 20 }}
						description="You are about to confirm sending the following extrinsic. We will sign the hash of the payload as it is oversized."
						payload={prehash}
						prefix={prefix}
					/>
				) : null}
				{isHash ? (
					<Text style={styles.title}>HASH</Text>
				) : (
					<Text style={styles.title}>MESSAGE</Text>
				)}
				<Text style={styles.message}>
					{isHash ? message : isAscii(message) ? hexToAscii(message) : data}
				</Text>
			</ScrollView>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		overflow: 'hidden'
	},
	message: {
		backgroundColor: colors.card_bg,
		fontFamily: fonts.regular,
		fontSize: 20,
		lineHeight: 26,
		marginBottom: 20,
		minHeight: 120,
		padding: 10
	},
	qr: {
		backgroundColor: colors.card_bg,
		marginBottom: 20
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	},
	topTitle: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
