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

import PropTypes from 'prop-types';
import React from 'react';
import { FlatList, StyleSheet, View } from 'react-native';
import { Subscribe } from 'unstated';

import colors from '../colors';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import fonts from '../fonts';
import AccountsStore from '../stores/AccountsStore';
import testIDs from '../../e2e/testIDs';
import ButtonMainAction from '../components/ButtonMainAction';

export default class LegacyAccountList extends React.PureComponent {
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => {
					return (
						<AccountListView
							{...this.props}
							accounts={accounts.getAccounts()}
							onAccountSelected={key => {
								accounts.select(key);
								this.props.navigation.navigate('AccountDetails');
							}}
						/>
					);
				}}
			</Subscribe>
		);
	}
}

class AccountListView extends React.PureComponent {
	static propTypes = {
		accounts: PropTypes.array.isRequired,
		onAccountSelected: PropTypes.func.isRequired
	};

	constructor(props) {
		super(props);
	}

	render() {
		const { accounts, navigation, onAccountSelected } = this.props;
		return (
			<View style={styles.body} testID={testIDs.AccountListScreen.accountList}>
				<Background />
				<FlatList
					ref={list => {
						this.list = list;
					}}
					style={styles.content}
					data={Array.from(accounts.entries())}
					keyExtractor={([key]) => key}
					renderItem={({ item: [accountKey, account] }) => {
						return (
							<AccountCard
								address={account.address}
								networkKey={account.networkKey}
								onPress={() => {
									onAccountSelected(accountKey);
								}}
								style={{ paddingBottom: null }}
								title={account.name}
							/>
						);
					}}
					enableEmptySections
				/>
				<ButtonMainAction onPress={() => navigation.navigate('QrScanner')} />
			</View>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column'
	},
	bottom: {
		marginTop: 20
	},
	content: {
		flex: 1
	},
	header: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center'
	},
	link: {
		textDecorationLine: 'underline'
	},
	menuView: {
		alignItems: 'flex-end',
		flex: 1
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
