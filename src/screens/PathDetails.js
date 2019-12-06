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
import ScreenHeading from '../components/ScreenHeading';
import colors from '../colors';
import QrView from '../components/QrView';
import {
	getAddressWithPath,
	getNetworkKeyByPath,
	isSubstratePath
} from '../util/identitiesUtils';
import { UnknownNetworkKeys } from '../constants';
import { alertDeleteAccount, alertPathDeletionError } from '../util/alertUtils';
import {
	navigateToPathsList,
	unlockSeedPhrase
} from '../util/navigationHelpers';
import testIDs from '../../e2e/testIDs';
import { generateAccountId } from '../util/account';
import UnknownAccountWarning from '../components/UnknownAccountWarning';

export function PathDetailsView({ accounts, navigation, path, networkKey }) {
	const { currentIdentity } = accounts.state;
	const address = getAddressWithPath(path, currentIdentity);
	if (!address) return null;
	const accountId = generateAccountId({
		address,
		networkKey: getNetworkKeyByPath(path)
	});

	const onOptionSelect = value => {
		if (value === 'PathDelete') {
			alertDeleteAccount('this key pairs', async () => {
				await unlockSeedPhrase(navigation);
				const deleteSucceed = await accounts.deletePath(path);
				if (deleteSucceed) {
					isSubstratePath(path)
						? navigateToPathsList(navigation, networkKey)
						: navigation.navigate('AccountNetworkChooser');
				} else {
					alertPathDeletionError();
				}
			});
		} else {
			navigation.navigate('PathManagement', { path });
		}
	};

	return (
		<View style={styles.body}>
			<ScreenHeading small={true} title="Public Address" />
			<View style={styles.menuView}>
				<PopupMenu
					testID={testIDs.PathDetail.popupMenuButton}
					onSelect={onOptionSelect}
					menuTriggerIconName={'more-vert'}
					menuItems={[
						{ text: 'Edit', value: 'PathManagement' },
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
				<PathCard identity={currentIdentity} path={path} />
				{networkKey === UnknownNetworkKeys.UNKNOWN ? (
					<>
						<QrView data={address} />
						<UnknownAccountWarning />
					</>
				) : (
					<QrView data={accountId} />
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
