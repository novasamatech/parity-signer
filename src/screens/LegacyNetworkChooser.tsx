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

import React from 'react';
import { StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import {
	NETWORK_LIST,
	UnknownNetworkKeys,
	SubstrateNetworkKeys
} from 'constants/networkSpecs';
import { NetworkParams } from 'types/networkSpecsTypes';
import { NavigationAccountProps, NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import TouchableItem from 'components/TouchableItem';
import AccountsStore from 'stores/AccountsStore';
import { emptyAccount } from 'utils/account';

export default class LegacyNetworkChooser extends React.PureComponent<
	NavigationProps<'LegacyNetworkChooser'>,
	{}
> {
	render(): React.ReactElement {
		return (
			<Subscribe to={[AccountsStore]}>
				{(accounts: AccountsStore): React.ReactElement => (
					<LegacyNetworkChooserView {...this.props} accounts={accounts} />
				)}
			</Subscribe>
		);
	}
}

class LegacyNetworkChooserView extends React.PureComponent<
	NavigationAccountProps<'LegacyNetworkChooser'>,
	{}
> {
	render(): React.ReactElement {
		const { navigation, accounts } = this.props;
		const excludedNetworks = [UnknownNetworkKeys.UNKNOWN];

		if (!__DEV__) {
			excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
			excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
		}

		return (
			<SafeAreaScrollViewContainer contentContainerStyle={{ padding: 20 }}>
				<Text style={styles.title}>CHOOSE NETWORK</Text>
				{Object.entries(NETWORK_LIST)
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
									accounts.updateNew(emptyAccount('', networkKey));
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
}

const styles = StyleSheet.create({
	bottom: {
		flexBasis: 50,
		paddingBottom: 15
	},
	card: {
		backgroundColor: colors.card_bg,
		padding: 20
	},
	cardText: {
		color: colors.card_text,
		fontFamily: fonts.bold,
		fontSize: 20
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	},
	titleTop: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	},
	top: {
		flex: 1
	}
});
