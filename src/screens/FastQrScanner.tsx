// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import React, { useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { RNCamera } from 'react-native-camera';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import ScreenHeading from 'components/ScreenHeading';
import { tryDecodeQr } from 'utils/native';
import { packetSize } from 'constants/raptorQ';
import { saveMetadata } from 'utils/db';

export default function Scanner({
	navigation
}: NavigationProps<'FastQrScanner'>): React.ReactElement {
	const [readPacketsData, setReadPacketsData] = useState<Array<string>>([]);
	const [readPacketsCount, setReadPacketsCount] = useState(0);
	const [messageSize, setMessageSize] = useState(0);
	const [nominalPacketsNumber, setNominalPacketsNumber] = useState(0);

	// all code to derive information when size of package is determined
	function setExpectedMessageInfo(size: string): void {
		const parsedPrefix = parseInt('0x' + size, 16);
		setMessageSize(parsedPrefix);
		//always ask for two more packets (here and ">") to kick P>99.9%
		setNominalPacketsNumber(~~(parsedPrefix / packetSize) + 1);
	}

	function processQrFrame(data: string): void {
		if (nominalPacketsNumber === 0) {
			setExpectedMessageInfo(data.substr(5, 16));
		}
		const payload = data.substr(21, packetSize * 2);
		if (!readPacketsData.includes(payload)) {
			setReadPacketsData(readPacketsData.concat(payload));
			setReadPacketsCount(readPacketsCount + 1);
		}
	}

	const onBarCodeRead = async function (event: any): Promise<void> {
		processQrFrame(event.rawData);
		if (
			readPacketsCount >= nominalPacketsNumber &&
			nominalPacketsNumber !== 0
		) {
			const decoded = await tryDecodeQr(readPacketsData, messageSize);
			if (decoded !== '') {
				//TODO: here we should place general handler if/when we switch
				//to ubiquitous fountains. Now this handles only metadata input
				saveMetadata(decoded);
				navigation.goBack();
			}
		}
	};

	return (
		<SafeAreaViewContainer>
			<RNCamera
				barCodeTypes={[RNCamera.Constants.BarCodeType.qr]}
				captureAudio={false}
				onBarCodeRead={onBarCodeRead}
				style={styles.view}
			>
				<View style={styles.body}>
					<View style={styles.top}>
						<ScreenHeading title="Scanner" />
					</View>
					<View style={styles.middle}>
						<View style={styles.middleLeft} />
						<View style={styles.middleCenter} />
						<View style={styles.middleRight} />
					</View>
					<View style={styles.bottom}>
						<Text style={styles.descTitle}>Scanning fountain data</Text>
						<Text style={styles.descSecondary}>
							Packets: {readPacketsCount} / {nominalPacketsNumber}
						</Text>
					</View>
				</View>
			</RNCamera>
		</SafeAreaViewContainer>
	);
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: 'transparent',
		flex: 1,
		flexDirection: 'column'
	},
	bottom: {
		alignItems: 'center',
		backgroundColor: 'rgba(0, 0, 0, 0.5)',
		flex: 1,
		justifyContent: 'center',
		paddingHorizontal: 15
	},
	descSecondary: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 14,
		paddingBottom: 20
	},
	descTitle: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 10,
		textAlign: 'center'
	},
	inactive: {
		backgroundColor: colors.background.app,
		flex: 1,
		flexDirection: 'column',
		padding: 20
	},
	middle: {
		backgroundColor: 'transparent',
		flexBasis: 280,
		flexDirection: 'row'
	},
	middleCenter: {
		backgroundColor: 'transparent',
		borderWidth: 1,
		flexBasis: 280
	},
	middleLeft: {
		backgroundColor: 'rgba(0, 0, 0, 0.5)',
		flex: 1
	},
	middleRight: {
		backgroundColor: 'rgba(0, 0, 0, 0.5)',
		flex: 1
	},
	progress: {
		alignItems: 'center',
		justifyContent: 'center'
	},
	top: {
		alignItems: 'center',
		backgroundColor: 'rgba(0, 0, 0, 0.5)',
		flexBasis: 80,
		flexDirection: 'row',
		justifyContent: 'center'
	},
	view: {
		backgroundColor: 'black',
		flex: 1
	}
});
