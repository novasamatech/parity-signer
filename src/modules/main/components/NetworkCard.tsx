// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import { StackNavigationProp } from '@react-navigation/stack';
import { useNavigation } from '@react-navigation/core';
import React, { ReactElement, useContext } from 'react';
import { StyleSheet, View, Text } from 'react-native';

import { NetworksContext } from 'stores/NetworkContext';
import { AlertStateContext } from 'stores/alertContext';
import { AccountsContext } from 'stores/AccountsContext';

import AccountIcon from 'components/AccountIcon';
import Button from 'components/Button';
import TouchableItem from 'components/TouchableItem';
import AccountPrefixedTitle from 'components/AccountPrefixedTitle';
import Separator from 'components/Separator';
import PopupMenu from 'components/PopupMenu';

import { AccountsStoreStateWithIdentity } from 'types/identityTypes';
import { ButtonListener } from 'types/props';
import { RootStackParamList } from 'types/routes';
import { isSubstrateNetworkParams, NetworkParams } from 'types/networkTypes';

import { alertDeleteAccount, alertError } from 'utils/alertUtils';
import { resetNavigationTo, navigateToReceiveBalance, navigateToSendBalance } from 'utils/navigationHelpers';

export function NetworkCard({
	networkKey,
	onPress,
	testID,
	title
}: {
	networkKey?: string;
	onPress?: ButtonListener;
	testID?: string;
	title: string;
}): ReactElement {
	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();
	const networksContextState = useContext(NetworksContext);
	const networkParams = networksContextState.getNetwork(networkKey ?? '');
	const { setAlert } = useContext(AlertStateContext);
	const accountsStore = useContext(AccountsContext);

	const onPressed = async (isSend: boolean): Promise<void> => {
		if (isSubstrateNetworkParams(networkParams)) {
			// navigate to substrate account
			const { pathId } = networkParams;
			const fullPath = `//${pathId}`;
			if (isSend) {
				navigateToSendBalance(navigation, networkKey ?? '', fullPath);
			} else {
				navigateToReceiveBalance(navigation, networkKey ?? '', fullPath);
			}
		} else {
			// navigate to ethereum account
			if (isSend) {
				navigateToSendBalance(navigation, networkKey ?? '', networkKey ?? '');
			} else {
				navigateToReceiveBalance(
					navigation,
					networkKey ?? '',
					networkKey ?? ''
				);
			}
		}
	};
	const onOptionSelect = async (value: string): Promise<void> => {
		switch (value) {
			case 'PathDelete':
				const { pathId } = networkParams;
				if (pathId) {
					accountsStore.deleteSubstratePath(`//${pathId}`, networksContextState);
                                } else {
					accountsStore.deleteEthereumAddress(networkKey);
                                }
				resetNavigationTo(navigation, 'Wallet');
				break;
		}
	};

	const isDisabled = onPress === undefined;
	return (
		<View>
                        <Separator
                                style={{
                                        backgroundColor: 'transparent',
                                        height: 0,
                                        marginVertical: 0
                                }}
                        />
			<View style={styles.content}>
				<AccountIcon address={''} network={networkParams} style={styles.icon} />
				<View style={styles.desc}>
					<AccountPrefixedTitle title={title} />
				</View>
			</View>
			<View style={styles.content}>
				<Text style={styles.text}>0</Text>
			</View>
			<View style={styles.content}>
				<Button
					title="Send"
					onPress={(): Promise<void> => onPressed(true)}
					small={true}
				/>
				<Button
					title="Receive"
					onPress={(): Promise<void> => onPressed(false)}
					small={true}
				/>
				<Button title="Add" small={true} />
				<PopupMenu
					onSelect={onOptionSelect}
					menuTriggerIconName={'more-vert'}
					menuItems={[
						{
							text: `Remove ${title}`,
							textStyle: styles.deleteText,
							value: 'PathDelete'
						}
					]}
				/>
			</View>
		</View>
	);
}

const styles = StyleSheet.create({
	content: {
		alignItems: 'center',
		flexDirection: 'row',
		display: 'flex',
		paddingLeft: 16
	},
	text: {
		color: '#fff',
	},
	desc: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'space-between',
		paddingLeft: 16
	},
	footer: {
		alignSelf: 'stretch',
		height: 80,
		marginLeft: 8,
		width: 4
	},
	icon: {
		height: 40,
		width: 40
	}
});
