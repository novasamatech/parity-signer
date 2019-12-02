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
import { Alert, ScrollView, StyleSheet, View } from 'react-native';
import PathCard from '../components/PathCard';
import PopupMenu from '../components/PopupMenu';
import ScreenHeading from '../components/ScreenHeading';
import colors from '../colors';
import QrView from '../components/QrView';
import {
	getAccountIdWithPath,
	getNetworkKeyByPath,
	isSubstratePath,
	unlockIdentitySeedWithBiometric
} from '../util/identitiesUtils';
import { UnknownNetworkKeys } from '../constants';
import { alertDeleteAccount, alertPathDeletionError } from '../util/alertUtils';
import {
	navigateToPathsList,
	unlockPin,
	unlockSeedPhrase
} from '../util/navigationHelpers';
import testIDs from '../../e2e/testIDs';

export function PathDetailsView({ accounts, navigation, path, networkKey }) {
	const { currentIdentity } = accounts.state;
	const address = getAccountIdWithPath(path, currentIdentity);

	async function onDelete() {
		const deleteSucceed = await accounts.deletePath(path);
		if (deleteSucceed) {
			isSubstratePath(path)
				? navigateToPathsList(navigation, networkKey)
				: navigation.navigate('AccountNetworkChooser');
		} else {
			alertPathDeletionError();
		}
	}

	async function noBiometric(value) {
		try {
			if (value === 'PathDelete') {
				alertDeleteAccount('this key pairs', async () => {
					await unlockSeedPhrase(navigation);
					await onDelete();
				});
			} else if (false) {
				const pin = await unlockPin(navigation);
				navigation.pop();
				if (currentIdentity.biometricEnabled) {
					// we can reach here if if biometric is enabled but failed for some reason, eg. if fingerprints were invalidated
					await accounts.identityDisableBiometric().catch(() => {
						// errors already handled
					});
				} else {
					await accounts.identityEnableBiometric(pin).catch(error => {
						// error here is likely no fingerprints/biometrics enrolled, so should be displayed to the user
						Alert.alert('Biometric Error', error.message, [
							{
								style: 'default',
								text: 'Ok'
							}
						]);
					});
				}
			} else {
				// impossible!
			}
		} catch (e) {}
	}

	async function withBiometric(value) {
		try {
			if (value === 'PathDelete') {
				alertDeleteAccount('this key pairs', async () => {
					await unlockIdentitySeedWithBiometric(currentIdentity)
						.then(onDelete)
						.catch(() => {
							unlockSeedPhrase(navigation).then(onDelete);
						});
				});
			} else if (false) {
				await unlockIdentitySeedWithBiometric(currentIdentity);
				await accounts.identityDisableBiometric();
			} else {
				// impossible!
			}
		} catch (e) {
			Alert.alert('Biometric Error', e.message, [
				{
					onDismiss: async () => {
						await noBiometric(value);
					},
					onPress: async () => {
						await noBiometric(value);
					},
					style: 'default',
					text: 'Ok'
				}
			]);
		}
	}

	const onOptionSelect = async value => {
		if (value !== 'PathManagement') {
			if (currentIdentity.biometricEnabled) {
				await withBiometric(value);
			} else {
				await noBiometric(value);
			}
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
				{networkKey !== UnknownNetworkKeys.UNKNOWN && address !== '' && (
					<QrView data={address} />
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
