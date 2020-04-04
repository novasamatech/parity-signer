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
	isSubstrateNetworkParams,
	isUnknownNetworkParams
} from 'types/networkSpecsTypes';
import {
	defaultNetworkKey,
	NETWORK_LIST,
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
	networkKey,
	testID,
	titlePrefix
}: {
	onPress?: ButtonListener;
	identity: Identity;
	path: string;
	name?: string;
	networkKey?: string;
	testID?: string;
	titlePrefix?: string;
}): React.ReactElement {
	const isNotEmptyName = name && name !== '';
	const pathName = isNotEmptyName ? name : getPathName(path, identity);
	const address = getAddressWithPath(path, identity);
	const isUnknownAddress = address === '';

	const hasPassword = identity.meta.get(path)?.hasPassword ?? false;
	const computedNetworkKey =
		networkKey || getNetworkKeyByPath(path, identity.meta.get(path)!);
	const networkParams =
		computedNetworkKey === UnknownNetworkKeys.UNKNOWN && !isUnknownAddress
			? NETWORK_LIST[defaultNetworkKey]
			: NETWORK_LIST[computedNetworkKey];

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
					address={address}
					network={networkParams}
					style={styles.iconUser}
				/>
				<View style={styles.desc}>
					<View>
						<Text style={[fontStyles.t_regular, { color: colors.bg_text_sec }]}>
							{networkParams.title}
						</Text>
					</View>
					<AccountPrefixedTitle title={pathName!} titlePrefix={titlePrefix} />
					<Address address={address} protocol={networkParams.protocol} />
				</View>
				<View
					style={[
						styles.footer,
						{
							backgroundColor: networkParams.color
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
			>
				<View style={[styles.content, styles.contentSubstrate]} testID={testID}>
					<AccountIcon
						address={address}
						network={networkParams}
						style={styles.iconUser}
					/>
					<View style={styles.desc}>
						<View style={styles.titleContainer}>
							<AccountPrefixedTitle
								title={pathName!}
								titlePrefix={titlePrefix}
							/>
							{hasPassword && <AntIcon name="lock" style={styles.iconLock} />}
						</View>
						<View style={{ alignItems: 'center', flexDirection: 'row' }}>
							<AntIcon name="user" size={10} color={colors.bg_text_sec} />
							<Text style={fontStyles.t_codeS}>
								{hasPassword ? `${path}///***` : path}
							</Text>
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

	return isSubstrateNetworkParams(networkParams) ||
		isUnknownNetworkParams(networkParams)
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
	contentSubstrate: {
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
	iconLock: {
		marginLeft: 4,
		...fontStyles.h2
	},
	iconUser: {
		height: 40,
		width: 40
	},
	titleContainer: {
		alignItems: 'center',
		flexDirection: 'row'
	}
});
