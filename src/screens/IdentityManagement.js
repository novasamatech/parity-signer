import React from 'react';
import { withNavigation } from 'react-navigation';
import { ScrollView, Text } from 'react-native';

import { withAccountStore } from '../util/HOC';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
import { navigateToLandingPage, unlockSeed } from '../util/navigationHelpers';
import {
	alertDeleteIdentity,
	alertIdentityDeletionError
} from '../util/alertUtils';
import testIDs from '../../e2e/testIDs';

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

			<Button
				testID={testIDs.IdentityManagement.deleteButton}
				onPress={() => {
					alertDeleteIdentity(async () => {
						await unlockSeed(navigation);
						const deleteSucceed = await accounts.deleteCurrentIdentity();
						if (deleteSucceed) {
							navigateToLandingPage(navigation);
						} else {
							alertIdentityDeletionError();
						}
					});
				}}
				title="Delete Identity"
			/>
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(IdentityManagement));
