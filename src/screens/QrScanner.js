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

// @flow

'use strict';

import PropTypes from 'prop-types';
import React from 'react';
import { Alert, Button, StyleSheet, Text, View } from 'react-native';
import { RNCamera } from 'react-native-camera';
import { Subscribe } from 'unstated';

import colors from '../colors';
import fonts from '../fonts';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import { isAddressString, isJsonString, rawDataToU8A } from '../util/decoders';

export default class Scanner extends React.PureComponent {
	static navigationOptions = {
		headerBackTitle: 'Scanner',
		title: 'Transaction Details'
	};

	constructor(props) {
		super(props);
		this.state = { enableScan: true };
	}

	showErrorMessage(scannerStore, title, message) {
		this.setState({ enableScan: false });
		Alert.alert(title, message, [
			{
				onPress: async () => {
					await scannerStore.cleanup();
					this.setState({ enableScan: true });
				},
				text: 'Try again'
			}
		]);
	}

	render() {
		const isScanningMetadata = this.props.navigation.getParam(
			'isScanningMetadata'
		);

		return (
			<Subscribe to={[ScannerStore, AccountsStore]}>
				{(scannerStore, accountsStore) => {
					return (
						<QrScannerView
							completedFramesCount={scannerStore.getCompletedFramesCount()}
							isMultipart={scannerStore.getTotalFramesCount() > 1}
							missedFrames={scannerStore.getMissedFrames()}
							navigation={this.props.navigation}
							scannerStore={scannerStore}
							totalFramesCount={scannerStore.getTotalFramesCount()}
							onBarCodeRead={async txRequestData => {
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
										console.log(
											'QrScanner isScanningMetadata -> ',
											isScanningMetadata
										);
										await scannerStore.setParsedData(
											strippedData,
											accountsStore,
											false,
											isScanningMetadata
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
									} else if (scannerStore.getMetadata()) {
										debugger;
										await scannerStore.setData(accountsStore);
										scannerStore.clearMultipartProgress();
										this.props.navigation.navigate('MetadataDetails');
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

export class QrScannerView extends React.Component {
	constructor(props) {
		super(props);
		this.setBusySubscription = null;
		this.setReadySubscription = null;
	}

	static propTypes = {
		onBarCodeRead: PropTypes.func.isRequired
	};

	componentDidMount() {
		this.setBusySubscription = this.props.navigation.addListener(
			'willFocus',
			() => {
				this.props.scannerStore.setReady();
			}
		);
		this.setReadySubscription = this.props.navigation.addListener(
			'didBlur',
			() => {
				this.props.scannerStore.setBusy();
			}
		);
	}

	componentWillUnmount() {
		this.setBusySubscription.remove();
		this.setReadySubscription.remove();
	}

	renderScanningNetworkSpecMessage() {
		return (
			<View style={styles.bottom}>
				<Text style={styles.descTitle}>Scan QR Code</Text>
				<Text style={styles.descSecondary}>To Add a New Network Spec</Text>
			</View>
		);
	}

	renderScanningTransactionMessage() {
		return (
			<View style={styles.bottom}>
				<Text style={styles.descTitle}>Scan QR Code</Text>
				<Text style={styles.descSecondary}>To Sign a New Transaction</Text>
			</View>
		);
	}

	renderScanningMultipartMessage() {
		return (
			<View style={styles.bottom}>
				<Text style={styles.descTitle}>
					Scanning Multipart Data, Please Hold Still...
				</Text>
				<Text style={styles.descSecondary}>
					{this.props.completedFramesCount} / {this.props.totalFramesCount}{' '}
					Completed.
				</Text>
				<Button
					onPress={() => this.props.scannerStore.clearMultipartProgress()}
					style={styles.descSecondary}
					title="Start Over"
				/>
			</View>
		);
	}

	render() {
		const { onBarCodeRead, scannerStore, navigation } = this.props;
		const isScanningMetadata = navigation.getParam('isScanningMetadata');

		const missedFrames = this.props.scannerStore.getMissedFrames();
		const missedFramesMessage = missedFrames && missedFrames.join(', ');

		if (scannerStore.isBusy()) {
			return <View style={styles.inactive} />;
		} else {
		return (
			<RNCamera
				captureAudio={false}
				onBarCodeRead={onBarCodeRead}
				style={styles.view}
			>
				<View style={styles.body}>
					<View style={styles.top}>
						<Text style={styles.titleTop}>SCANNER</Text>
					</View>
					<View style={styles.middle}>
						<View style={styles.middleLeft} />
						<View style={styles.middleCenter} />
						<View style={styles.middleRight} />
					</View>
					{isScanningNetworkSpec
						? this.renderScanningNetworkSpecMessage()
						: this.renderScanningTransactionMessage()}
					{this.props.isMultipart ? (
						<View style={styles.bottom}>
							<Text style={styles.descTitle}>
								Scanning Multipart Data, Please Hold Still...
							</Text>
							<Text style={styles.descSecondary}>
								{this.props.completedFramesCount} /{' '}
								{this.props.totalFramesCount} Completed.
							</Text>
							<Button
								onPress={() => this.props.scannerStore.clearMultipartProgress()}
								style={styles.descSecondary}
								title="Start Over"
							/>
						</View>
					) : (
						<View style={styles.bottom}>
							<Text style={styles.descTitle}>Scan QR Code</Text>
							<Text style={styles.descSecondary}>
								To Sign a New Transaction
							</Text>
						</View>
						{this.props.isMultipart
							? this.renderScanningMultipartMessage()
							: isScanningMetadata
							? this.renderScanningNetworkSpecMessage()
							: this.renderScanningTransactionMessage()}
						{missedFrames && missedFrames.length >= 1 ? (
							<View style={styles.bottom}>
								<Text style={styles.descTitle}>
									You missed the following frames: {missedFramesMessage}
								</Text>
							</View>
						) : (
							undefined
						)}
					</View>
				</RNCamera>
			);
		}
	}
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
	titleTop: {
		color: colors.bg_text,
		fontFamily: fonts.bold,
		fontSize: 26,
		textAlign: 'center'
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
