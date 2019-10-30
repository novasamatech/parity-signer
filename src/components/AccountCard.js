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

import PropTypes from 'prop-types';
import React from 'react';
import { StyleSheet, Text, View, ViewPropTypes } from 'react-native';

import AccountIcon from './AccountIcon';
import Address from './Address';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import fontStyles from '../fontStyles';
import TouchableItem from './TouchableItem';

export default class AccountCard extends React.PureComponent {
	static propTypes = {
		address: PropTypes.string.isRequired,
		networkKey: PropTypes.string,
		onPress: PropTypes.func,
		seedType: PropTypes.string,
		style: ViewPropTypes.style,
		title: PropTypes.string
	};

	static defaultProps = {
		onPress: () => {},
		title: 'no name'
	};

	render() {
		const { address, networkKey, onPress, seedType } = this.props;
		let { title } = this.props;
		title = title.length ? title : AccountCard.defaultProps.title;
		const seedTypeDisplay = seedType || '';
		const network =
			NETWORK_LIST[networkKey] || NETWORK_LIST[NetworkProtocols.UNKNOWN];

		return (
			<TouchableItem
				accessibilityComponentType="button"
				disabled={false}
				onPress={onPress}
			>
				<View style={styles.content}>
					<AccountIcon
						address={address}
						protocol={network.protocol}
						style={styles.icon}
					/>
					<View style={styles.desc}>
						<View>
							<Text style={fontStyles.t_codeS}>
								{network.title} {seedTypeDisplay}{' '}
							</Text>
						</View>
						<Text numberOfLines={1} style={fontStyles.h2}>
							{title}
						</Text>
						<Address address={address} protocol={network.protocol} />
					</View>
					<View
						style={[
							styles.footer,
							{
								backgroundColor: network.color
							}
						]}
					/>
				</View>
			</TouchableItem>
		);
	}
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
		backgroundColor: '#977CF6',
		height: 88,
		width: 8
	},
	icon: {
		height: 40,
		width: 40
	}
});
