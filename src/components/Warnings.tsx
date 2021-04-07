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

import React from 'react';
import { StyleSheet, Text, View } from 'react-native';

import { colors, fonts } from 'styles/index';

export function UnknownAccountWarning({
	isPath
}: {
	isPath?: boolean;
}): React.ReactElement {
	return (
		<View style={styles.warningView}>
			<Text style={styles.warningTitle}>Warning</Text>
			{isPath ? (
				<Text style={styles.warningText}>
					This account is not bond to a specific network.
					{'\n'}
					{'\n'}
					This could be because the network specifications are updated or the
					account is generated in a previous version. The address currently
					displayed is using Kusama format.
					{'\n'}
					{'\n'}
					To bind the desired network with this account you need to:
					{'\n'}- remember account path
					{'\n'}- delete the account
					{'\n'}- tap "Add Network Account {'->'} Create Custom Path"
					{'\n'}- derive an account with the same path as before
				</Text>
			) : (
				<Text style={styles.warningText}>
					This account wasn't retrieved successfully. This could be because
					network's genesisHash is changed, or you upgraded Layer Wallet without
					wiping your device and this account couldn't be migrated.
					{'\n'}
					{'\n'}
					To be able to use this account you need to:
					{'\n'}- write down its key phrase
					{'\n'}- delete it
					{'\n'}- recover/derive it
					{'\n'}
				</Text>
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	warningText: {
		color: colors.text.main,
		fontFamily: fonts.regular
	},
	warningTitle: {
		color: colors.text.error,
		fontFamily: fonts.bold,
		fontSize: 20,
		marginBottom: 10
	},
	warningView: {
		padding: 20
	}
});
