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

// @flow

'use strict';

import PropTypes from 'prop-types';
import React from 'react';
import { StyleSheet, Text, View, ViewPropTypes } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import Separator from '../components/Separator';
import AccountIcon from './AccountIcon';
import Address from './Address';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import fontStyles from '../fontStyles';
import TouchableItem from './TouchableItem';
import colors from '../colors';
import AccountPrefixedTitle from './AccountPrefixedTitle';

const CardSeparator = () => (
	<Separator
		shadow={true}
		style={{
			backgroundColor: 'transparent',
			height: 0,
			marginVertical: 0
		}}
	/>
);

const NetworkFooter = ({ networkColor, network }) => (
	<View
		style={[
			styles.footer,
			{
				backgroundColor: networkColor || network.color
			}
		]}
	/>
);

NetworkCard.propTypes = {
	isAdd: PropTypes.bool,
	networkColor: PropTypes.string,
	networkKey: PropTypes.string,
	onPress: PropTypes.func.isRequired,
	testID: PropTypes.string,
	title: PropTypes.string.isRequired
};

export function NetworkCard({
	isAdd,
	networkColor,
	networkKey,
	onPress,
	testID,
	title
}) {
	const network =
		NETWORK_LIST[networkKey] || NETWORK_LIST[NetworkProtocols.UNKNOWN];

	return (
		<TouchableItem
			accessibilityComponentType="button"
			testID={testID}
			disabled={false}
			onPress={onPress}
		>
			<CardSeparator />
			<View style={styles.content}>
				{isAdd ? (
					<View style={{ height: 40, width: 40 }}>
						<Icon name="add" color={colors.bg_text} size={32} />
					</View>
				) : (
					<AccountIcon
						address={''}
						protocol={network.protocol}
						network={network}
						style={styles.icon}
					/>
				)}
				<View style={styles.desc}>
					<AccountPrefixedTitle title={title} />
				</View>
				<NetworkFooter network={network} networkColor={networkColor} />
			</View>
		</TouchableItem>
	);
}

AccountCard.propTypes = {
	address: PropTypes.string,
	networkKey: PropTypes.string,
	onPress: PropTypes.func,
	seedType: PropTypes.string,
	style: ViewPropTypes.style,
	testID: PropTypes.string,
	title: PropTypes.string,
	titlePrefix: PropTypes.string
};

AccountCard.defaultProps = {
	onPress: () => {},
	title: 'No name'
};

export default function AccountCard({
	address,
	networkKey,
	networkColor,
	onPress,
	seedType,
	style,
	testID,
	title,
	titlePrefix
}) {
	const displayTitle = title.length ? title : AccountCard.defaultProps.title;
	const seedTypeDisplay = seedType || '';
	const network =
		NETWORK_LIST[networkKey] || NETWORK_LIST[NetworkProtocols.UNKNOWN];

	return (
		<TouchableItem
			accessibilityComponentType="button"
			testID={testID}
			disabled={false}
			onPress={onPress}
		>
			<CardSeparator />
			<View style={[styles.content, style]}>
				<AccountIcon
					address={address}
					protocol={network.protocol}
					network={network}
					style={styles.icon}
				/>
				<View style={styles.desc}>
					<View>
						<Text style={[fontStyles.t_regular, { color: colors.bg_text_sec }]}>
							{`${network.title}${seedTypeDisplay} `}
						</Text>
					</View>
					<AccountPrefixedTitle
						title={displayTitle}
						titlePrefix={titlePrefix}
					/>
					{address !== '' && (
						<Address address={address} protocol={network.protocol} />
					)}
				</View>
				<NetworkFooter network={network} networkColor={networkColor} />
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
	footer: {
		alignSelf: 'stretch',
		height: 88,
		marginLeft: 8,
		width: 8
	},
	icon: {
		height: 40,
		width: 40
	}
});
