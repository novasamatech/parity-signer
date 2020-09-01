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

import { GenericExtrinsicPayload } from '@polkadot/types';
import Call from '@polkadot/types/generic/Call';
import { formatBalance } from '@polkadot/util';
import { decodeAddress, encodeAddress } from '@polkadot/util-crypto';
import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, Text, View, ViewStyle } from 'react-native';
import { AnyU8a, IExtrinsicEra, IMethod } from '@polkadot/types/types';
import { ExtrinsicEra } from '@polkadot/types/interfaces';

import { AlertStateContext } from 'stores/alertContext';
import { NetworksContext } from 'stores/NetworkContext';
import { RegistriesStoreState } from 'stores/RegistriesContext';
import colors from 'styles/colors';
import { withRegistriesStore } from 'utils/HOC';
import { shortString } from 'utils/strings';
import fontStyles from 'styles/fontStyles';
import { alertDecodeError } from 'utils/alertUtils';

const recodeAddress = (encodedAddress: string, prefix: number): string =>
	encodeAddress(decodeAddress(encodedAddress), prefix);

type ExtrinsicPartProps = {
	fallback?: string;
	label: string;
	networkKey: string;
	registriesStore: RegistriesStoreState;
	value: AnyU8a | IMethod | IExtrinsicEra;
};

const ExtrinsicPart = withRegistriesStore<ExtrinsicPartProps>(
	({
		fallback,
		label,
		networkKey,
		registriesStore,
		value
	}: ExtrinsicPartProps): React.ReactElement => {
		const [period, setPeriod] = useState<string>();
		const [phase, setPhase] = useState<string>();
		const [formattedCallArgs, setFormattedCallArgs] = useState<any>();
		const [tip, setTip] = useState<string>();
		const [useFallback, setUseFallBack] = useState(false);
		const { setAlert } = useContext(AlertStateContext);
		const { networks, getSubstrateNetwork } = useContext(NetworksContext);
		const networkParams = getSubstrateNetwork(networkKey);
		const prefix = networkParams.prefix;

		useEffect(() => {
			if (label === 'Method' && !fallback) {
				try {
					const registry = registriesStore.get(networks, networkKey);
					const call = registry.createType('Call', value);

					const methodArgs = {};

					function formatArgs(
						callInstance: Call,
						callMethodArgs: any,
						depth: number
					): void {
						const { args, meta, methodName, sectionName } = callInstance;
						const paramArgKvArray = [];
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
								argument = recodeAddress(args[i].toString(), prefix);
							} else if (args[i] instanceof Call) {
								argument = formatArgs(args[i] as Call, callMethodArgs, depth++); // go deeper into the nested calls
							} else if (
								args[i].toRawType() === 'Vec<AccountId>' ||
								args[i].toRawType() === 'Vec<Address>'
							) {
								argument = (args[i] as any).map((v: any) =>
									recodeAddress(v.toString(), prefix)
								);
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
		}, [
			fallback,
			label,
			prefix,
			value,
			networkKey,
			registriesStore,
			setAlert,
			networks
		]);

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
							{value.toString()}
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
						<View key={index} style={styles.callDetails}>
							<Text style={styles.secondaryText}>
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
							{useFallback ? value.toString() : value}
						</Text>
					)}
				</View>
			</View>
		);
	}
);

interface PayloadDetailsCardProps {
	description?: string;
	payload?: GenericExtrinsicPayload;
	signature?: string;
	style?: ViewStyle;
	networkKey: string;
}

export default function PayloadDetailsCard(
	props: PayloadDetailsCardProps
): React.ReactElement {
	const { networks, getSubstrateNetwork } = useContext(NetworksContext);
	const { networkKey, description, payload, signature, style } = props;
	const isKnownNetworkKey = networks.has(networkKey);
	const fallback = !isKnownNetworkKey;
	const networkParams = getSubstrateNetwork(networkKey);

	if (isKnownNetworkKey) {
		formatBalance.setDefaults({
			decimals: networkParams.decimals,
			unit: networkParams.unit
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
						value={fallback ? payload.method.toString() : payload.method}
					/>
					<ExtrinsicPart
						label="Block Hash"
						networkKey={networkKey}
						value={payload.blockHash.toString()}
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
					<ExtrinsicPart
						label="Genesis Hash"
						networkKey={networkKey}
						value={payload.genesisHash.toString()}
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
	era: {
		flexDirection: 'row'
	},
	extrinsicContainer: {
		paddingTop: 16
	},
	label: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.main,
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
