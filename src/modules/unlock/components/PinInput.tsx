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

import TextInput from 'components/TextInput';
import React, { MutableRefObject } from 'react';
import { KeyboardTypeOptions, StyleSheet, TextInputProps } from 'react-native';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';

interface PinInputProps extends TextInputProps {
	label: string;
	focus?: boolean;
	keyboardType?: KeyboardTypeOptions;
	ref?: MutableRefObject<TextInput | null>;
}

export default function PinInput(props: PinInputProps): React.ReactElement {
	return (
		<TextInput
			autoCorrect={false}
			editable
			keyboardAppearance="dark"
			keyboardType={props.keyboardType ?? 'numeric'}
			multiline={false}
			numberOfLines={1}
			returnKeyType="next"
			secureTextEntry
			{...props}
			style={StyleSheet.flatten([
				fontStyles.t_seed,
				styles.pinInput,
				{ fontSize: 18 },
				props.style
			])}
		/>
	);
}

const styles = StyleSheet.create({
	pinInput: {
		borderBottomColor: colors.border.light,
		borderColor: colors.border.light,
		minHeight: 48,
		paddingLeft: 10,
		paddingRight: 10
	}
});
