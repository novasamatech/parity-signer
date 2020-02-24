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

import { isHex } from '@polkadot/util';
import React, { useEffect, useState } from 'react';
import { Dimensions, Image, StyleSheet, View, ViewStyle } from 'react-native';

import { qrCode, qrCodeHex } from 'utils/native';

interface Props {
	data: string;
	size?: number;
	height?: number;
	style?: ViewStyle;
}

export default function QrView(props: Props): React.ReactElement {
	const [qr, setQr] = useState('');

	useEffect(() => {
		async function displayQrCode(data: string): Promise<void> {
			try {
				const generatedQr = isHex(data)
					? await qrCodeHex(data)
					: await qrCode(data);
				setQr(generatedQr);
			} catch (e) {
				console.error(e);
			}
		}

		displayQrCode(props.data);
	}, [props.data]);

	const { width: deviceWidth } = Dimensions.get('window');
	const size = props.size || deviceWidth - 64;
	const flexBasis = props.height || deviceWidth - 32;

	return (
		<View
			style={[
				styles.rectangleContainer,
				{
					backgroundColor: 'white',
					flexBasis,
					height: flexBasis,
					marginHorizontal: 16,
					marginVertical: 32,
					width: deviceWidth - 32
				},
				props.style
			]}
		>
			{qr !== '' && (
				<Image source={{ uri: qr }} style={{ height: size, width: size }} />
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	rectangleContainer: {
		alignItems: 'center',
		backgroundColor: 'transparent',
		flex: 1,
		justifyContent: 'center'
	}
});
