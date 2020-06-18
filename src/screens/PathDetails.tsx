// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import { StackNavigationProp } from '@react-navigation/stack';
import React from 'react';
import { ScrollView, StyleSheet, View } from 'react-native';

import QRScannerAndDerivationTab from 'components/QRScannerAndDerivationTab';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
// TODO use typescript 3.8's type import, Wait for prettier update.
import { AccountsStoreStateWithIdentity } from 'types/identityTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { RootStackParamList } from 'types/routes';
import { withAccountStore, withCurrentIdentity } from 'utils/HOC';
import PathCard from 'components/PathCard';
import PopupMenu from 'components/PopupMenu';
import { LeftScreenHeading } from 'components/ScreenHeading';
import colors from 'styles/colors';
import QrView from 'components/QrView';
import {
	getAddressWithPath,
	getNetworkKey,
	getPathName,
	getPathsWithSubstrateNetworkKey,
	isSubstrateHardDerivedPath,
	isSubstratePath
} from 'utils/identitiesUtils';
import { alertDeleteAccount, alertPathDeletionError } from 'utils/alertUtils';
import { navigateToPathsList, useUnlockSeed } from 'utils/navigationHelpers';
import { generateAccountId } from 'utils/account';
import { UnknownAccountWarning } from 'components/Warnings';
import { useSeedRef } from 'utils/seedRefHooks';

interface Props {
	path: string;
	networkKey: string;
	navigation:
		| StackNavigationProp<RootStackParamList, 'PathDetails'>
		| StackNavigationProp<RootStackParamList, 'PathsList'>;
	accounts: AccountsStoreStateWithIdentity;
}

export function PathDetailsView({
	accounts,
	navigation,
	path,
	networkKey
}: Props): React.ReactElement {
	const { currentIdentity } = accounts.state;
	const address = getAddressWithPath(path, currentIdentity);
	const accountName = getPathName(path, currentIdentity);
	const { isSeedRefValid } = useSeedRef(currentIdentity.encryptedSeed);
	const { unlockWithoutPassword, unlockWithPassword } = useUnlockSeed(
		isSeedRefValid
	);
	if (!address) return <View />;
	const isUnknownNetwork = networkKey === UnknownNetworkKeys.UNKNOWN;
	const formattedNetworkKey = isUnknownNetwork ? defaultNetworkKey : networkKey;
	const accountId = generateAccountId({
		address,
		networkKey: formattedNetworkKey
	});

	const onTapDeriveButton = (): Promise<void> =>
		unlockWithoutPassword({
			name: 'PathDerivation',
			params: { parentPath: path }
		});

	const onOptionSelect = async (value: string): Promise<void> => {
		switch (value) {
			case 'PathDelete':
				alertDeleteAccount('this account', async () => {
					try {
						await accounts.deletePath(path);
						if (isSubstratePath(path)) {
							const listedPaths = getPathsWithSubstrateNetworkKey(
								accounts.state.currentIdentity,
								networkKey
							);
							const hasOtherPaths = listedPaths.length > 0;
							hasOtherPaths
								? navigateToPathsList(navigation, networkKey)
								: navigation.navigate('Main');
						} else {
							navigation.navigate('Main');
						}
					} catch (err) {
						alertPathDeletionError(err);
					}
				});
				break;
			case 'PathExport': {
				const pathMeta = currentIdentity.meta.get(path)!;
				if (pathMeta.hasPassword) {
					await unlockWithPassword(password => ({
						name: 'PathSecret',
						params: {
							password,
							path
						}
					}));
				} else {
					await unlockWithoutPassword({ name: 'PathSecret', params: { path } });
				}
				break;
			}
			case 'PathManagement':
				navigation.navigate('PathManagement', { path });
				break;
		}
	};

	return (
		<SafeAreaViewContainer>
			<ScrollView testID={testIDs.PathDetail.screen} bounces={false}>
				<LeftScreenHeading
					title="Public Address"
					networkKey={formattedNetworkKey}
					headMenu={
						<PopupMenu
							testID={testIDs.PathDetail.popupMenuButton}
							onSelect={onOptionSelect}
							menuTriggerIconName={'more-vert'}
							menuItems={[
								{ text: 'Edit', value: 'PathManagement' },
								{
									hide: !isSubstrateHardDerivedPath(path),
									testID: testIDs.PathDetail.exportButton,
									text: 'Export Account',
									value: 'PathExport'
								},
								{
									testID: testIDs.PathDetail.deleteButton,
									text: 'Delete',
									textStyle: styles.deleteText,
									value: 'PathDelete'
								}
							]}
						/>
					}
				/>
				<PathCard identity={currentIdentity} path={path} />
				<QrView data={`${accountId}:${accountName}`} />
				{isUnknownNetwork && <UnknownAccountWarning isPath />}
			</ScrollView>
			{isSubstratePath(path) && (
				<QRScannerAndDerivationTab
					derivationTestID={testIDs.PathDetail.deriveButton}
					title="Derive New Account"
					onPress={onTapDeriveButton}
				/>
			)}
		</SafeAreaViewContainer>
	);
}

function PathDetails({
	accounts,
	navigation,
	route
}: NavigationAccountIdentityProps<'PathDetails'>): React.ReactElement {
	const path = route.params.path;
	const networkKey = getNetworkKey(path, accounts.state.currentIdentity);
	return (
		<PathDetailsView
			accounts={accounts}
			navigation={navigation}
			path={path}
			networkKey={networkKey}
		/>
	);
}

const styles = StyleSheet.create({
	deleteText: {
		color: colors.signal.error
	}
});

export default withAccountStore(withCurrentIdentity(PathDetails));
