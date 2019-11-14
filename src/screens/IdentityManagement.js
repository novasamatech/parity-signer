import React from 'react';
import { withNavigation } from 'react-navigation';
import { ScrollView, Text, StyleSheet } from 'react-native';

import { withAccountStore } from '../util/HOC';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
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
