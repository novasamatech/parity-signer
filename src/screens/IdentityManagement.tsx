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

import React from 'react';
import { withNavigation } from 'react-navigation';
import { ScrollView, StyleSheet, View } from 'react-native';

import testIDs from 'e2e/testIDs';
import { NavigationAccountProps } from 'types/props';
import { withAccountStore } from 'utils/HOC';
import TextInput from 'components/TextInput';
import {
	navigateToLandingPage,
	unlockSeedPhrase
} from 'utils/navigationHelpers';
import {
	alertDeleteIdentity,
	alertIdentityDeletionError
} from 'utils/alertUtils';
import ScreenHeading from 'components/ScreenHeading';
import colors from 'styles/colors';
import PopupMenu from 'components/PopupMenu';

function IdentityManagement({
	accounts,
	navigation
}: NavigationAccountProps<{}>): React.ReactElement {
	const { currentIdentity } = accounts.state;
	if (!currentIdentity) return <View />;

	const onOptionSelect = (value: string): void => {
		if (value === 'PathDelete') {
			alertDeleteIdentity(
				async (): Promise<void> => {
					await unlockSeedPhrase(navigation);
					const deleteSucceed = await accounts.deleteCurrentIdentity();
					if (deleteSucceed) {
						navigateToLandingPage(navigation, true);
					} else {
						alertIdentityDeletionError();
					}
				}
			);
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
				onChangeText={(name: string): Promise<void> =>
					accounts.updateIdentityName(name)
				}
				value={currentIdentity.name}
				placeholder="Enter a new identity name"
				focus
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
