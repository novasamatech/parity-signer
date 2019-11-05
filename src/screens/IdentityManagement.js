import React from 'react';
import { withNavigation } from 'react-navigation';
import { ScrollView, Text } from 'react-native';

import { withAccountStore } from '../util/HOC';
import Button from '../components/Button';
import TextInput from '../components/TextInput';

function IdentityManagement({ accounts, navigation }) {
	const { currentIdentity } = accounts.state;

	return (
		<ScrollView>
			<Text>Display Name</Text>
			<TextInput
				onChangeText={name => accounts.updateIdentityName(name)}
				value={currentIdentity.name}
				placeholder="Enter a new identity name"
				focus={true}
			/>
			<Text>Recover Phrase</Text>
			<Button
				onPress={() => {
					navigation.navigate('IdentityBackup', { isNew: false });
				}}
				title="Backup"
			/>
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(IdentityManagement));
