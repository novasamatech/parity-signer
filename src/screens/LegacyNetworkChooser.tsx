// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import React, { useContext } from 'react';
import { StyleSheet, Text } from 'react-native';

import { AccountsContext } from 'stores/AccountsContext';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import {
	UnknownNetworkKeys,
	SubstrateNetworkKeys
} from 'constants/networkSpecs';
import { NetworksContext } from 'stores/NetworkContext';
import { NetworkParams } from 'types/networkTypes';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import TouchableItem from 'components/TouchableItem';
import { emptyAccount } from 'utils/account';

export default function LegacyNetworkChooserView({
	navigation
}: NavigationProps<'LegacyNetworkChooser'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { allNetworks } = useContext(NetworksContext);
	const excludedNetworks = [UnknownNetworkKeys.UNKNOWN];

	if (!__DEV__) {
		excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
		excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
	}

	return (
		<SafeAreaScrollViewContainer contentContainerStyle={{ padding: 20 }}>
			<Text style={styles.title}>CHOOSE NETWORK</Text>
			{Array.from(allNetworks.entries())
				.filter(
					([networkKey]: [string, any]): boolean =>
						!excludedNetworks.includes(networkKey)
				)
				.map(
					([networkKey, networkParams]: [
						string,
						NetworkParams
					]): React.ReactElement => (
						<TouchableItem
							key={networkKey}
							style={[
								styles.card,
								{
									backgroundColor: networkParams.color,
									marginTop: 20
								}
							]}
							onPress={(): void => {
								accountsStore.updateNew(emptyAccount('', networkKey));
								navigation.goBack();
							}}
						>
							<Text
								style={[
									styles.cardText,
									{
										color: networkParams.secondaryColor
									}
								]}
							>
								{networkParams.title}
							</Text>
						</TouchableItem>
					)
				)}
		</SafeAreaScrollViewContainer>
	);
}

const styles = StyleSheet.create({
	bottom: {
		flexBasis: 50,
		paddingBottom: 15
	},
	card: {
		backgroundColor: colors.background.card,
		padding: 20
	},
	cardText: {
		color: colors.background.app,
		fontFamily: fonts.bold,
		fontSize: 20
	},
	title: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	},
	titleTop: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	},
	top: {
		flex: 1
	}
});
