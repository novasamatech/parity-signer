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

import { GenericCall, getTypeRegistry, Metadata } from '@polkadot/types';
import Call from '@polkadot/types/primitive/Generic/Call';
import { formatBalance } from '@polkadot/util';
import { decodeAddress, encodeAddress } from '@polkadot/util-crypto';

import PropTypes from 'prop-types';
import React, { useEffect, useState } from 'react';
import { StyleSheet, Text, View, ViewPropTypes } from 'react-native';

import colors from '../colors';
import { SUBSTRATE_NETWORK_LIST, SubstrateNetworkKeys } from '../constants';
import kusamaMetadata from '../util/static-kusama';
import substrateDevMetadata from '../util/static-substrate';
import { shortString } from '../util/strings';
import fontStyles from '../fontStyles';
import { alertDecodeError } from '../util/alertUtils';

export default class PayloadDetailsCard extends React.PureComponent {
	static propTypes = {
		description: PropTypes.string,
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

		// KUSAMA and KUSAMA_DEV have the same metadata and Defaults values
		const isKusama =
			this.props.prefix ===
				SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].prefix ||
			SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA_DEV].prefix;
		const isSubstrateDev =
			this.props.prefix ===
			SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.SUBSTRATE_DEV].prefix;

		let metadata;
		if (isKusama) {
			metadata = new Metadata(kusamaMetadata);

			formatBalance.setDefaults({
				decimals: SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].decimals,
				unit: SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].unit
			});
		} else if (__DEV__ && isSubstrateDev) {
			metadata = new Metadata(substrateDevMetadata);

			formatBalance.setDefaults({
				decimals:
					SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.SUBSTRATE_DEV].decimals,
				unit: SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.SUBSTRATE_DEV].unit
			});
		}

		if (!metadata) {
			this.setState({
				fallback: true
			});
		}

		getTypeRegistry().register({
			Keys: 'SessionKeysPolkadot'
		});

		GenericCall.injectMetadata(metadata);
	}

	render() {
		const { fallback } = this.state;
		const { description, payload, prefix, signature, style } = this.props;

		return (
			<View style={[styles.body, style]}>
				{!!description && <Text style={styles.titleText}>{description}</Text>}
				{!!payload && (
					<View style={{ paddingVertical: 2 }}>
						<ExtrinsicPart
							label="Method"
							prefix={prefix}
							value={fallback ? payload.method.toString() : payload.method}
						/>
						<ExtrinsicPart
							label="Block Hash"
							prefix={prefix}
							value={payload.blockHash.toString()}
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
					<View style={{ alignItems: 'baseline', paddingVertical: 2 }}>
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

				// todo: clean this up
				function formatArgs(callInstance, callMethodArgs, depth) {
					const { args, meta, methodName, sectionName } = callInstance;
					let paramArgKvArray = [];
					if (!meta.args.length) {
						const sectionMethod = `${sectionName}.${methodName}`;
						callMethodArgs[sectionMethod] = null;
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
							argument = formatArgs(args[i], callMethodArgs, depth++); // go deeper into the nested calls
						} else if (
							args[i].toRawType() === 'Vec<AccountId>' ||
							args[i].toRawType() === 'Vec<Address>'
						) {
							// FIXME: lord forgive me for i have sinned. this is all a mess but rushing to get this out the door.
							for (let p = 0; p < args[i].length; p++) {
								args[i][p] = encodeAddress(
									decodeAddress(args[i][p].toString()),
									prefix
								);
							}
							argument = args[i];
						} else {
							argument = args[i].toString();
						}
						const param = meta.args[i].name.toString();
						const sectionMethod = `${sectionName}.${methodName}`;
						paramArgKvArray.push([param, argument]);
						callMethodArgs[sectionMethod] = paramArgKvArray;
					}
				}

				formatArgs(call, methodArgs, 0);
				setFormattedCallArgs(methodArgs);
			} catch (e) {
				alertDecodeError();
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
				<View style={styles.era}>
					<Text style={{ ...styles.subLabel, flex: 1 }}>phase: {phase} </Text>
					<Text style={{ ...styles.subLabel, flex: 1 }}>period: {period}</Text>
				</View>
			);
		} else {
			return (
				<View
					style={{
						display: 'flex',
						flexDirection: 'row',
						flexWrap: 'wrap'
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
			const formattedArgs = Object.entries(formattedCallArgs);

			// HACK: if there's a sudo method just put it to the front. Better way would be to order by depth but currently this is only relevant for a single extrinsic, so seems like overkill.
			for (let i = 1; i < formattedArgs.length; i++) {
				if (formattedArgs[i][0].includes('sudo')) {
					let tmp = formattedArgs[i];
					formattedArgs.splice(i, 1);
					formattedArgs.unshift(tmp);
					break;
				}
			}

			return formattedArgs.map((entry, index) => {
				const sectionMethod = entry[0];
				const paramArgs = entry[1];

				return (
					<View key={index} style={styles.callDetails}>
						<Text style={styles.subLabel}>
							Call <Text style={styles.titleText}>{sectionMethod}</Text> with
							the following arguments:
						</Text>
						{paramArgs ? (
							paramArgs.map(([param, arg]) => (
								<View key={param} style={styles.callDetails}>
									<Text style={styles.titleText}>
										{' { '}
										{param}:{' '}
										{arg && arg.length > 50
											? shortString(arg)
											: arg instanceof Array
											? arg.join(', ')
											: arg}{' '}
										{'}'}
									</Text>
								</View>
							))
						) : (
							<Text style={styles.secondaryText}>
								This method takes 0 arguments.
							</Text>
						)}
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
			<View style={{ marginBottom: 12, width: '100%' }}>
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
		marginTop: 8
	},
	callDetails: {
		marginBottom: 4
	},
	era: {
		flexDirection: 'row'
	},
	label: {
		...fontStyles.t_label,
		backgroundColor: colors.label_text,
		marginBottom: 10,
		paddingLeft: 8,
		textAlign: 'left'
	},
	secondaryText: {
		...fontStyles.t_codeS,
		color: colors.label_text,
		paddingHorizontal: 8,
		textAlign: 'left'
	},
	subLabel: {
		...fontStyles.t_codeS,
		color: colors.label_text,
		paddingLeft: 8,
		textAlign: 'left'
	},
	titleText: {
		...fontStyles.t_codeS,
		color: colors.label_text_sec
	}
});
