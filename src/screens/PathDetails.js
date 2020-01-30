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

'use strict';

import React from 'react';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { ScrollView, StyleSheet, View } from 'react-native';
import PathCard from '../components/PathCard';
import PopupMenu from '../components/PopupMenu';
import { PathCardHeading } from '../components/ScreenHeading';
import colors from '../colors';
import QrView from '../components/QrView';
import {
	getAddressWithPath,
	getIdentityName,
	getNetworkKeyByPath,
	getPathName,
	getPathsWithSubstrateNetwork,
	isSubstratePath
} from '../util/identitiesUtils';
import { defaultNetworkKey, UnknownNetworkKeys } from '../constants';
import { alertDeleteAccount, alertPathDeletionError } from '../util/alertUtils';
import {
	navigateToPathsList,
	unlockSeedPhrase
} from '../util/navigationHelpers';
import testIDs from '../../e2e/testIDs';
import { generateAccountId } from '../util/account';
import UnknownAccountWarning from '../components/UnknownAccountWarning';
import AccountCard from '../components/AccountCard';

export function PathDetailsView({ accounts, navigation, path, networkKey }) {
	const { currentIdentity } = accounts.state;
	const address = getAddressWithPath(path, currentIdentity);
	const pathName = getPathName(path, currentIdentity);
	if (!address) return null;
	const accountId = generateAccountId({
		address,
		networkKey: getNetworkKeyByPath(path)
	});
	const isUnknownNetwork = networkKey === UnknownNetworkKeys.UNKNOWN;
	//TODO enable user to select networkKey.
	const isRootPath = path === '';
	const formattedNetworkKey = isUnknownNetwork ? defaultNetworkKey : networkKey;

	const onOptionSelect = value => {
		switch (value) {
			case 'PathDelete':
				alertDeleteAccount('this account', async () => {
					await unlockSeedPhrase(navigation);
					const deleteSucceed = await accounts.deletePath(path);
					const paths = Array.from(accounts.state.currentIdentity.meta.keys());
					const listedPaths = getPathsWithSubstrateNetwork(paths, networkKey);
					const hasOtherPaths = listedPaths.length > 0;
					if (deleteSucceed) {
						isSubstratePath(path) && !isRootPath && hasOtherPaths
							? navigateToPathsList(navigation, networkKey)
							: navigation.navigate('AccountNetworkChooser');
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
		<View style={styles.body} testID={testIDs.PathDetail.screen}>
			<PathCardHeading
				title="Public Address"
				networkKey={formattedNetworkKey}
			/>
			<View style={styles.menuView}>
				<PopupMenu
					testID={testIDs.PathDetail.popupMenuButton}
					onSelect={onOptionSelect}
					menuTriggerIconName={'more-vert'}
					menuItems={[
						{ hide: isUnknownNetwork, text: 'Edit', value: 'PathManagement' },
						{ text: 'Derive Account', value: 'PathDerivation' },
						{
							testID: testIDs.PathDetail.deleteButton,
							text: 'Delete',
							textStyle: styles.deleteText,
							value: 'PathDelete'
						}
					]}
				/>
			</View>
			<ScrollView>
				{isUnknownNetwork ? (
					<>
						<AccountCard
							title={
								isRootPath
									? getIdentityName(currentIdentity, accounts.state.identities)
									: getPathName(path, currentIdentity)
							}
							address={address}
							networkKey={formattedNetworkKey}
						/>
						<QrView data={generateAccountId({ address, networkKey })} />
						<UnknownAccountWarning isPath />
					</>
				) : (
					<>
						<PathCard identity={currentIdentity} path={path} />
						<QrView data={`${accountId}:${pathName}`} />
					</>
				)}
			</ScrollView>
		</View>
	);
}

function PathDetails({ accounts, navigation }) {
	const path = navigation.getParam('path', '');
	const networkKey = getNetworkKeyByPath(path);
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
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column'
	},
	deleteText: {
		color: colors.bg_alert
	},
	menuView: {
		alignItems: 'flex-end',
		flex: 1,
		position: 'absolute',
		right: 16,
		top: 5
	}
});

export default withAccountStore(withNavigation(PathDetails));
