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
import { StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';

import colors from '../colors';
import Background from '../components/Background';
import Button from '../components/Button';
import DerivationPathField from '../components/DerivationPathField';
import KeyboardScrollView from '../components/KeyboardScrollView';
import TextInput from '../components/TextInput';
import { NETWORK_LIST } from '../constants';
import fonts from '../fonts';
import AccountsStore from '../stores/AccountsStore';
import { empty } from '../util/account';
import { constructSURI } from '../util/suri';
import { substrateAddress } from '../util/native';

export default class DeriveNew extends React.Component {
	static navigationOptions = {
		headerBackTitle: 'Back',
		title: 'Derive New Account'
	};
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => <DeriveNewView {...this.props} accounts={accounts} />}
			</Subscribe>
		);
	}
}

class DeriveNewView extends React.Component {
	constructor(props) {
		super(props);
		const { accounts } = this.props;
		const { seedPhrase, derivationPath, networkKey } = accounts.getSelected();
		accounts.updateNew({ derivationPath, networkKey, seedPhrase });

		this.state = {
			isDerivationPathValid: true
		};
	}

	componentWillUnmount = function() {
		// called when the user goes back, or finishes the whole process
		this.props.accounts.updateNew(empty());
	};

	render() {
		const { accounts, navigation } = this.props;
		const { isDerivationPathValid } = this.state;
		const selectedAccount = accounts.getSelected();
		const { seedPhrase, networkKey } = selectedAccount;
		const { name } = accounts.getNew();

		if (!selectedAccount) {
			return null;
		}

		return (
			<View style={styles.body}>
				<KeyboardScrollView style={{ padding: 20 }}>
					<Background />
					<View style={styles.top}>
						<Text style={styles.titleTop}>DERIVE ACCOUNT</Text>
						<Text style={styles.title}>NAME</Text>
						<TextInput
							onChangeText={newName => accounts.updateNew({ newName })}
							value={name}
							placeholder="Enter a new account name"
						/>
						<DerivationPathField
							onChange={async ({
								derivationPassword,
								derivationPath,
								isDerivationPathValid
							}) => {
								const prefix = NETWORK_LIST[networkKey].prefix;
								const suri = constructSURI({
									derivePath: derivationPath,
									password: derivationPassword,
									phrase: seedPhrase
								});
								const address = await substrateAddress(suri, prefix);
								accounts.updateNew({
									address,
									derivationPassword,
									derivationPath,
									seed: suri,
									seedPhrase: seedPhrase,
									validBip39Seed: true
								});
								this.setState({
									isDerivationPathValid
								});
							}}
							defaultPath={accounts.getSelected().derivationPath}
							styles={styles}
						/>
					</View>
					<View style={styles.bottom}>
						<Text style={styles.hintText}>
							Next, you will be asked to backup your account, get a pen and some
							paper.
						</Text>
						<Button
							buttonStyles={styles.nextStep}
							title="Unlock to create"
							disabled={!isDerivationPathValid}
							onPress={async () => {
								navigation.navigate('AccountUnlock', {
									isDerived: true
								});
							}}
						/>
					</View>
				</KeyboardScrollView>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		overflow: 'hidden'
	},
	bottom: {
		flexBasis: 50,
		paddingBottom: 15
	},
	hintText: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 12,
		paddingTop: 20,
		textAlign: 'center'
	},
	nextStep: {
		marginTop: 15
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	},
	titleTop: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	},
	top: {
		flex: 1
	}
});
