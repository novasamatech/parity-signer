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

import Identicon from '@polkadot/reactnative-identicon';
import React, { ReactElement, useEffect, useState } from 'react';
import { Image, ImageStyle, StyleSheet, View, ViewStyle } from 'react-native';
import MaterialIcon from 'react-native-vector-icons/MaterialIcons';
import FontAwesome from 'react-native-vector-icons/FontAwesome';

import colors from 'styles/colors';
import { NetworkProtocols } from 'constants/networkSpecs';
import { blockiesIcon } from 'utils/native';
import { NetworkParams, SubstrateNetworkParams } from 'types/networkSpecsTypes';

export default function AccountIcon(props: {
	address: string;
	network: NetworkParams;
	style?: ViewStyle | ImageStyle;
}): ReactElement {
	const { address, style, network } = props;
	const [ethereumIconUri, setEthereumIconUri] = useState('');
	const protocol = network.protocol;

	useEffect((): void => {
		const loadEthereumIcon = function(ethereumAddress: string): void {
			blockiesIcon('0x' + ethereumAddress)
				.then(uri => {
					setEthereumIconUri(uri);
				})
				.catch(console.error);
		};

		if (protocol === NetworkProtocols.ETHEREUM) {
			loadEthereumIcon(address);
		}
	}, [address, protocol]);

	if (address === '') {
		return (
			<View style={style as ViewStyle}>
				{(network as SubstrateNetworkParams).logo ? (
					<Image
						source={(network as SubstrateNetworkParams).logo}
						style={styles.logo}
					/>
				) : (
					<View style={styles.logo}>
						<FontAwesome name="question" color={colors.bg} size={28} />
					</View>
				)}
			</View>
		);
	}
	if (protocol === NetworkProtocols.SUBSTRATE) {
		let iconSize;
		if (typeof style?.width === 'string') {
			const parseIconSize = parseInt(style.width, 10);
			iconSize = isNaN(parseIconSize) ? undefined : parseIconSize;
		} else {
			iconSize = style?.width;
		}
		return <Identicon value={address} size={iconSize || 40} />;
	} else if (protocol === NetworkProtocols.ETHEREUM && ethereumIconUri) {
		return (
			<Image
				source={{ uri: ethereumIconUri }}
				style={(style as ImageStyle) || { height: 40, width: 40 }}
			/>
		);
	} else {
		// if there's no protocol or it's unknown we return a warning
		return <MaterialIcon color={colors.bg_text} name={'error'} size={44} />;
	}
}

const styles = StyleSheet.create({
	logo: {
		alignItems: 'center',
		backgroundColor: colors.bg_text_sec,
		borderRadius: 40,
		height: 40,
		justifyContent: 'center',
		width: 40
	}
});
