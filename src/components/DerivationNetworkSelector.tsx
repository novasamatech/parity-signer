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

import React, { useContext } from 'react';
import {
	Image,
	Platform,
	StyleSheet,
	Text,
	TouchableNativeFeedback,
	TouchableNativeFeedbackProps,
	TouchableOpacity,
	View
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import TransparentBackground from './TransparentBackground';

import { NetworksContext } from 'stores/NetworkContext';
import { SubstrateNetworkKeys } from 'constants/networkSpecs';
import fontStyles from 'styles/fontStyles';
import colors from 'styles/colors';

const ACCOUNT_NETWORK = 'Account Network';
const Touchable: React.ComponentClass<TouchableNativeFeedbackProps> =
	Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;

const excludedNetworks: string[] = [];
if (!__DEV__) {
	excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
	excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
}

export function DerivationNetworkSelector({
	networkKey,
	setVisible
}: {
	networkKey: string;
	setVisible: (shouldVisible: boolean) => void;
}): React.ReactElement {
	const { getSubstrateNetwork } = useContext(NetworksContext);
	const network = getSubstrateNetwork(networkKey);
	return (
		<View style={styles.body}>
			<Text style={styles.label}>{ACCOUNT_NETWORK}</Text>
			<Touchable onPress={(): void => setVisible(true)}>
				<View style={styles.triggerWrapper}>
					<Text style={styles.triggerLabel}>{network.title}</Text>
					<Icon name="more-vert" size={25} color={colors.text.main} />
				</View>
			</Touchable>
		</View>
	);
}

export function NetworkOptions({
	setNetworkKey,
	visible,
	setVisible
}: {
	setNetworkKey: (networkKey: string) => void;
	visible: boolean;
	setVisible: (shouldVisible: boolean) => void;
}): React.ReactElement {
	const { networks } = useContext(NetworksContext);
	const onNetworkSelected = (networkKey: string): void => {
		setNetworkKey(networkKey);
		setVisible(false);
	};

	const menuOptions = Array.from(networks.entries())
		.filter(([networkKey]) => !excludedNetworks.includes(networkKey))
		.map(([networkKey, networkParams]) => {
			return (
				<Touchable
					key={networkKey}
					onPress={(): void => onNetworkSelected(networkKey)}
				>
					<View style={styles.optionWrapper}>
						<Image source={networkParams.logo} style={styles.optionLogo} />
						<Text style={styles.optionText}>{networkParams.title}</Text>
					</View>
				</Touchable>
			);
		});

	return (
		<TransparentBackground
			style={styles.optionsWrapper}
			visible={visible}
			setVisible={setVisible}
			animationType="fade"
		>
			<View style={styles.optionsBackground}>
				<View style={{ ...styles.optionWrapper, borderTopWidth: 0 }}>
					<Text style={styles.optionHeadingText}>
						{ACCOUNT_NETWORK.toUpperCase()}
					</Text>
				</View>
				{menuOptions}
			</View>
		</TransparentBackground>
	);
}

const styles = StyleSheet.create({
	body: {
		flex: 1,
		marginBottom: 48,
		marginTop: 24,
		paddingHorizontal: 16
	},
	label: {
		flex: 1,
		marginBottom: 3,
		...fontStyles.t_regular
	},
	menuOption: {
		width: '100%'
	},
	optionHeadingText: {
		...fontStyles.h_subheading,
		paddingLeft: 16
	},
	optionLogo: {
		alignItems: 'center',
		height: 32,
		justifyContent: 'center',
		marginHorizontal: 16,
		width: 32
	},
	optionText: {
		...fontStyles.h2,
		color: colors.text.main
	},
	optionWrapper: {
		alignItems: 'center',
		borderTopColor: 'black',
		borderTopWidth: 1,
		flexDirection: 'row',
		paddingVertical: 8
	},
	optionsBackground: {
		backgroundColor: colors.background.app
	},
	optionsWrapper: {
		justifyContent: 'flex-end'
	},
	triggerLabel: {
		flex: 1,
		...fontStyles.h2
	},
	triggerWrapper: {
		alignItems: 'center',
		backgroundColor: colors.background.app,
		borderBottomColor: colors.border.light,
		borderBottomWidth: 0.8,
		flexDirection: 'row',
		height: 40
	}
});
