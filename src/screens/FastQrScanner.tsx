// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import React, { useState, useEffect } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { RNCamera } from 'react-native-camera';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import ScreenHeading from 'components/ScreenHeading';
import { tryDecodeQr } from 'utils/native';
import { resetNavigationWithNetworkChooser } from 'utils/navigationHelpers';
//for tests
import testIDs from 'e2e/testIDs';
import { useInjectionQR } from 'e2e/injections';

export default function Scanner({
	navigation
}: NavigationProps<'FastQrScanner'>): React.ReactElement {
	const [readPacketsData, setReadPacketsData] = useState<Array<string>>([]);
	const [readPacketsCount, setReadPacketsCount] = useState(0);
	const [messageSize, setMessageSize] = useState(0);
	const [nominalPacketsNumber, setNominalPacketsNumber] = useState(0);
	const [packetSize, setPacketSize] = useState(0);
	const [decodeProcess, setDecodeProcess] = useState(true);

	// E2E tests
	//const [mockIndex, onMockBarCodeRead] = useInjectionQR();

	// all code to derive information when size of package is determined
	function setExpectedMessageInfo(size: string): void {
		const parsedPacketSize = parseInt(size.substr(0, 4), 16) - 4;
		const parsedMessageSize = parseInt(size.substr(4, 8), 16) - 0x80000000;
		console.log(parsedPacketSize);
		console.log(parsedMessageSize);
		setPacketSize(parsedPacketSize);
		setMessageSize(parsedMessageSize);
		//always ask for two more packets to kick P>99.9%
		setNominalPacketsNumber(~~(parsedMessageSize / (parsedPacketSize - 4)) + 2);
	}

	function processPackage(payload: string): void {
		resetNavigationWithNetworkChooser(navigation, 'DetailsTx', {
			payload: payload
		});
	}

	function processQrFrame(data: string): void {
		if (nominalPacketsNumber === 0) {
			console.log(data);
			setDecodeProcess(true);
			if (parseInt(data.substr(5, 1), 16) & 8) {
				setExpectedMessageInfo(data.substr(1, 12));
			} else {
				console.log('legacy package');
				const parsedLegacySize = parseInt(data.substr(1, 4), 16) - 5;
				console.log(parsedLegacySize);
				console.log(data.substr(15, parsedLegacySize * 2));
				processPackage(data.substr(15, parsedLegacySize * 2));
				//navigation.goBack();
			}
		}
		const payload = data.substr(13, packetSize * 2);

		//fountain message received and packetSize still not updated? Tough luck!

		if (!readPacketsData.includes(payload) && payload) {
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
			const decoded = await tryDecodeQr(
				readPacketsData,
				messageSize,
				packetSize
			);
			if (decoded !== '' && decodeProcess) {
				console.log('success');
				console.log(decoded.substr(0, 128));
				setDecodeProcess(false);
				processPackage(decoded);
			}
		}
	};

	//	useEffect(() => {
	/** E2E Test Injection Code **/
	/*		if (global.inTest && global.scanRequest !== undefined) {
			onMockBarCodeRead(
				global.scanRequest,
				async (tx: TxRequestData): Promise<void> => {
					await onBarCodeRead(tx);
				}
			);
		}
		/* eslint-disable-next-line react-hooks/exhaustive-deps */
	//	}, [mockIndex]);

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
