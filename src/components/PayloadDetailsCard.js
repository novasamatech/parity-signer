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

import extrinsicsFromMeta from '@polkadot/api-metadata/extrinsics/fromMetadata';
import { GenericCall, getTypeRegistry, Metadata } from '@polkadot/types';
import Call from '@polkadot/types/primitive/Generic/Call';
import { formatBalance } from '@polkadot/util';
import { decodeAddress, encodeAddress } from '@polkadot/util-crypto';

import PropTypes from 'prop-types';
import React, { useEffect, useState } from 'react';
import { Alert, StyleSheet, Text, View, ViewPropTypes } from 'react-native';

import fonts from '../fonts';
import colors from '../colors';
import { SUBSTRATE_NETWORK_LIST, SubstrateNetworkKeys } from '../constants';
import kusamaMetadata from '../util/static-kusama';
// import substrateDevMetadata from '../util/static-substrate';

export default class PayloadDetailsCard extends React.PureComponent {
	static propTypes = {
		description: PropTypes.string.isRequired,
		payload: PropTypes.object,
		prefix: PropTypes.number.isRequired,
		signature: PropTypes.string,
		style: ViewPropTypes.style
	};

	state = {
		fallback: false
	};

	constructor(props) {
		super(props);

		const isKusama =
			this.props.prefix ===
			SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].prefix;
		// const isSubstrateDev = this.props.prefix === SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.SUBSTRATE_DEV].prefix;

		let metadata;
		if (isKusama) {
			metadata = new Metadata(kusamaMetadata);

			formatBalance.setDefaults({
				decimals: SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].decimals,
				unit: SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].unit
			});
		}
		// } else if (isSubstrateDev) {
		//   metadata = new Metadata(substrateDevMetadata);

		//   formatBalance.setDefaults({
		//     decimals: SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.SUBSTRATE_DEV].decimals,
		//     unit: SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.SUBSTRATE_DEV].unit
		//   });
		// }

		if (!metadata) {
			this.setState({
				fallback: true
			});
		}

		getTypeRegistry().register({
			Keys: 'SessionKeysPolkadot'
		});

		const extrinsics = extrinsicsFromMeta(metadata);
		GenericCall.injectMethods(extrinsics);
	}

	render() {
		const { fallback } = this.state;
		const { description, payload, prefix, signature, style } = this.props;

		return (
			<View style={[styles.body, style]}>
				<Text style={styles.titleText}>{description}</Text>
				{!!payload && (
					<View style={{ padding: 5, paddingVertical: 2 }}>
						<ExtrinsicPart
							label="Block Hash"
							prefix={prefix}
							value={payload.blockHash.toString()}
						/>
						<ExtrinsicPart
							label="Method"
							prefix={prefix}
							value={fallback ? payload.method.toString() : payload.method}
						/>
						<ExtrinsicPart
							label="Era"
							prefix={prefix}
							value={fallback ? payload.era.toString() : payload.era}
						/>
						<ExtrinsicPart
							label="Nonce"
							prefix={prefix}
							value={payload.nonce.toString()}
						/>
						<ExtrinsicPart
							label="Tip"
							prefix={prefix}
							value={payload.tip.toString()}
						/>
						<ExtrinsicPart
							label="Genesis Hash"
							prefix={prefix}
							value={payload.genesisHash.toString()}
						/>
					</View>
				)}
				{!!signature && (
					<View
						style={{ alignItems: 'baseline', padding: 5, paddingVertical: 2 }}
					>
						<Text style={styles.label}>Signature</Text>
						<Text style={styles.secondaryText}>{signature}</Text>
					</View>
				)}
			</View>
		);
	}
}

