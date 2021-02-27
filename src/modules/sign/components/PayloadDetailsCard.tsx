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

import type { Call, ExtrinsicEra } from '@polkadot/types/interfaces';

import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, Text, View, ViewStyle } from 'react-native';
import { RegistriesContext, RegistriesStoreState } from 'stores/RegistriesContext';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { isSubstrateNetwork } from 'types/networkTypes';
import { alertDecodeError } from 'utils/alertUtils';
import { withRegistriesStore } from 'utils/HOC';
import { shortString } from 'utils/strings';

import { GenericExtrinsicPayload } from '@polkadot/types';
import { AnyJson, AnyU8a, IExtrinsicEra, IMethod } from '@polkadot/types/types';
import { formatBalance } from '@polkadot/util';
import { decodeAddress, encodeAddress } from '@polkadot/util-crypto';

import { AlertContext, NetworksContext } from '../../../context';

const recodeAddress = (encodedAddress: string, prefix: number): string =>
	encodeAddress(decodeAddress(encodedAddress), prefix);

type ExtrinsicPartProps = {
	fallback?: string;
	label: string;
	networkKey: string;
	registriesStore: RegistriesStoreState;
	value: AnyJson | AnyU8a | IMethod | IExtrinsicEra;
};

const ExtrinsicPart = withRegistriesStore<ExtrinsicPartProps>(({ fallback, label, networkKey, registriesStore, value }: ExtrinsicPartProps): React.ReactElement => {
	const [period, setPeriod] = useState<string>();
	const [phase, setPhase] = useState<string>();
	const [formattedCallArgs, setFormattedCallArgs] = useState<any>();
	const [tip, setTip] = useState<string>();
	const [useFallback, setUseFallBack] = useState(false);
	const { getTypeRegistry } = useContext(RegistriesContext);
	const { setAlert } = useContext(AlertContext);
	const { getSubstrateNetwork, networks } = useContext(NetworksContext);
	const networkParams = getSubstrateNetwork(networkKey);
	const prefix = networkParams.prefix;
	const typeRegistry = getTypeRegistry(networks, networkKey)!;

	useEffect(() => {

		if (label === 'Method' && !fallback) {
			try {
				const call = typeRegistry.createType('Call', value);
				const methodArgs = {};

				function formatArgs(callInstance: Call,
					callMethodArgs: any,
					depth: number): void {
					const { args, meta } = callInstance;
					const paramArgKvArray = [];

					if (!meta.args.length) {
						const sectionMethod = `${call.method}.${call.section}`;

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
							argument = recodeAddress(args[i].toString(), prefix);
						} else if ((args[i] as Call).section) {
							argument = formatArgs(args[i] as Call, callMethodArgs, depth++); // go deeper into the nested calls
						} else if (
							args[i].toRawType() === 'Vec<AccountId>' ||
								args[i].toRawType() === 'Vec<Address>'
						) {
							argument = (args[i] as any).map((v: any) =>
								recodeAddress(v.toString(), prefix));
						} else {
							argument = args[i].toString();
						}

						const param = meta.args[i].name.toString();
						const sectionMethod = `${call.method}.${call.section}`;

						paramArgKvArray.push([param, argument]);
						callMethodArgs[sectionMethod] = paramArgKvArray;
					}
				}

				formatArgs(call, methodArgs, 0);
				setFormattedCallArgs(methodArgs);
			} catch (e) {
				alertDecodeError(setAlert);
				setUseFallBack(true);
			}
		}

		if (label === 'Era' && !fallback) {
			if ((value as ExtrinsicEra).isMortalEra) {
				setPeriod((value as ExtrinsicEra).asMortalEra.period.toString());
				setPhase((value as ExtrinsicEra).asMortalEra.phase.toString());
			}
		}

		if (label === 'Tip' && !fallback) {
			setTip(formatBalance(value as any));
		}
	}, [fallback, label, prefix, setAlert, typeRegistry, value]);

	const renderEraDetails = (): React.ReactElement => {
		if (period && phase) {
			return (
				<View style={styles.era}>
					<Text style={{ ...styles.secondaryText, flex: 1 }}>
							phase: {phase}{' '}
					</Text>
					<Text style={{ ...styles.secondaryText, flex: 1 }}>
							period: {period}
					</Text>
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
					<Text style={{ ...styles.secondaryText, flex: 1 }}>
							Immortal Era
					</Text>
					<Text style={{ ...styles.secondaryText, flex: 3 }}>
						{value?.toString()}
					</Text>
				</View>
			);
		}
	};

		type ArgsList = Array<[string, any]>;
		type MethodCall = [string, ArgsList];
		type FormattedArgs = Array<MethodCall>;

		const renderMethodDetails = (): React.ReactNode => {
			if (formattedCallArgs) {
				const formattedArgs: FormattedArgs = Object.entries(formattedCallArgs);

				// HACK: if there's a sudo method just put it to the front. Better way would be to order by depth but currently this is only relevant for a single extrinsic, so seems like overkill.
				for (let i = 1; i < formattedArgs.length; i++) {
					if (formattedArgs[i][0].includes('sudo')) {
						const tmp = formattedArgs[i];

						formattedArgs.splice(i, 1);
						formattedArgs.unshift(tmp);
						break;
					}
				}

				return formattedArgs.map((entry, index) => {
					const sectionMethod = entry[0];
					const paramArgs: Array<[any, any]> = entry[1];

					return (
						<View key={index}
							style={styles.callDetails}>
							<Text style={styles.secondaryText}>
								Call <Text style={styles.titleText}>{sectionMethod}</Text> with
								the following arguments:
							</Text>
							{paramArgs ? (
								paramArgs.map(([param, arg]) => (
									<View key={param}
										style={styles.callDetails}>
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

		const renderTipDetails = (): React.ReactElement => {
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
							{useFallback ? value?.toString() : value}
						</Text>
					)}
				</View>
			</View>
		);
});

interface PayloadDetailsCardProps {
	description?: string;
	payload?: GenericExtrinsicPayload;
	signature?: string;
	style?: ViewStyle;
	networkKey: string;
}

const PayloadDetailsCard = ({ description, networkKey, payload, signature, style }: PayloadDetailsCardProps): React.ReactElement =>  {
	const { getNetwork } = useContext(NetworksContext);
	const network = getNetwork(networkKey);
	const fallback = !network;

	if (isSubstrateNetwork(network)) {
		formatBalance.setDefaults({
			decimals: network.decimals,
			unit: network.unit
		});
	}

	return (
		<View style={[styles.body, style]}>
			{!!description && <Text style={styles.titleText}>{description}</Text>}
			{!!payload && (
				<View style={styles.extrinsicContainer}>
					<ExtrinsicPart
						label="Method"
						networkKey={networkKey}
						value={fallback ? payload.method.toHuman() : payload.method}
					/>
					<ExtrinsicPart
						label="Era"
						networkKey={networkKey}
						value={fallback ? payload.era.toString() : payload.era}
					/>
					<ExtrinsicPart
						label="Nonce"
						networkKey={networkKey}
						value={payload.nonce.toString()}
					/>
					<ExtrinsicPart
						label="Tip"
						networkKey={networkKey}
						value={payload.tip.toString()}
					/>
				</View>
			)}
			{!!signature && (
				<View style={styles.extrinsicContainer}>
					<Text style={styles.label}>Signature</Text>
					<Text style={styles.secondaryText}>{signature}</Text>
				</View>
			)}
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
	era: { flexDirection: 'row' },
	extrinsicContainer: {
		paddingTop: 16
	},
	label: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.main,
		color: colors.text.main,
		marginBottom: 10,
		paddingLeft: 8,
		textAlign: 'left'
	},
	secondaryText: {
		...fontStyles.t_codeS,
		color: colors.signal.main,
		paddingHorizontal: 8,
		textAlign: 'left'
	},
	titleText: {
		...fontStyles.t_codeS,
		color: colors.text.main
	}
});

export default PayloadDetailsCard;
