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

import React from 'react';
import { ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import AccountCard from '../components/AccountCard';
import fonts from '../fonts';
import {
	NETWORK_LIST,
	UnknownNetworkKeys,
	SubstrateNetworkKeys
} from '../constants';
import AccountsStore from '../stores/AccountsStore';
import { emptyAccount } from '../util/account';

export default class AccountNetworkChooser extends React.PureComponent {
	static navigationOptions = {
		headerBackTitle: 'Back',
		title: 'Choose a network'
	};
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => (
					<AccountNetworkChooserView {...this.props} accounts={accounts} />
				)}
			</Subscribe>
		);
	}
}

class AccountNetworkChooserView extends React.PureComponent {
	render() {
		const { navigation } = this.props;
		const { accounts } = this.props;

		return (
			<ScrollView style={styles.body}>
				<Text style={styles.title}>CHOOSE NETWORK</Text>
				{Object.entries(NETWORK_LIST)
					.filter(
						([networkKey]) =>
							(__DEV__ && networkKey !== UnknownNetworkKeys.UNKNOWN) ||
							(networkKey !== SubstrateNetworkKeys.SUBSTRATE_DEV &&
								networkKey !== UnknownNetworkKeys.UNKNOWN)
					)
					.map(([networkKey, networkParams]) => (
						<AccountCard
							address={''}
							networkKey={networkKey}
							onPress={() => {
								accounts.updateNew(emptyAccount('', networkKey));
								navigation.goBack();
							}}
							title={networkParams.title}
						/>
					))}
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
	header: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center'
	},
	title: {
		color: colors.bg_text_sec,
		flexDirection: 'column',
		fontFamily: fonts.bold,
		fontSize: 18,
		justifyContent: 'center',
		marginTop: 16,
		paddingLeft: 72
	}
});
