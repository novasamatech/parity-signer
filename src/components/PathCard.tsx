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
import { StyleSheet, Text, View } from 'react-native';
import AntIcon from 'react-native-vector-icons/AntDesign';

import AccountIcon from './AccountIcon';
import Address from './Address';
import TouchableItem from './TouchableItem';
import AccountPrefixedTitle from './AccountPrefixedTitle';

import {
	defaultNetworkKey,
	NETWORK_LIST,
	NetworkProtocols,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import Separator from 'components/Separator';
import {
	getAddressWithPath,
	getNetworkKeyByPath,
	getPathName
} from 'utils/identitiesUtils';
import { ButtonListener } from 'types/props';
import { Identity } from 'types/identityTypes';

export default function PathCard({
	onPress,
	identity,
	path,
	name,
	testID,
	titlePrefix
}: {
	onPress?: ButtonListener;
	identity: Identity;
	path: string;
	name?: string;
	testID?: string;
	titlePrefix?: string;
}): React.ReactElement {
	const isNotEmptyName = name && name !== '';
	const pathName = isNotEmptyName ? name : getPathName(path, identity);
	const address = getAddressWithPath(path, identity);
	const isUnknownAddress = address === '';

	const networkKey = getNetworkKeyByPath(path);
	const network =
		networkKey === UnknownNetworkKeys.UNKNOWN && !isUnknownAddress
			? NETWORK_LIST[defaultNetworkKey]
			: NETWORK_LIST[networkKey];

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
				<AccountIcon address={address} network={network} style={styles.icon} />
				<View style={styles.desc}>
					<View>
						<Text style={[fontStyles.t_regular, { color: colors.bg_text_sec }]}>
							{network.title}
						</Text>
					</View>
					<AccountPrefixedTitle title={pathName!} titlePrefix={titlePrefix} />
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
						address={address}
						network={network}
						style={styles.icon}
					/>
					<View style={styles.desc}>
						<AccountPrefixedTitle title={pathName!} titlePrefix={titlePrefix} />
						<View style={{ alignItems: 'center', flexDirection: 'row' }}>
							<AntIcon name="user" size={10} color={colors.bg_text_sec} />
							<Text style={fontStyles.t_codeS}>{path}</Text>
						</View>
						{address !== '' && (
							<Text
								style={fontStyles.t_codeS}
								ellipsizeMode="middle"
								numberOfLines={1}
							>
								{address}
							</Text>
						)}
					</View>
				</View>
			</TouchableItem>
		</View>
	);

	return network.protocol === NetworkProtocols.SUBSTRATE ||
		network.protocol === NetworkProtocols.UNKNOWN
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
