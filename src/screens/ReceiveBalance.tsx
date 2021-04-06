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
import { ScrollView, View } from 'react-native';

import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import { NetworksContext } from 'stores/NetworkContext';
import { AccountsStoreStateWithIdentity } from 'types/identityTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { RootStackParamList } from 'types/routes';
import { withCurrentIdentity } from 'utils/HOC';
import {
	getAddressWithPath,
	getNetworkKey,
	getPathName
} from 'utils/identitiesUtils';
import { generateAccountId } from 'utils/account';
import { UnknownAccountWarning } from 'components/Warnings';
import PathCard from 'components/PathCard';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { ScreenHeadingWithNetworkIcon } from 'components/ScreenHeading';
import QrView from 'components/QrView';

interface Props {
	path: string;
	networkKey: string;
	navigation: StackNavigationProp<RootStackParamList, 'ReceiveBalance'>;
	accountsStore: AccountsStoreStateWithIdentity;
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
	const { currentIdentity } = accountsStore.state;
	const address = getAddressWithPath(path, currentIdentity);
	const accountName = getPathName(path, currentIdentity);
	const { allNetworks } = networksContextState;
	if (!address) return <View />;
	const isUnknownNetwork = networkKey === UnknownNetworkKeys.UNKNOWN;
	const formattedNetworkKey = isUnknownNetwork ? defaultNetworkKey : networkKey;
	const accountId = generateAccountId(
		address,
		formattedNetworkKey,
		allNetworks
	);

	return (
		<SafeAreaViewContainer>
			<ScrollView bounces={false}>
				<ScreenHeadingWithNetworkIcon
					title="Receive Balance"
					networkKey={formattedNetworkKey}
				/>
				<PathCard identity={currentIdentity} path={path} />
				<QrView data={`${accountId}:${accountName}`} />
				{isUnknownNetwork && <UnknownAccountWarning isPath />}
			</ScrollView>
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(ReceiveBalance);
