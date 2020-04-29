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

import React, { useContext, useEffect, useState } from 'react';
import { Alert, Button, StyleSheet, Text, View } from 'react-native';
import { RNCamera } from 'react-native-camera';

import { processBarCode } from 'modules/sign/utils';
import { onMockBarCodeRead } from 'e2e/injections';
import { SeedRefsContext } from 'stores/SeedRefStore';
import { NavigationAccountScannerProps } from 'types/props';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import ScreenHeading from 'components/ScreenHeading';
import { TxRequestData } from 'types/scannerTypes';
import {Tx} from 'types/tx';
import { withAccountAndScannerStore } from 'utils/HOC';

export function Scanner({
	navigation,
	accounts,
	scannerStore
}: NavigationAccountScannerProps<'QrScanner'>): React.ReactElement {
	const [seedRefs] = useContext<SeedRefsContext>(SeedRefsContext);
	const [enableScan, setEnableScan] = useState<boolean>(true);
	const [lastFrame, setLastFrame] = useState<null|TxRequestData>(null);
	useEffect((): (() => void) => {
		const unsubscribeFocus = navigation.addListener(
			'focus',
			scannerStore.setReady.bind(scannerStore)
		);
		const unsubscribeBlur = navigation.addListener(
			'blur',
			scannerStore.setBusy.bind(scannerStore)
		);
		return (): void => {
			unsubscribeFocus();
			unsubscribeBlur();
			scannerStore.setReady();
		};
	}, [navigation, scannerStore]);

	const completedFramesCount = scannerStore.getCompletedFramesCount();
	const isMultipart = scannerStore.getTotalFramesCount() > 1;
	const missedFrames = scannerStore.getMissedFrames();
	const totalFramesCount = scannerStore.getTotalFramesCount();
	const missedFramesMessage = missedFrames && missedFrames.join(', ');

	function showErrorMessage(title: string, message: string): void {
		setEnableScan(false);
		scannerStore.setBusy();
		Alert.alert(title, message, [
			{
				onPress: async (): Promise<void> => {
					await scannerStore.cleanup();
					scannerStore.setReady();
					setLastFrame(null);
					setEnableScan(true);
				},
				text: 'Try again'
			}
		]);
	}

	//e2e signing test injection
	if (global.inTest && global.scanRequest !== undefined) {
		onMockBarCodeRead(global.scanRequest, (tx: TxRequestData) => {
			processBarCode(
				showErrorMessage,
				tx as TxRequestData,
				navigation,
				accounts,
				scannerStore,
				seedRefs
			);
		});
	}

	return (
		<RNCamera
			captureAudio={false}
			onBarCodeRead={(event: any): void => {
				if (scannerStore.isBusy() || !enableScan) {
					return;
				}
				if(event as TxRequestData === lastFrame) {
					return;
				}
				setLastFrame(event);
				processBarCode(
					showErrorMessage,
					event as TxRequestData,
					navigation,
					accounts,
					scannerStore,
					seedRefs
				);
			}}
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
				{isMultipart ? (
					<View style={styles.bottom}>
						<Text style={styles.descTitle}>
							Scanning Multipart Data, Please Hold Still...
						</Text>
						<Text style={styles.descSecondary}>
							{completedFramesCount} / {totalFramesCount} Completed.
						</Text>
						<Button
							onPress={(): void => scannerStore.clearMultipartProgress()}
							title="Start Over"
						/>
					</View>
				) : (
					<View style={styles.bottom}>
						<Text style={styles.descTitle}>Scan QR Code</Text>
						<Text style={styles.descSecondary}>To Sign a New Transaction</Text>
					</View>
				)}
				{missedFrames && missedFrames.length >= 1 && (
					<View style={styles.bottom}>
						<Text style={styles.descTitle}>
							You missed the following frames: {missedFramesMessage}
						</Text>
					</View>
				)}
			</View>
		</RNCamera>
	);
}

export default withAccountAndScannerStore(Scanner);

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
		color: colors.bg_text,
		fontFamily: fonts.bold,
		fontSize: 14,
		paddingBottom: 20
	},
	descTitle: {
		color: colors.bg_text,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 10,
		textAlign: 'center'
	},
	inactive: {
		backgroundColor: colors.bg,
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
