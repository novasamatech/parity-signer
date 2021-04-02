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
import React, { useContext } from 'react';
import { ScrollView, StyleSheet, View } from 'react-native';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { AlertStateContext } from 'stores/alertContext';
import { NetworksContext } from 'stores/NetworkContext';
// TODO use typescript 3.8's type import, Wait for prettier update.
import { AccountsStoreStateWithIdentity } from 'types/identityTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { RootStackParamList } from 'types/routes';
import PathCard from 'components/PathCard';
import PopupMenu from 'components/PopupMenu';
import { LeftScreenHeading } from 'components/ScreenHeading';
import colors from 'styles/colors';
import QrView from 'components/QrView';
import { withCurrentIdentity } from 'utils/HOC';
import {
	getAddressWithPath,
	getNetworkKey,
	getPathName
} from 'utils/identitiesUtils';
import { alertDeleteAccount, alertError } from 'utils/alertUtils';
import { generateAccountId } from 'utils/account';
import { UnknownAccountWarning } from 'components/Warnings';
import { resetNavigationTo } from 'utils/navigationHelpers';

interface Props {
	path: string;
	networkKey: string;
	navigation: StackNavigationProp<RootStackParamList, 'ReceiveBalance'>;
	accountsStore: AccountsStoreStateWithIdentity;
}

function PathDetailsView({
	accountsStore,
	navigation,
	path,
	networkKey
}: Props): React.ReactElement {
	const { currentIdentity } = accountsStore.state;
	const address = getAddressWithPath(path, currentIdentity);
	const accountName = getPathName(path, currentIdentity);
	const { setAlert } = useContext(AlertStateContext);
	const networksContextState = useContext(NetworksContext);
	const { allNetworks } = networksContextState;
	if (!address) return <View />;
	const isUnknownNetwork = networkKey === UnknownNetworkKeys.UNKNOWN;
	const formattedNetworkKey = isUnknownNetwork ? defaultNetworkKey : networkKey;
	const accountId = generateAccountId(
		address,
		formattedNetworkKey,
		allNetworks
	);

	const onOptionSelect = async (value: string): Promise<void> => {
		switch (value) {
			case 'PathDelete':
				alertDeleteAccount(setAlert, 'this account', async () => {
					try {
						accountsStore.deletePath(path, networksContextState);
						resetNavigationTo(navigation, 'Main');
					} catch (err) {
						alertError(
							setAlert,
							`Can't delete this account: ${err.toString()}`
						);
					}
				});
				break;
		}
	};

	return (
		<SafeAreaViewContainer>
			<ScrollView testID={testIDs.PathDetail.screen} bounces={false}>
				<LeftScreenHeading
					title="Receive Balance"
					networkKey={formattedNetworkKey}
					headMenu={
						<PopupMenu
							testID={testIDs.PathDetail.popupMenuButton}
							onSelect={onOptionSelect}
							menuTriggerIconName={'more-vert'}
							menuItems={[
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
				<PathCard
					identity={currentIdentity}
					path={path}
				/>
				<QrView data={`${accountId}:${accountName}`} />
				{isUnknownNetwork && <UnknownAccountWarning isPath />}
			</ScrollView>
		</SafeAreaViewContainer>
	);
}

function ReceiveBalance({
	accountsStore,
	navigation,
	route
}: NavigationAccountIdentityProps<'ReceiveBalance'>): React.ReactElement {
	const path = route.params.path;
	const networksContextState = useContext(NetworksContext);
	const networkKey = getNetworkKey(
		path,
		accountsStore.state.currentIdentity,
		networksContextState
	);
	return (
		<PathDetailsView
			accountsStore={accountsStore}
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

export default withCurrentIdentity(ReceiveBalance);
