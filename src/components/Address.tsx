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

import React, { ReactElement } from 'react';
import { StyleSheet, Text, TextStyle } from 'react-native';

import colors from 'styles/colors';
import fonts from 'styles/fonts';
import fontStyles from 'styles/fontStyles';
import { NetworkProtocols } from 'constants/networkSpecs';
import { NetworkProtocol } from 'types/networkSpecsTypes';

export default function Address(props: {
	address: string;
	protocol?: NetworkProtocol;
	style?: TextStyle;
}): ReactElement {
	const { address, protocol = NetworkProtocols.SUBSTRATE, style = {} } = props;
	const prefix = protocol === NetworkProtocols.ETHEREUM ? '0x' : '';

	return (
		<Text
			numberOfLines={1}
			style={[styles.secondaryText, style, fontStyles.t_codeS]}
			ellipsizeMode="middle"
		>
			{prefix}
			{address}
		</Text>
	);
}

const styles = StyleSheet.create({
	secondaryText: {
		color: colors.bg_text_sec,
		fontFamily: fonts.regular,
		fontSize: 12,
		lineHeight: 16
	}
});
