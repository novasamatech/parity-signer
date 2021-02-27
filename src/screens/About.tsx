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

import CustomScrollView from 'components/CustomScrollView';
import React from 'react';
import { Linking, StyleSheet, Text, View } from 'react-native';
import colors from 'styles/colors';
import fonts from 'styles/fonts';

import { version } from '../../package.json';

export default class About extends React.PureComponent {
	render(): React.ReactElement {
		return (
			<CustomScrollView contentContainerStyle={{ padding: 20 }}>
				<Text style={styles.title}>Stylo v{version}</Text>
				<View>
					<Text style={styles.text}>
						The Stylo mobile application is a secure air-gapped wallet
						developed by Parity Technologies. It allows users to use a
						smartphone as cold storage.
					</Text>
					<Text style={styles.text}>
						This application is meant to be used on a phone that will remain
						offline at any point in time. To upgrade the app, you need to make
						sure you backup your accounts (e.g by writing the recovery phrase on
						paper), then factory reset the phone, then install Stylo's
						new version either from the store (iPhone or android) or from a sd
						card, and finally turn your phone offline for good before recoveing
						or generating new accounts.
					</Text>
					<Text style={styles.text}>
						Any data transfer from or to the App will happen using QR code
						scanning. By doing so, the most sensitive piece of information, the
						private keys, will never leave the phone. The Stylo mobile
						app can be used to store Ethereum or Kusama accounts. This includes
						ETH, ETC or Ether from various testnets (Kovan, Ropsten…) as well as
						KSMs.
					</Text>
					<Text style={styles.text}>
						This app does not send any data to Parity Technologies or any
						partner. The app works entirely offline once installed.
					</Text>
					<Text style={styles.text}>
						The code of this application is available on Github (
						<Text
							onPress={(): Promise<any> =>
								Linking.openURL('https://github.com/paritytech/parity-signer')
							}
							style={[styles.text, { textDecorationLine: 'underline' }]}
						>
							{'https://github.com/paritytech/parity-signer'}
						</Text>
						) and licensed under GNU General Public License v3.0.
					</Text>
					<Text style={styles.text}>
						Find on the Stylo wiki more information about this
						application as well as some tutorials:
						<Text
							onPress={(): Promise<any> =>
								Linking.openURL('https://wiki.parity.io/Parity-Signer-Mobile-App')
							}
							style={[styles.text, { textDecorationLine: 'underline' }]}
						>
							{' https://wiki.parity.io/Parity-Signer-Mobile-App'}
						</Text>
						.
					</Text>
				</View>
			</CustomScrollView>
		);
	}
}

const styles = StyleSheet.create({
	bottom: {
		flexBasis: 50,
		paddingBottom: 15
	},
	text: {
		color: colors.text.main,
		fontFamily: fonts.regular,
		fontSize: 14,
		marginBottom: 20
	},
	title: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	},
	top: { flex: 1 }
});
