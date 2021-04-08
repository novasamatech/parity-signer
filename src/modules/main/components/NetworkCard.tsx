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
import { AccountsContext } from 'stores/AccountsContext';
import AccountIcon from 'components/AccountIcon';
import Button from 'components/Button';
import PopupMenu from 'components/PopupMenu';
import { ButtonListener } from 'types/props';
import { RootStackParamList } from 'types/routes';
import { isSubstrateNetworkParams } from 'types/networkTypes';
import { colors, fonts, fontStyles } from 'styles/index';
import {
	resetNavigationTo,
	navigateToReceiveBalance,
	navigateToSendBalance
} from 'utils/navigationHelpers';

export function NetworkCard({
	networkKey,
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
				if (isSubstrateNetworkParams(networkParams)) {
					const { pathId } = networkParams;
					accountsStore.deleteSubstratePath(
						`//${pathId}`,
						networksContextState
					);
				} else {
					accountsStore.deleteEthereumAddress(networkKey);
				}
				resetNavigationTo(navigation, 'Wallet');
				break;
			case 'SignTransaction':
				navigation.navigate('SignTransaction');
				break;
		}
	};

	return (
		<View style={styles.wrapper}>
			<View style={styles.content}>
				<View style={styles.contentRow}>
					<AccountIcon
						address={''}
						network={networkParams}
						style={styles.icon}
					/>
					<View style={styles.desc}>
						<Text numberOfLines={1} style={[fontStyles.h2, { marginTop: -2 }]}>
							{title}
						</Text>
					</View>
					<View style={styles.contentRow}>
						<Text style={styles.text}>0 {networkParams.unit}</Text>
					</View>
					<PopupMenu
						onSelect={onOptionSelect}
						menuTriggerIconName={'more-vert'}
						menuItems={[
							{
								text: 'Sign transaction',
								value: 'SignTransaction'
							},
							{
								text: 'Remove this network',
								value: 'PathDelete'
							}
						]}
					/>
				</View>
				<View style={styles.contentRow}>
					<View style={styles.contentColumn}>
						<Button
							title="Send"
							fluid={true}
							onPress={(): Promise<void> => onPressed(true)}
						/>
					</View>
					<View style={styles.contentColumn}>
						<Button
							title="Receive"
							fluid={true}
							onPress={(): Promise<void> => onPressed(false)}
						/>
					</View>
				</View>
			</View>
		</View>
	);
}

const styles = StyleSheet.create({
	content: {
		backgroundColor: colors.background.accentLight,
		borderRadius: 16,
		marginBottom: 12,
		paddingBottom: 12,
		paddingHorizontal: 24,
		paddingTop: 20
	},
	contentColumn: {
		flex: 1
	},
	contentRow: {
		alignItems: 'center',
		display: 'flex',
		flexDirection: 'row'
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
	},
	text: {
		color: colors.text.dark,
		fontFamily: fonts.regular,
		fontSize: 17
	},
	wrapper: {
		marginHorizontal: 16
	}
});
