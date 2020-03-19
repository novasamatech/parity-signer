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

import { StackNavigationProp } from '@react-navigation/stack';
import React from 'react';
import { StyleSheet, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
// TODO use typescript 3.8's type import, Wait for prettier update.
import AccountsStore from 'stores/AccountsStore';
import { NavigationAccountProps } from 'types/props';
import { RootStackParamList } from 'types/routes';
import { withAccountStore } from 'utils/HOC';
import PathCard from 'components/PathCard';
import PopupMenu from 'components/PopupMenu';
import { LeftScreenHeading } from 'components/ScreenHeading';
import colors from 'styles/colors';
import QrView from 'components/QrView';
import {
	getAddressWithPath,
	getNetworkKey,
	getNetworkKeyByPath,
	getPathName,
	getPathsWithSubstrateNetworkKey,
	isSubstratePath
} from 'utils/identitiesUtils';
import { alertDeleteAccount, alertPathDeletionError } from 'utils/alertUtils';
import { navigateToPathsList, unlockSeedPhrase } from 'utils/navigationHelpers';
import { generateAccountId } from 'utils/account';
import UnknownAccountWarning from 'components/UnknownAccountWarning';

interface Props {
	path: string;
	networkKey: string;
	navigation:
		| StackNavigationProp<RootStackParamList, 'PathDetails'>
		| StackNavigationProp<RootStackParamList, 'PathsList'>;
	accounts: AccountsStore;
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
	if (!address) return <View />;
	const isUnknownNetwork = networkKey === UnknownNetworkKeys.UNKNOWN;
	const formattedNetworkKey = isUnknownNetwork ? defaultNetworkKey : networkKey;
	const accountId = generateAccountId({
		address,
		networkKey: formattedNetworkKey
	});

	const onOptionSelect = (value: string): void => {
		switch (value) {
			case 'PathDelete':
				alertDeleteAccount('this account', async () => {
					await unlockSeedPhrase(navigation);
					const deleteSucceed = await accounts.deletePath(path);
					const paths = Array.from(accounts.state.currentIdentity!.meta.keys());
					const pathIndicatedNetworkKey = getNetworkKeyByPath(path);
					if (deleteSucceed) {
						if (isSubstratePath(path)) {
							const listedPaths = getPathsWithSubstrateNetworkKey(
								paths,
								pathIndicatedNetworkKey
							);
							const hasOtherPaths = listedPaths.length > 0;
							hasOtherPaths
								? navigateToPathsList(navigation, pathIndicatedNetworkKey)
								: navigation.navigate('AccountNetworkChooser');
						} else {
							navigation.navigate('AccountNetworkChooser');
						}
					} else {
						alertPathDeletionError();
					}
				});
				break;
			case 'PathDerivation':
				navigation.navigate('PathDerivation', { parentPath: path });
				break;
			case 'PathManagement':
				navigation.navigate('PathManagement', { path });
				break;
		}
	};

	return (
		<SafeAreaScrollViewContainer testID={testIDs.PathDetail.screen}>
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
								hide: !isSubstratePath(path),
								text: 'Derive Account',
								value: 'PathDerivation'
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
			<PathCard identity={currentIdentity!} path={path} />
			<QrView data={`${accountId}:${accountName}`} />
			{isUnknownNetwork && <UnknownAccountWarning isPath />}
		</SafeAreaScrollViewContainer>
	);
}

function PathDetails({
	accounts,
	navigation,
	route
}: NavigationAccountProps<'PathDetails'>): React.ReactElement {
	const path = route.params.path ?? '';
	const networkKey = getNetworkKey(path, accounts.state.currentIdentity!);
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
		color: colors.bg_alert
	}
});

export default withAccountStore(PathDetails);