function ExtrinsicPart({ label, fallback, prefix, value }) {
	const [period, setPeriod] = useState();
	const [phase, setPhase] = useState();
	const [formattedCallArgs, setFormattedCallArgs] = useState();
	const [tip, setTip] = useState();
	const [useFallback, setUseFallBack] = useState(false);

	useEffect(() => {
		if (label === 'Method' && !fallback) {
			try {
				const call = new Call(value);

				let methodArgs = {};

				function formatArgs(callInstance, methodArgs, depth) {
					const { args, meta, methodName, sectionName } = callInstance;
					let paramArgKvArray = [];

					if (!meta.args.length) {
						const sectionMethod = `${sectionName}.${methodName}`;
						methodArgs[sectionMethod] = null;
						return;
					}

					for (let i = 0; i < meta.args.length; i++) {
						let argument;

						if (
							args[i].toRawType() === 'Balance' ||
							args[i].toRawType() === 'Compact<Balance>'
						) {
							argument = formatBalance(args[i].toString());
						} else if (
							args[i].toRawType() === 'Address' ||
							args[i].toRawType() === 'AccountId'
						) {
							// encode Address and AccountId to the appropriate prefix
							argument = encodeAddress(
								decodeAddress(args[i].toString()),
								prefix
							);
						} else if (args[i] instanceof Call) {
							argument = formatArgs(args[i], methodArgs, depth++); // go deeper into the nested calls
						} else {
							argument = args[i].toString();
						}
						const param = meta.args[i].name.toString();
						const sectionMethod = `${sectionName}.${methodName}`;
						paramArgKvArray.push([param, argument]);
						methodArgs[sectionMethod] = paramArgKvArray;
					}
				}

				formatArgs(call, methodArgs, 0);
				setFormattedCallArgs(methodArgs);
			} catch (e) {
				Alert.alert(
					'Could not decode method with available metadata.',
					'Signing something you do not understand is inherently unsafe. Do not sign this extrinsic unless you know what you are doing, or update Parity Signer to be able to decode this message. If you are not sure, or you are using the latest version, please open an issue on github.com/paritytech/parity-signer.',
					[
						{
							style: 'default',
							text: 'Okay'
						}
					]
				);
				setUseFallBack(true);
			}
		}

		if (label === 'Era' && !fallback) {
			if (value.isMortalEra) {
				setPeriod(value.asMortalEra.period.toString());
				setPhase(value.asMortalEra.phase.toString());
			}
		}

		if (label === 'Tip' && !fallback) {
			setTip(formatBalance(value));
		}
	}, [fallback, label, prefix, value]);

	const renderEraDetails = () => {
		if (period && phase) {
			return (
				<View style={{ display: 'flex', flexDirection: 'column', padding: 5 }}>
					<View
						style={{
							alignItems: 'flex-end',
							display: 'flex',
							flexDirection: 'row',
							justifyContent: 'space-around'
						}}
					>
						<Text style={{ ...styles.subLabel, flex: 1 }}>phase: </Text>
						<Text style={{ ...styles.secondaryText, flex: 1 }}>{phase}</Text>
						<Text style={{ ...styles.subLabel, flex: 1 }}>period: </Text>
						<Text style={{ ...styles.secondaryText, flex: 1 }}>{period}</Text>
					</View>
				</View>
			);
		} else {
			return (
				<View
					style={{
						display: 'flex',
						flexDirection: 'row',
						flexWrap: 'wrap',
						padding: 5
					}}
				>
					<Text style={{ ...styles.subLabel, flex: 1 }}>Immortal Era</Text>
					<Text style={{ ...styles.secondaryText, flex: 3 }}>
						{value.toString()}
					</Text>
				</View>
			);
		}
	};

	const renderMethodDetails = () => {
		if (formattedCallArgs) {
			return Object.entries(formattedCallArgs)
				.reverse()
				.map((entry, index) => {
					const sectionMethod = entry[0];
					const paramArgs = entry[1];

					return (
						<View key={index} style={styles.callDetails}>
							<Text>
								Call <Text style={styles.secondaryText}>{sectionMethod}</Text>{' '}
								with the following arguments:
							</Text>
							{paramArgs.map(([param, arg]) => (
								<React.Fragment key={param}>
									<Text style={{ ...styles.subLabel, flex: 1 }}>{param}: </Text>
									<Text style={{ ...styles.secondaryText, flex: 3 }}>
										{arg}
									</Text>
								</React.Fragment>
							))}
						</View>
					);
				});
		}
	};

	const renderTipDetails = () => {
		return (
			<View style={{ display: 'flex', flexDirection: 'column' }}>
				<Text style={styles.secondaryText}>{tip}</Text>
			</View>
		);
	};

	return (
		<View style={[{ alignItems: 'baseline', justifyContent: 'flex-start' }]}>
			<View
				style={{ margin: 5, padding: 5, paddingVertical: 2, width: '100%' }}
			>
				<Text style={styles.label}>{label}</Text>
				{label === 'Method' && !useFallback ? (
					renderMethodDetails()
				) : label === 'Era' ? (
					renderEraDetails()
				) : label === 'Tip' ? (
					renderTipDetails()
				) : (
					<Text style={styles.secondaryText}>
						{useFallback ? value.toString() : value}
					</Text>
				)}
			</View>
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.card_bg,
		flexDirection: 'column',
		padding: 20,
		paddingTop: 10
	},
	callDetails: {
		alignItems: 'flex-start',
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'flex-start',
		paddingLeft: 5,
		width: '100%'
	},
	icon: {
		height: 47,
		width: 47
	},
	label: {
		backgroundColor: colors.bg,
		color: colors.card_bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 20,
		textAlign: 'left'
	},
	secondaryText: {
		color: colors.card_bg_text_sec,
		fontFamily: fonts.semiBold,
		fontSize: 14,
		paddingLeft: 8,
		textAlign: 'left'
	},
	subLabel: {
		backgroundColor: null,
		color: colors.card_bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 14,
		paddingLeft: 5,
		textAlign: 'left'
	},
	titleText: {
		color: colors.card_bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 14,
		textAlign: 'center'
	}
});
