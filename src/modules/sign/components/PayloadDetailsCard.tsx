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

import { GenericExtrinsicPayload, GenericCall, Struct } from '@polkadot/types';
import type { Call, ExtrinsicEra } from '@polkadot/types/interfaces';
import {
	AnyJson,
	AnyU8a,
	Codec,
	IExtrinsicEra,
	IMethod
} from '@polkadot/types/types';
import { formatBalance } from '@polkadot/util';
import { decodeAddress, encodeAddress } from '@polkadot/util-crypto';
import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, Text, View, ViewStyle } from 'react-native';

import { AlertStateContext } from 'stores/alertContext';
import { NetworksContext, NetworksContextState } from 'stores/NetworkContext';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { alertDecodeError } from 'utils/alertUtils';
import { withRegistriesStore } from 'utils/HOC';
import { shortString } from 'utils/strings';

const recodeAddress = (encodedAddress: string, prefix: number): string =>
	encodeAddress(decodeAddress(encodedAddress), prefix);

type ExtrinsicPartProps = {
	fallback?: string;
	label: string;
	networkKey: string;
	registriesStore: NetworksContextState;
	value: AnyJson | AnyU8a | IMethod | IExtrinsicEra;
};

type FrameMethod = {
	method: string;
	pallet: string;
};

type SanitizedArgs = {
	[key: string]: unknown;
	call?: SanitizedCall;
	calls?: SanitizedCall[];
};

type SanitizedCall = {
	[key: string]: unknown;
	args: SanitizedArgs;
	callIndex?: Uint8Array | string;
	method: string | FrameMethod;
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
		const { networks, getSubstrateNetwork, getTypeRegistry } = useContext(
			NetworksContext
		);
		const networkParams = getSubstrateNetwork(networkKey);
		const prefix = networkParams.prefix;
		const typeRegistry = getTypeRegistry(networkKey)!;

		function parseArrayGenericCalls(
			argsArray: Codec[]
		): (Codec | SanitizedCall)[] {
			return argsArray.map(argument => {
				if (argument instanceof GenericCall) {
					return parseGenericCall(argument);
				}

				return argument;
			});
		}

		function parseGenericCall(genericCall: GenericCall): SanitizedCall {
			const newArgs: SanitizedArgs = {};

			// Pull out the struct of arguments to this call
			const callArgs = genericCall.get('args') as Struct;

			// Make sure callArgs exists and we can access its keys
			if (callArgs && callArgs.defKeys) {
				// paramName is a string
				for (const paramName of callArgs.defKeys) {
					const argument = callArgs.get(paramName);

					if (Array.isArray(argument)) {
						newArgs[paramName] = parseArrayGenericCalls(argument);
					} else if (argument instanceof GenericCall) {
						newArgs[paramName] = parseGenericCall(argument);
					} else if (
						paramName === 'call' &&
						argument?.toRawType() === 'Bytes'
					) {
						// multiSig.asMulti.args.call is an OpaqueCall (Vec<u8>) that we
						// serialize to a polkadot-js Call and parse so it is not a hex blob.
						try {
							const call = typeRegistry.createType('Call', argument.toHex());
							newArgs[paramName] = parseGenericCall(call);
						} catch {
							newArgs[paramName] = (argument as any) as SanitizedCall;
						}
					} else {
						newArgs[paramName] = (argument as any) as SanitizedCall;
					}
				}
			}

			return {
				args: newArgs,
				method: {
					method: genericCall.method,
					pallet: genericCall.section
				}
			};
		}

		useEffect(() => {
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
			typeRegistry,
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
							{value?.toString()}
						</Text>
					</View>
				);
			}
		};

		const renderMethodDetails = (): React.ReactNode => {
			const call = typeRegistry.createType('Call', value);
			const parsed = JSON.stringify(parseGenericCall(call), null, 2);
			return (
				<View style={styles.callDetails}>
					<Text style={styles.titleText}>{parsed}</Text>
				</View>
			);
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
	era: {
		flexDirection: 'row'
	},
	extrinsicContainer: {
		paddingTop: 16
	},
	label: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.main,
		color: colors.background.app,
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
		color: colors.text.main,
		paddingHorizontal: 16
	}
});
