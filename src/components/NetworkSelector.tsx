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

'use strict';

import React from 'react';
import PropTypes from 'prop-types';
import {
	Image,
	Platform,
	StyleSheet,
	Text,
	TouchableNativeFeedback,
	TouchableOpacity,
	View
} from 'react-native';
import fontStyles from '../fontStyles';
import { SUBSTRATE_NETWORK_LIST, SubstrateNetworkKeys } from '../constants';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from '../colors';
import TransparentBackground from './TransparentBackground';
import fonts from '../fonts';

const ACCOUNT_NETWORK = 'Account Network';
const Touchable =
	Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;

NetworkSelector.protoTypes = {
	networkKey: PropTypes.string.isRequired,
	setVisible: PropTypes.func.isRequired
};

const excludedNetworks = [SubstrateNetworkKeys.KUSAMA_CC2];
if (!__DEV__) {
	excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
	excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
}

export function NetworkSelector({ networkKey, setVisible }) {
	return (
		<View style={styles.body}>
			<Text style={styles.label}>{ACCOUNT_NETWORK}</Text>
			<Touchable onPress={() => setVisible(true)}>
				<View style={styles.triggerWrapper}>
					<Text style={styles.triggerLabel}>
						{SUBSTRATE_NETWORK_LIST[networkKey].title}
					</Text>
					<Icon name="more-vert" size={25} color={colors.bg_text} />
				</View>
			</Touchable>
		</View>
	);
}

NetworkOptions.propTypes = {
	setNetworkKey: PropTypes.func.isRequired,
	setVisible: PropTypes.func.isRequired,
	visible: PropTypes.bool.isRequired
};

export function NetworkOptions({ setNetworkKey, visible, setVisible }) {
	const onNetworkSelected = networkKey => {
		setNetworkKey(networkKey);
		setVisible(false);
	};

	const menuOptions = Object.entries(SUBSTRATE_NETWORK_LIST)
		.filter(([networkKey]) => !excludedNetworks.includes(networkKey))
		.map(([networkKey, networkParams]) => {
			return (
				<Touchable
					key={networkKey}
					value={networkKey}
					onPress={() => onNetworkSelected(networkKey)}
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
		marginVertical: 8,
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
		color: colors.bg_text,
		fontFamily: fonts.robotoMedium,
		fontSize: 14,
		paddingLeft: 16
	},
	optionLogo: {
		alignItems: 'center',
		backgroundColor: colors.bg_text_sec,
		borderRadius: 30,
		height: 30,
		justifyContent: 'center',
		marginHorizontal: 16,
		width: 30
	},
	optionText: {
		color: colors.bg_text,
		...fontStyles.h2
	},
	optionWrapper: {
		alignItems: 'center',
		borderTopColor: 'black',
		borderTopWidth: 1,
		flexDirection: 'row',
		paddingVertical: 8
	},
	optionsBackground: {
		backgroundColor: colors.bg
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
		backgroundColor: colors.bg,
		borderBottomColor: colors.card_bg_text_sec,
		borderBottomWidth: 0.8,
		flex: 1,
		flexDirection: 'row',
		height: 40,
		paddingTop: 8
	}
});
