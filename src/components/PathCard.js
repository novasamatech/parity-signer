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

'use strict';

import React from 'react';
import PropTypes from 'prop-types';
import { StyleSheet, Text, View } from 'react-native';
import AntIcon from 'react-native-vector-icons/AntDesign';
import {
	getAccountIdWithPath,
	getAddressFromAccountId,
	getNetworkKeyByPath,
	getPathName
} from '../util/identitiesUtils';
import { NETWORK_LIST } from '../constants';
import Separator from '../components/Separator';
import AccountIcon from './AccountIcon';
import Address from './Address';
import colors from '../colors';
import fontStyles from '../fontStyles';
import TouchableItem from './TouchableItem';
import { AccountPrefixedTitle } from './AccountPrefixedTitle';

PathCard.propTypes = {
	identity: PropTypes.object.isRequired,
	name: PropTypes.string,
	onPress: PropTypes.func,
	path: PropTypes.string.isRequired,
	testID: PropTypes.string,
	titlePrefix: PropTypes.string
};

export default function PathCard({
	onPress,
	identity,
	path,
	name,
	testID,
	titlePrefix
}) {
	const isNotEmptyName = name && name !== '';
	const pathName = isNotEmptyName ? name : getPathName(path, identity);
	const accountId = getAccountIdWithPath(path, identity);

	const networkKey = getNetworkKeyByPath(path);
	const network = NETWORK_LIST[networkKey];
	const extractAddress = getAddressFromAccountId(accountId, network.protocol);

	const nonSubstrateCard = (
		<View testID={testID}>
			<Separator
				shadow={true}
				style={{
					backgroundColor: 'transparent',
					height: 0,
					marginVertical: 0
				}}
			/>
			<View style={styles.content}>
				<AccountIcon
					address={extractAddress}
					protocol={network.protocol}
					network={network}
					style={styles.icon}
				/>
				<View style={styles.desc}>
					<View>
						<Text style={[fontStyles.t_regular, { color: colors.bg_text_sec }]}>
							{network.title}
						</Text>
					</View>
					<AccountPrefixedTitle title={pathName} titlePrefix={titlePrefix} />
					<Address address={extractAddress} protocol={network.protocol} />
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
		</View>
	);
	const substrateDerivationCard = (
		<View style={styles.body}>
			<TouchableItem
				accessibilityComponentType="button"
				disabled={false}
				onPress={onPress}
				testID={testID}
			>
				<View style={[styles.content, styles.contentDer]}>
					<AccountIcon
						address={extractAddress}
						protocol={network.protocol}
						network={network}
						style={styles.icon}
					/>
					<View style={styles.desc}>
						<AccountPrefixedTitle title={pathName} titlePrefix={titlePrefix} />
						<View style={{ alignItems: 'center', flexDirection: 'row' }}>
							<AntIcon name="user" size={10} color={colors.bg_text_sec} />
							<Text style={fontStyles.t_codeS}>{path}</Text>
						</View>
						{extractAddress !== '' && (
							<Text
								style={fontStyles.t_codeS}
								ellipsizeMode="middle"
								numberOfLines={1}
							>
								{extractAddress}
							</Text>
						)}
					</View>
				</View>
			</TouchableItem>
		</View>
	);

	return network.protocol === 'substrate'
		? substrateDerivationCard
		: nonSubstrateCard;
}

const styles = StyleSheet.create({
	body: {
		flexDirection: 'column',
		marginBottom: 10
	},
	content: {
		alignItems: 'center',
		flexDirection: 'row',
		paddingLeft: 16
	},
	contentDer: {
		backgroundColor: colors.card_bg,
		paddingVertical: 8
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
		height: 100,
		marginLeft: 8,
		width: 8
	},
	icon: {
		height: 40,
		width: 40
	}
});
