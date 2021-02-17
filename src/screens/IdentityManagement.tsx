// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import PopupMenu from 'components/PopupMenu';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading from 'components/ScreenHeading';
import TextInput from 'components/TextInput';
import testIDs from 'e2e/testIDs';
import React, { useContext } from 'react';
import { StyleSheet, View } from 'react-native';
import colors from 'styles/colors';
import { NavigationAccountIdentityProps } from 'types/props';
import { alertDeleteIdentity, alertError } from 'utils/alertUtils';
import { withCurrentIdentity } from 'utils/HOC';
import { navigateToLandingPage, unlockAndReturnSeed, unlockSeedPhrase } from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';

import { AlertContext } from '../context';

type Props = NavigationAccountIdentityProps<'IdentityManagement'>;

function IdentityManagement({ accountsStore, navigation }: Props): React.ReactElement {
	const { currentIdentity } = accountsStore.state;
	const { setAlert } = useContext(AlertContext);
	const { destroySeedRef } = useSeedRef(currentIdentity.encryptedSeed);

	if (!currentIdentity) return <View />;

	const onRenameIdentity = async (name: string): Promise<void> => {
		try {
			await accountsStore.updateIdentityName(name);
		} catch (err) {
			alertError(setAlert, `Can't rename: ${err.message}`);
		}
	};

	const onOptionSelect = async (value: string): Promise<void> => {
		if (value === 'IdentityDelete') {
			alertDeleteIdentity(setAlert,
				async (): Promise<void> => {
					await unlockSeedPhrase(navigation, false);

					try {
						await destroySeedRef();
						await accountsStore.deleteCurrentIdentity();
						navigateToLandingPage(navigation);
					} catch (err) {
						alertError(setAlert, "Can't delete Identity.");
					}
				});
		} else if (value === 'IdentityBackup') {
			const seedPhrase = await unlockAndReturnSeed(navigation);

			navigation.pop();
			navigation.navigate(value, { isNew: false, seedPhrase });
		}
	};

	return (
		<SafeAreaViewContainer>
			<ScreenHeading
				headMenu={
					<PopupMenu
						menuItems={[
							{ text: 'Backup', value: 'IdentityBackup' },
							{
								testID: testIDs.IdentityManagement.deleteButton,
								text: 'Delete',
								textStyle: styles.deleteText,
								value: 'IdentityDelete'
							}
						]}
						menuTriggerIconName={'more-vert'}
						onSelect={onOptionSelect}
						testID={testIDs.IdentityManagement.popupMenuButton}
					/>
				}
				title="Manage Identity"
			/>
			<TextInput
				focus
				label="Display Name"
				onChangeText={onRenameIdentity}
				placeholder="Enter a new identity name"
				value={currentIdentity.name}
			/>
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(IdentityManagement);

const styles = StyleSheet.create({
	deleteText: { color: colors.signal.error },
	header: {
		flexDirection: 'row',
		paddingBottom: 24,
		paddingLeft: 16,
		paddingRight: 16
	}
});
