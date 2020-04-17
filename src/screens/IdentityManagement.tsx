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
import { StyleSheet } from 'react-native';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { NavigationAccountProps } from 'types/props';
import { withAccountStore, withCurrentIdentity } from 'utils/HOC';
import TextInput from 'components/TextInput';
import {
	getSeedPhrase,
	navigateToLandingPage,
	unlockSeedPhrase
} from 'utils/navigationHelpers';
import {
	alertDeleteIdentity,
	alertIdentityDeletionError,
	alertRenamingError
} from 'utils/alertUtils';
import ScreenHeading from 'components/ScreenHeading';
import colors from 'styles/colors';
import PopupMenu from 'components/PopupMenu';
import { useSeedRef } from 'utils/seedRefHooks';

type Props = NavigationAccountProps<'IdentityManagement'>;

function IdentityManagement({
	accounts,
	navigation
}: Props): React.ReactElement {
	const { currentIdentity } = accounts.state;
	const { destroySeedRef } = useSeedRef(currentIdentity!.encryptedSeed);

	const onRenameIdentity = async (name: string): Promise<void> => {
		try {
			await accounts.updateIdentityName(name);
		} catch (err) {
			alertRenamingError(err.message);
		}
	};

	const onOptionSelect = async (value: string): Promise<void> => {
		if (value === 'IdentityDelete') {
			alertDeleteIdentity(
				async (): Promise<void> => {
					await unlockSeedPhrase(navigation, false);
					try {
						await destroySeedRef();
						await accounts.deleteCurrentIdentity();
						navigateToLandingPage(navigation);
					} catch (err) {
						alertIdentityDeletionError();
					}
				}
			);
		} else if (value === 'IdentityBackup') {
			const seedPhrase = await getSeedPhrase(navigation);
			navigation.pop();
			navigation.navigate(value, { isNew: false, seedPhrase });
		}
	};

	return (
		<SafeAreaViewContainer>
			<ScreenHeading
				title="Manage Identity"
				headMenu={
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
								value: 'IdentityDelete'
							}
						]}
					/>
				}
			/>
			<TextInput
				label="Display Name"
				onChangeText={onRenameIdentity}
				value={currentIdentity!.name}
				placeholder="Enter a new identity name"
				focus
			/>
		</SafeAreaViewContainer>
	);
}

export default withAccountStore(withCurrentIdentity(IdentityManagement));

const styles = StyleSheet.create({
	deleteText: {
		color: colors.bg_alert
	},
	header: {
		flexDirection: 'row',
		paddingBottom: 24,
		paddingLeft: 16,
		paddingRight: 16
	}
});
