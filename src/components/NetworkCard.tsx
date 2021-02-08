// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import React, { ReactElement, useContext } from 'react';
import { StyleSheet, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import { NetworksContext } from 'stores/NetworkContext';
import colors from 'styles/colors';
import { ButtonListener } from 'types/props';

import AccountIcon from './AccountIcon';
import AccountPrefixedTitle from './AccountPrefixedTitle';
import { CardSeparator } from './CardSeparator';
import { NetworkFooter } from './NetworkFooter';
import TouchableItem from './TouchableItem';

interface NetworkCardProps {
	isAdd?: boolean;
	networkColor?: string;
	networkKey?: string;
	onPress?: ButtonListener;
	testID?: string;
	title: string;
}

export function NetworkCard({ isAdd, networkColor, networkKey, onPress, testID, title }: NetworkCardProps): ReactElement {
	const { getNetwork } = useContext(NetworksContext);
	const networkParams = getNetwork(networkKey ?? '');
	const isDisabled = onPress === undefined;

	return (
		<TouchableItem disabled={isDisabled}
			onPress={onPress}
			testID={testID}>
			<CardSeparator />
			<View style={styles.content}>
				{isAdd ? (
					<View
						style={{
							alignItems: 'center',
							height: 40,
							justifyContent: 'center',
							width: 40
						}}
					>
						<Icon color={colors.text.main}
							name="add"
							size={30} />
					</View>
				) : (
					<AccountIcon
						address={''}
						network={networkParams}
						style={styles.icon}
					/>
				)}
				<View style={styles.desc}>
					<AccountPrefixedTitle title={title} />
				</View>
				<NetworkFooter color={networkColor ?? networkParams.color} />
			</View>
		</TouchableItem>
	);
}

const styles = StyleSheet.create({
	content: {
		alignItems: 'center',
		flexDirection: 'row',
		paddingLeft: 16
	},
	desc: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'space-between',
		paddingLeft: 16
	},
	icon: {
		height: 40,
		width: 40
	}
});
