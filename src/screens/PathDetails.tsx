// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import { StackNavigationProp } from '@react-navigation/stack';
import PathCard from 'components/PathCard';
import PopupMenu from 'components/PopupMenu';
import QRScannerAndDerivationTab from 'components/QRScannerAndDerivationTab';
import QrView from 'components/QrView';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { LeftScreenHeading } from 'components/ScreenHeading';
import { UnknownAccountWarning } from 'components/Warnings';
import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import React, { useContext } from 'react';
import { ScrollView, StyleSheet, View } from 'react-native';
import colors from 'styles/colors';
import { AccountsStoreStateWithIdentity } from 'types/identityTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { RootStackParamList } from 'types/routes';
import { generateAccountId } from 'utils/account';
import { alertDeleteAccount, alertError } from 'utils/alertUtils';
import { withCurrentIdentity } from 'utils/HOC';
import { getAddressWithPath, getNetworkKey, getPathName, getPathsWithSubstrateNetworkKey, isSubstrateHardDerivedPath, isSubstratePath } from 'utils/identitiesUtils';
import { navigateToPathsList, useUnlockSeed } from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';

import { AlertContext, NetworksContext } from '../context';

interface Props {
	path: string;
	networkKey: string;
	navigation:
		| StackNavigationProp<RootStackParamList, 'PathDetails'>
		| StackNavigationProp<RootStackParamList, 'PathsList'>;
	accountsStore: AccountsStoreStateWithIdentity;
}

export function PathDetailsView({ accountsStore, navigation, networkKey, path }: Props): React.ReactElement {
	const { currentIdentity } = accountsStore.state;
	const address = getAddressWithPath(path, currentIdentity);
	const accountName = getPathName(path, currentIdentity);
	const { setAlert } = useContext(AlertContext);
	const { isSeedRefValid } = useSeedRef(currentIdentity.encryptedSeed);
	const { unlockWithPassword, unlockWithoutPassword } = useUnlockSeed(isSeedRefValid);
	const networksContextState = useContext(NetworksContext);
	const { allNetworks } = networksContextState;

	if (!address) {
		return <View />;
	}

	const isUnknownNetwork = networkKey === UnknownNetworkKeys.UNKNOWN;
	const formattedNetworkKey = isUnknownNetwork ? defaultNetworkKey : networkKey;
	const accountId = generateAccountId(address,
		formattedNetworkKey,
		allNetworks);

	const onTapDeriveButton = (): Promise<void> =>
		unlockWithoutPassword({
			name: 'PathDerivation',
			params: { parentPath: path }
		});

	const onOptionSelect = async (value: string): Promise<void> => {
		switch (value) {
		case 'PathDelete':
			alertDeleteAccount(setAlert, 'this account', async () => {
				try {
					await accountsStore.deletePath(path, networksContextState);

					if (isSubstratePath(path)) {
						const listedPaths = getPathsWithSubstrateNetworkKey(accountsStore.state.currentIdentity!,
							networkKey,
							networksContextState);
						const hasOtherPaths = listedPaths.length > 0;

						hasOtherPaths
							? navigateToPathsList(navigation, networkKey)
							: navigation.navigate('Main');
					} else {
						navigation.navigate('Main');
					}
				} catch (err) {
					alertError(setAlert,
						`Can't delete this account: ${err.toString()}`);
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
			<ScrollView
				bounces={false}
				testID={testIDs.PathDetail.screen}
			>
				<LeftScreenHeading
					headMenu={
						<PopupMenu
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
							menuTriggerIconName={'more-vert'}
							onSelect={onOptionSelect}
							testID={testIDs.PathDetail.popupMenuButton}
						/>
					}
					networkKey={formattedNetworkKey}
					title="Public Address"
				/>
				<PathCard
					identity={currentIdentity}
					path={path}
				/>
				<QrView data={`${accountId}:${accountName}`} />
				{isUnknownNetwork && <UnknownAccountWarning isPath />}
			</ScrollView>
			{isSubstratePath(path) && (
				<QRScannerAndDerivationTab
					derivationTestID={testIDs.PathDetail.deriveButton}
					onPress={onTapDeriveButton}
					title="Derive New Account"
				/>
			)}
		</SafeAreaViewContainer>
	);
}

function PathDetails({ accountsStore, navigation, route }: NavigationAccountIdentityProps<'PathDetails'>): React.ReactElement {
	const path = route.params.path;
	const networksContextState = useContext(NetworksContext);
	const networkKey = getNetworkKey(path,
		accountsStore.state.currentIdentity,
		networksContextState);

	return (
		<PathDetailsView
			accountsStore={accountsStore}
			navigation={navigation}
			networkKey={networkKey}
			path={path}
		/>
	);
}

const styles = StyleSheet.create({ deleteText: { color: colors.signal.error } });

export default withCurrentIdentity(PathDetails);
