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

import React from 'react';
import { Text } from 'react-native';
import Clipboard from '@react-native-community/clipboard';

import { fontStyles } from 'styles';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NavigationProps } from 'types/props';
import TouchableItem from 'components/TouchableItem';
import ScreenHeading from 'components/ScreenHeading';

function ShowRecoveryPhrase({
	route
}: NavigationProps<'ShowRecoveryPhrase'>): React.ReactElement {
	return (
		<SafeAreaViewContainer>
			<ScreenHeading title={'Show Key Phrase'} />
			<Text>
				Write these words down on paper and keep them somewhere secure. These
				words allow anyone to recover this account and access its funds.
			</Text>
			<TouchableItem
				onPress={(): void => {
					// only allow the copy of the key phrase in dev environment
					if (__DEV__) {
						Clipboard.setString(route.params.seedPhrase);
					}
				}}
			>
				<Text style={[fontStyles.t_seed, { marginHorizontal: 16 }]}>
					{route.params.seedPhrase}
				</Text>
			</TouchableItem>
		</SafeAreaViewContainer>
	);
}

export default ShowRecoveryPhrase;
