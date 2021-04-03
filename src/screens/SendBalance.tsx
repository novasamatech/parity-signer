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
import { ScrollView, View, Text } from 'react-native';

import { NetworksContext } from 'stores/NetworkContext';

import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import { AccountsStoreStateWithIdentity } from 'types/identityTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { RootStackParamList } from 'types/routes';

import { withCurrentIdentity } from 'utils/HOC';
import { getNetworkKey } from 'utils/identitiesUtils';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { LeftScreenHeading } from 'components/ScreenHeading';

interface Props {
	path: string;
	networkKey: string;
	navigation: StackNavigationProp<RootStackParamList, 'SendBalance'>;
	accountsStore: AccountsStoreStateWithIdentity;
}

function SendBalance({
  accountsStore,
  navigation,
  route,
}: NavigationAccountIdentityProps<'SendBalance'>): React.ReactElement {
	const path = route.params.path;
	const networksContextState = useContext(NetworksContext);
	const networkKey = getNetworkKey(
		path,
		accountsStore.state.currentIdentity,
		networksContextState
	);
	const isUnknownNetwork = networkKey === UnknownNetworkKeys.UNKNOWN;
	const formattedNetworkKey = isUnknownNetwork ? defaultNetworkKey : networkKey;

	return (
		<SafeAreaViewContainer>
			<ScrollView bounces={false}>
				<LeftScreenHeading
					title="Send Balance"
					networkKey={formattedNetworkKey}
				/>
				<View>
					<Text>To be implemented</Text>
				</View>
			</ScrollView>
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(SendBalance);
