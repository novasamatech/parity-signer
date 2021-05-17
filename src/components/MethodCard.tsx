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

// This component recursively renders all possible nested methods with their arguments

import React, { ReactElement } from 'react';
import { StyleSheet, View, Text } from 'react-native';

import { SanitizedCall } from 'types/payloads';
import fontStyles from 'styles/fontStyles';
import colors from 'styles/colors';

// Function to render arguments - and if arguments contain other calls,
// recursively call another MethodCard
function renderArgs(
	argObject: { [key: string]: unknown },
	argsDepth: number
): React.ReactNode {
	return Object.keys(argObject).map((key: string) => {
		if (key === 'call') {
			return (
				<MethodCard
					renderCall={argObject[key] as SanitizedCall}
					depth={argsDepth}
				/>
			);
		} else if (
			key === 'calls' &&
			argObject[key] &&
			typeof argObject[key] === 'object'
		) {
			return (argObject[key] as SanitizedCall[]).map(
				(recursiveCall: SanitizedCall): React.ReactNode => {
					return <MethodCard renderCall={recursiveCall} depth={argsDepth} />;
				}
			);
		} else {
			//TODO: more nice rendering options for args
			const stringifiedArgs = JSON.stringify(argObject[key], null, 2);
			return (
				<View>
					<Text
						style={[
							styles.titleText,
							{
								paddingLeft: argsDepth + '0%'
							}
						]}
					>
						{key} : <Text style={styles.secondaryText}>{stringifiedArgs}</Text>
					</Text>
				</View>
			);
		}
	});
}

type MethodCardArgs = {
	renderCall: SanitizedCall;
	depth: number;
};

// Don't use virtualized views for hierarchical list to avoid surprises
export function MethodCard({
	renderCall,
	depth
}: MethodCardArgs): ReactElement {
	// no depth limiter since it is already implemented for the only call generator;
	// If something fails, RN will just drop the app, which is probably the safest
	// course of action to protect user's keys

	return (
		<View>
			{typeof renderCall.method === 'object' ? (
				<View>
					<Text
						style={[
							styles.titleText,
							{
								paddingLeft: depth + '0%'
							}
						]}
					>
						{renderCall.method.method}
						<Text style={styles.secondaryText}> from </Text>
						{renderCall.method.pallet}
					</Text>
					{renderArgs(renderCall.args, depth + 1)}
				</View>
			) : (
				<View>
					<Text style={styles.warningText}>{renderCall.method}</Text>
				</View>
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	secondaryText: {
		...fontStyles.t_codeS,
		color: colors.signal.main,
		paddingHorizontal: 8,
		textAlign: 'left'
	},
	sublabel: {
		...fontStyles.t_label,
		backgroundColor: colors.signal.main,
		color: colors.background.app,
		marginBottom: 10,
		paddingLeft: 8,
		textAlign: 'left'
	},
	titleText: {
		...fontStyles.t_codeS,
		color: colors.text.main,
		paddingHorizontal: 16
	},
	warningText: {
		...fontStyles.t_codeS,
		color: colors.signal.main
	}
});
