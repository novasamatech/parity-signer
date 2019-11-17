import React from 'react';
import { withNavigation } from 'react-navigation';
import { ScrollView, StyleSheet, View } from 'react-native';

import { withAccountStore } from '../util/HOC';
import TextInput from '../components/TextInput';
import { navigateToLandingPage, unlockSeed } from '../util/navigationHelpers';
import {
	alertDeleteIdentity,
	alertIdentityDeletionError
} from '../util/alertUtils';
import testIDs from '../../e2e/testIDs';
import ScreenHeading from '../components/ScreenHeading';
import colors from '../colors';
import PopupMenu from '../components/PopupMenu';

function IdentityManagement({ accounts, navigation }) {
	const { currentIdentity } = accounts.state;
	if (!currentIdentity) return null;

	const onOptionSelect = value => {
		if (value === 'PathDelete') {
			alertDeleteIdentity(async () => {
				await unlockSeed(navigation);
				const deleteSucceed = await accounts.deleteCurrentIdentity();
				if (deleteSucceed) {
					navigateToLandingPage(navigation, true);
				} else {
					alertIdentityDeletionError();
				}
			});
		} else {
			navigation.navigate('IdentityBackup', { isNew: false });
		}
	};

	return (
		<ScrollView style={styles.body}>
			<ScreenHeading title="Manage Identity" />
			<View style={styles.menuView}>
				<PopupMenu
					testID={testIDs.IdentityManagement.popupMenuButton}
					onSelect={onOptionSelect}
					menuTriggerIconName={'more-vert'}
					menuItems={[
						{ text: 'Backup', value: 'IdentityBackup' },
						{
							testID: testIDs.IdentityManagement.deleteButton,
							text: 'Delete',
							textStyle: styles.deleteText,
							value: 'PathDelete'
						}
					]}
				/>
			</View>
			<TextInput
				label="Display Name"
				onChangeText={name => accounts.updateIdentityName(name)}
				value={currentIdentity.name}
				placeholder="Enter a new identity name"
				focus={true}
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
	},
	deleteText: {
		color: colors.bg_alert
	},
	header: {
		flexDirection: 'row',
		paddingBottom: 24,
		paddingLeft: 16,
		paddingRight: 16
	},
	menuView: {
		alignItems: 'flex-end',
		flex: 1,
		position: 'absolute',
		right: 16,
		top: 5
	}
});
