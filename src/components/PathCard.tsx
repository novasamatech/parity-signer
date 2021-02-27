// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import Separator from 'components/Separator';
import { defaultNetworkKey, NETWORK_LIST, UnknownNetworkKeys } from 'constants/networkSpecs';
import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import AntIcon from 'react-native-vector-icons/AntDesign';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { Identity } from 'types/identityTypes';
import { isSubstrateNetwork, isUnknownNetworkParams } from 'types/networkTypes';
import { ButtonListener } from 'types/props';
import { getAddressWithPath, getNetworkKeyByPath, getPathName } from 'utils/identitiesUtils';
import { useSeedRef } from 'utils/seedRefHooks';

import { NetworksContext } from '../context';
import AccountIcon from './AccountIcon';
import AccountPrefixedTitle from './AccountPrefixedTitle';
import Address from './Address';
import TouchableItem from './TouchableItem';

interface Props {
	onPress?: ButtonListener;
	identity: Identity;
	isPathValid?: boolean;
	path: string;
	name?: string;
	networkKey?: string;
	testID?: string;
	titlePrefix?: string;
}

export default function PathCard({ identity, isPathValid = true, name, networkKey, onPress, path, testID, titlePrefix }: Props): React.ReactElement {
	const networksContext = useContext(NetworksContext);
	const { allNetworks, networks } = networksContext;
	const isNotEmptyName = name && name !== '';
	const pathName = isNotEmptyName ? name : getPathName(path, identity);
	const { isSeedRefValid, substrateAddress } = useSeedRef(identity.encryptedSeed);
	const [address, setAddress] = useState('');
	const computedNetworkKey =
		networkKey ||
		getNetworkKeyByPath(path, identity.meta.get(path)!, networksContext);

	useEffect(() => {
		const getAddress = async (): Promise<void> => {
			const existedAddress = getAddressWithPath(path, identity);

			if (existedAddress !== '') return setAddress(existedAddress);

			if (isSeedRefValid && isPathValid && networks.has(computedNetworkKey)) {
				const prefix = networks.get(computedNetworkKey)!.prefix;
				const generatedAddress = await substrateAddress(path, prefix);

				return setAddress(generatedAddress);
			}

			setAddress('');
		};

		getAddress();
	}, [
		path,
		identity,
		isPathValid,
		networkKey,
		computedNetworkKey,
		isSeedRefValid,
		substrateAddress,
		networks
	]);

	const isUnknownAddress = address === '';
	const hasPassword = identity.meta.get(path)?.hasPassword ?? false;
	const networkParams =
		computedNetworkKey === UnknownNetworkKeys.UNKNOWN &&
		!isUnknownAddress &&
		!allNetworks.has(computedNetworkKey)
			? NETWORK_LIST[defaultNetworkKey]
			: allNetworks.get(computedNetworkKey)!;

	const nonSubstrateCard = (
		<>
			<Separator
				shadow={true}
				style={{
					backgroundColor: 'transparent',
					height: 0,
					marginVertical: 0
				}}
			/>
			<View
				style={[
					styles.content,
					{ backgroundColor: 'transparent', paddingVertical: 0 }
				]}
				testID={testID}
			>
				<AccountIcon
					address={address}
					network={networkParams}
					style={styles.identicon}
				/>
				<View style={styles.desc}>
					<Text style={[fontStyles.t_regular, { color: colors.text.faded }]}>
						{networkParams.title}
					</Text>
					<AccountPrefixedTitle title={pathName!}
						titlePrefix={titlePrefix} />
					<Address address={address}
						protocol={networkParams.protocol} />
				</View>
				<View
					style={[
						styles.footer,
						{ backgroundColor: networkParams.color }
					]}
				/>
			</View>
		</>
	);

	const substrateDerivationCard = (
		<TouchableItem
			disabled={false}
			onPress={onPress}
			style={styles.body}
		>
			<View style={styles.content}
				testID={testID}>
				<AccountIcon
					address={address}
					network={networkParams}
					style={styles.identicon}
				/>
				<View style={styles.desc}>
					<View style={styles.row}>
						<AccountPrefixedTitle title={pathName!}
							titlePrefix={titlePrefix} />
						{hasPassword && <AntIcon name="lock"
							style={styles.iconLock} />}
					</View>
					<View
						style={{
							alignItems: 'center',
							flexDirection: 'row'
						}}
					>
						<AntIcon
							color={colors.signal.main}
							name="user"
							size={fontStyles.i_small.fontSize}
						/>

						{hasPassword ? (
							<View style={styles.row}>
								<Text
									style={[fontStyles.t_codeS, { color: colors.signal.main }]}
								>
									{path}///
								</Text>

								<AntIcon
									color={colors.signal.main}
									name="lock"
									size={fontStyles.i_small.fontSize}
									style={{ alignSelf: 'center' }}
								/>
							</View>
						) : (
							<Text style={[fontStyles.t_codeS, { color: colors.signal.main }]}>
								{path}
							</Text>
						)}
					</View>
					{address !== '' && (
						<Text
							ellipsizeMode="middle"
							numberOfLines={1}
							style={[fontStyles.t_codeS, { color: colors.text.faded }]}
						>
							{address}
						</Text>
					)}
				</View>
			</View>
		</TouchableItem>
	);

	return isSubstrateNetwork(networkParams) ||
		isUnknownNetworkParams(networkParams)
		? substrateDerivationCard
		: nonSubstrateCard;
}

const styles = StyleSheet.create({
	body: {
		borderBottomWidth: 1,
		borderColor: colors.background.app,
		borderTopWidth: 1
	},
	content: {
		alignItems: 'center',
		backgroundColor: colors.background.card,
		flexDirection: 'row',
		paddingLeft: 16,
		paddingVertical: 8
	},
	desc: {
		flex: 1,
		paddingLeft: 16
	},
	footer: {
		height: 80,
		marginLeft: 8,
		width: 4
	},
	iconLock: {
		marginLeft: 4,
		...fontStyles.h2
	},
	identicon: {
		height: 40,
		width: 40
	},
	row: {
		alignItems: 'flex-end',
		flexDirection: 'row'
	}
});
