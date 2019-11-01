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
import { StyleSheet, View, Text } from 'react-native';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';
import { emptyIdentity } from '../util/identitiesUtils';
import colors from '../colors';
import fonts from '../fonts';

export default class IdentityNew extends React.Component {
	static navigationOptions = {
		headerBackTitle: 'Back',
		title: 'New Identity'
	};
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => <IdentityNewView {...this.props} accounts={accounts} />}
			</Subscribe>
		);
	}
}

function IdentityNewView({ accounts, navigation }) {
	useEffect(() => {
		accounts.updateNewIdentity(emptyIdentity());
		return function() {
			accounts.updateNewIdentity(emptyIdentity());
		};
	}, [accounts]);

	const updateName = name => {
		accounts.updateNewIdentity({ name });
	};

	return (
		<View style={styles.body}>
			<Text style={styles.titleTop}>NEW IDENTITY</Text>
			<TextInput
				onChangeText={updateName}
				value={accounts.getNewIdentity().name}
				placeholder="Enter a new identity name"
				focus={true}
			/>
			<View style={styles.btnBox}>
				<Button
					title="Recover Identity"
					onPress={() => {
						navigation.navigate('IdentityBackup', {
							isNew: true
						});
					}}
					small={true}
					onlyText={true}
				/>
				<Button title="Create" onPress={() => {}} small={true} />
			</View>
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		overflow: 'hidden',
		padding: 16
	},
	btnBox: { flexDirection: 'row', flexWrap: 'wrap', marginTop: 80 },
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18
	},
	titleTop: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
