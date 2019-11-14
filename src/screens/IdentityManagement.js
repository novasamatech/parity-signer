import React from 'react';
import { withNavigation } from 'react-navigation';
import { ScrollView, Text, StyleSheet } from 'react-native';

import { withAccountStore } from '../util/HOC';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
import { navigateToLandingPage, unlockSeed } from '../util/navigationHelpers';
import {
	alertDeleteIdentity,
	alertIdentityDeletionError
} from '../util/alertUtils';
import testIDs from '../../e2e/testIDs';
import ScreenHeading from '../components/ScreenHeading';
import colors from '../colors';

function IdentityManagement({ accounts, navigation }) {
	const { currentIdentity } = accounts.state;

	return (
		<ScrollView style={styles.body}>
			<ScreenHeading title="Manage Identity" />
			<TextInput
				label="Display Name"
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

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column'
	}
});
