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
import Separator from '../components/Separator';
import AccountIcon from './AccountIcon';
import Address from './Address';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import fontStyles from '../fontStyles';
import TouchableItem from './TouchableItem';
import colors from '../colors';
import { getAddressFromAccountId } from '../util/identitiesUtils';

export default class AccountCard extends React.PureComponent {
	static propTypes = {
		accountId: PropTypes.string.isRequired,
		networkKey: PropTypes.string,
		onPress: PropTypes.func,
		seedType: PropTypes.string,
		style: ViewPropTypes.style,
		testID: PropTypes.string,
		title: PropTypes.string,
		titlePrefix: PropTypes.string
	};

	static defaultProps = {
		onPress: () => {},
		title: 'No name'
	};

	render() {
		const {
			accountId,
			isNetworkCard,
			networkKey,
			networkColor,
			onPress,
			seedType,
			style,
			testID,
			titlePrefix
		} = this.props;
		let { title } = this.props;
		title = title.length ? title : AccountCard.defaultProps.title;
		const seedTypeDisplay = seedType || '';
		const network =
			NETWORK_LIST[networkKey] || NETWORK_LIST[NetworkProtocols.UNKNOWN];

		const extractAddress = getAddressFromAccountId(accountId, network.protocol);

		return (
			<TouchableItem
				accessibilityComponentType="button"
				testID={testID}
				disabled={false}
				onPress={onPress}
			>
				<Separator
					shadow={true}
					style={{
						backgroundColor: 'transparent',
						height: 0,
						marginVertical: 0
					}}
				/>
				<View style={[styles.content, style]}>
					<AccountIcon
						address={extractAddress}
						protocol={network.protocol}
						network={network}
						style={styles.icon}
					/>
					<View style={styles.desc}>
						{!isNetworkCard && (
							<View>
								<Text
									style={[fontStyles.t_regular, { color: colors.bg_text_sec }]}
								>
									{network.title} {seedTypeDisplay}{' '}
								</Text>
							</View>
						)}
						<View style={{ flexDirection: 'row', marginTop: -2 }}>
							<Text
								numberOfLines={1}
								style={[
									fontStyles.t_codeS,
									{ color: colors.bg_button, marginTop: 7 }
								]}
							>
								{titlePrefix}
							</Text>
							<Text numberOfLines={1} style={fontStyles.h2}>
								{title}
							</Text>
						</View>
						{accountId !== '' && (
							<Address address={extractAddress} protocol={network.protocol} />
						)}
					</View>
					<View
						style={[
							styles.footer,
							{
								backgroundColor: networkColor || network.color
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
		height: 88,
		marginLeft: 8,
		width: 8
	},
	icon: {
		height: 40,
		width: 40
	}
});
