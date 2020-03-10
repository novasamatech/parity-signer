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

import React, { useEffect } from 'react';
import { Alert, Button, StyleSheet, Text, View } from 'react-native';
import { RNCamera } from 'react-native-camera';
import { Subscribe } from 'unstated';

import { createMockSignRequest } from 'e2e/mock';
import { NavigationProps, NavigationScannerProps } from 'types/props';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import AccountsStore from 'stores/AccountsStore';
import ScannerStore from 'stores/ScannerStore';
import { isAddressString, isJsonString, rawDataToU8A } from 'utils/decoders';
import ScreenHeading from 'components/ScreenHeading';
import { TxRequestData } from 'types/scannerTypes';

interface State {
	enableScan: boolean;
}

export default class Scanner extends React.PureComponent<
	NavigationProps<{}>,
	State
> {
	constructor(props: NavigationProps<{}>) {
		super(props);
		this.state = { enableScan: true };
	}

	showErrorMessage(
		scannerStore: ScannerStore,
		title: string,
		message: string
	): void {
		this.setState({ enableScan: false });
		Alert.alert(title, message, [
			{
				onPress: async (): Promise<void> => {
					await scannerStore.cleanup();
					this.setState({ enableScan: true });
				},
				text: 'Try again'
			}
		]);
	}

	render(): React.ReactElement {
		return (
			<Subscribe to={[ScannerStore, AccountsStore]}>
				{(
					scannerStore: ScannerStore,
					accountsStore: AccountsStore
				): React.ReactElement => {
					return (
						<QrScannerView
							completedFramesCount={scannerStore.getCompletedFramesCount()}
							isMultipart={scannerStore.getTotalFramesCount() > 1}
							missedFrames={scannerStore.getMissedFrames()}
							navigation={this.props.navigation}
							scannerStore={scannerStore}
							totalFramesCount={scannerStore.getTotalFramesCount()}
							onBarCodeRead={async (
								txRequestData: TxRequestData
							): Promise<void> => {
								if (scannerStore.isBusy() || !this.state.enableScan) {
									return;
								}
								try {
									if (isAddressString(txRequestData.data)) {
										return this.showErrorMessage(
											scannerStore,
											text.ADDRESS_ERROR_TITLE,
											text.ADDRESS_ERROR_MESSAGE
										);
									} else if (isJsonString(txRequestData.data)) {
										// Ethereum Legacy
										await scannerStore.setUnsigned(txRequestData.data);
									} else if (!scannerStore.isMultipartComplete()) {
										const strippedData = rawDataToU8A(txRequestData.rawData);
										if (strippedData === null)
											return this.showErrorMessage(
												scannerStore,
												text.PARSE_ERROR_TITLE,
												'There is no raw Data from the request'
											);
										await scannerStore.setParsedData(
											strippedData,
											accountsStore,
											false
										);
									}

									if (scannerStore.getErrorMsg()) {
										throw new Error(scannerStore.getErrorMsg());
									}

									if (scannerStore.getUnsigned()) {
										await scannerStore.setData(accountsStore);
										if (scannerStore.getType() === 'transaction') {
											scannerStore.clearMultipartProgress();
											this.props.navigation.navigate('TxDetails');
										} else {
											scannerStore.clearMultipartProgress();
											this.props.navigation.navigate('MessageDetails');
										}
									}
								} catch (e) {
									return this.showErrorMessage(
										scannerStore,
										text.PARSE_ERROR_TITLE,
										e.message
									);
								}
							}}
						/>
					);
				}}
			</Subscribe>
		);
	}
}

interface ViewProps extends NavigationScannerProps<{}> {
	onBarCodeRead: (listener: TxRequestData) => void;
	completedFramesCount: number;
	isMultipart: boolean;
	missedFrames: number[];
	totalFramesCount: number;
}

function QrScannerView({
	navigation,
	scannerStore,
	...props
}: ViewProps): React.ReactElement {
	if (global.inTest) {
		props.onBarCodeRead(createMockSignRequest());
	}

	useEffect((): (() => void) => {
		const setBusySubscription = navigation.addListener('willFocus', () => {
			scannerStore.setReady();
		});
		const setReadySubscription = navigation.addListener('didBlur', () => {
			scannerStore.setBusy();
		});
		return (): void => {
			setBusySubscription.remove();
			setReadySubscription.remove();
			scannerStore.setReady();
		};
	}, [navigation, scannerStore]);

	const missedFrames = scannerStore.getMissedFrames();
	const missedFramesMessage = missedFrames && missedFrames.join(', ');

	if (scannerStore.isBusy()) {
		return <View style={styles.inactive} />;
	}
	return (
		<RNCamera
			captureAudio={false}
			onBarCodeRead={(event: any): void =>
				props.onBarCodeRead(event as TxRequestData)
			}
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
				{props.isMultipart ? (
					<View style={styles.bottom}>
						<Text style={styles.descTitle}>
							Scanning Multipart Data, Please Hold Still...
						</Text>
						<Text style={styles.descSecondary}>
							{props.completedFramesCount} / {props.totalFramesCount} Completed.
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

const text = {
	ADDRESS_ERROR_MESSAGE:
		'Please create a transaction using a software such as MyCrypto or Fether so that Parity Signer can sign it.',
	ADDRESS_ERROR_TITLE: 'Address detected',
	PARSE_ERROR_TITLE: 'Unable to parse transaction'
};

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
