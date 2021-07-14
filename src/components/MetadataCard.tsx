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

import React, { ReactElement } from 'react';
import { StyleSheet, Text, View } from 'react-native';

import TouchableItem from 'components/TouchableItem';
//import CardSeparator from 'components/CardSeparator';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import Separator from 'components/Separator';
import { ButtonListener } from 'types/props';

type MetadataCardProps = {
	specName: string;
	specVersion: string;
	metadataHash: string;
	onPress?: ButtonListener;
};

const CardSeparator = (): ReactElement => (
	<Separator
		shadow={true}
		style={{
			backgroundColor: 'transparent',
			height: 0,
			marginVertical: 0
		}}
	/>
);

export function MetadataCard({
	specName,
	specVersion,
	metadataHash,
	onPress
}: MetadataCardProps): React.ReactElement {
	//	const { getNetwork } = useContext();

	return (
		<TouchableItem
			accessibilityComponentType="button"
			disabled={false}
			onPress={onPress}
		>
			<CardSeparator />
			<View style={styles.body}>
				<View style={styles.label}>
					<Text style={fontStyles.t_important}>Metadata version</Text>
				</View>
				<View style={styles.content}>
					<Text style={fontStyles.t_code}>
						{`spec_name: ${specName}\nspec_version: ${specVersion}\nhash: ${metadataHash}`}
					</Text>
				</View>
				<View />
			</View>
		</TouchableItem>
	);
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.background.app,
		flexDirection: 'row'
	},
	content: {
		alignItems: 'flex-start',
		flex: 3,
		justifyContent: 'center',
		padding: 20
	},
	desc: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'space-between',
		paddingLeft: 16
	},
	label: {
		alignItems: 'flex-start',
		flex: 1,
		justifyContent: 'center',
		padding: 20
	},
	text: {
		color: colors.signal.main
	}
});
