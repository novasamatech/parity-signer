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
import React, { ReactElement, useContext, useEffect, useState } from 'react';
import { StyleSheet, View, Text } from 'react-native';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { TypeRegistry } from '@polkadot/types';
import { spec } from '@edgeware/node-types';
import BN from 'bn.js';

import { NetworksContext } from 'stores/NetworkContext';
import { AccountsContext } from 'stores/AccountsContext';
import AccountIcon from 'components/AccountIcon';
import Button from 'components/Button';
import AccountPrefixedTitle from 'components/AccountPrefixedTitle';
import Separator from 'components/Separator';
import PopupMenu from 'components/PopupMenu';
import { ButtonListener } from 'types/props';
import { RootStackParamList } from 'types/routes';
import { isSubstrateNetworkParams } from 'types/networkTypes';
import {
	resetNavigationTo,
	navigateToReceiveBalance,
	navigateToSendBalance
} from 'utils/navigationHelpers';
import { getAddressWithPath } from 'utils/identitiesUtils';

export function NetworkCard({
	networkKey,
	title,
	path
}: {
	networkKey?: string;
	onPress?: ButtonListener;
	testID?: string;
	title: string;
	path: string;
}): ReactElement {
	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();
	const networksContextState = useContext(NetworksContext);
	const networkParams = networksContextState.getNetwork(networkKey ?? '');
	const accountsStore = useContext(AccountsContext);
	const { currentIdentity } = accountsStore.state;
	const address = getAddressWithPath(path, currentIdentity);
	const [balance, setBalance] = useState('LOADING...');

	useEffect(() => {
		let api: ApiPromise | null = null;
		const fetchBalance = async (): Promise<void> => {
			if (isSubstrateNetworkParams(networkParams)) {
				const provider = new WsProvider(networkParams.url);
				// TODO: get types for each network
				// TODO: load metadata at startup
				// TODO: handle errors
				// TODO: make this stateful so we don't have to reload every time we come here
				const registry = new TypeRegistry();
				console.log(`CREATING API: ${networkParams.url}`);
				api = await ApiPromise.create({ provider, registry, ...spec });
				const balances = await api.query.balances.account(address);
				console.log('DISCONNECTING API');
				await api.disconnect();
				const base = new BN(10).pow(new BN(networkParams.decimals));
				const div = balances.free.div(base);
				const mod = balances.free.mod(base);
				setBalance(div + '.' + mod.toString(10, networkParams.decimals));
			} else {
				// TODO
				setBalance('0');
			}
		};

		fetchBalance();

		// cleanup
		// return (): void => {
		// 	if (api?.isConnected) {
		// 		api.disconnect();
		// 	}
		// };
	});

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
		}
	};

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
				<Text style={styles.text}>{balance}</Text>
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
				<Button title="Add" small={true} onPress={(): void => void 0} />
				<PopupMenu
					onSelect={onOptionSelect}
					menuTriggerIconName={'more-vert'}
					menuItems={[
						{
							text: `Remove ${title}`,
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
		display: 'flex',
		flexDirection: 'row',
		paddingLeft: 16
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
		color: '#fff'
	}
});
