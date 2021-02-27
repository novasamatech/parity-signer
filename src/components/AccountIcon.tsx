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

import { NetworkProtocols } from 'constants/networkSpecs';
import React, { ReactElement, useEffect, useState } from 'react';
import { Image, ImageStyle, StyleSheet, View, ViewStyle } from 'react-native';
import FontAwesome from 'react-native-vector-icons/FontAwesome';
import MaterialIcon from 'react-native-vector-icons/MaterialIcons';
import colors from 'styles/colors';
import { NetworkParams, SubstrateNetworkParams } from 'types/networkTypes';
import { blockiesIcon } from 'utils/native';

import Identicon from '@polkadot/reactnative-identicon';

interface Props {
	address: string;
	network: NetworkParams | null;
	style?: ViewStyle;
}

export default function AccountIcon({ address, network, style }: Props): ReactElement {
	const [ethereumIconUri, setEthereumIconUri] = useState('');
	const protocol = network?.protocol;

	useEffect((): (() => void) => {
		let promiseDisabled = false;

		if (protocol === NetworkProtocols.ETHEREUM && address !== '') {
			const setEthereumIcon = async (): Promise<void> => {
				const generatedIconUri = await blockiesIcon('0x' + address);

				if (promiseDisabled) return;
				setEthereumIconUri(generatedIconUri);
			};

			setEthereumIcon();
		}

		return (): void => {
			promiseDisabled = true;
		};
	}, [address, protocol]);

	if (address === '' && !!network) {
		return (
			<View style={style}>
				{(network as SubstrateNetworkParams).logo ? (
					<Image
						source={(network as SubstrateNetworkParams).logo}
						style={styles.logo}
					/>
				) : (
					<View style={styles.logo}>
						<FontAwesome color={colors.text.main}
							name="question"
							size={28} />
					</View>
				)}
			</View>
		);
	}

	if (protocol === NetworkProtocols.ETHEREUM && ethereumIconUri) {
		return (
			<Image
				source={{ uri: ethereumIconUri }}
				style={StyleSheet.flatten([style, styles.ethereumIdenticon]) as ImageStyle}
			/>
		);
	} else if (address !== '') {
		let iconSize;

		if (typeof style?.width === 'string') {
			const parseIconSize = parseInt(style.width, 10);

			iconSize = isNaN(parseIconSize) ? undefined : parseIconSize;
		} else {
			iconSize = style?.width;
		}

		return (
			<Identicon
				size={iconSize || 40}
				value={address}
			/>
		);
	} else {
		return (
			<MaterialIcon
				color={colors.signal.error}
				name={'error'}
				size={44}
			/>
		);
	}
}

const styles = StyleSheet.create({
	ethereumIdenticon:{
		borderRadius: 50,
		height: 50,
		width: 50
	},
	logo: {
		alignItems: 'center',
		height: 36,
		justifyContent: 'center',
		marginHorizontal: 2,
		opacity: 0.7,
		width: 36
	}
});
