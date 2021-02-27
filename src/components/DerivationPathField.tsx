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

import React, { useCallback, useState } from 'react';
import { StyleSheet, Text, TextStyle, TouchableWithoutFeedback,View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { parseDerivationPath } from 'utils/suri';

import TextInput from './TextInput';

interface Props {
	onChange: (derivationEvent: { derivationPassword: string; derivationPath: string; isDerivationPathValid: boolean; }) => void;
	styles: { title: TextStyle; };
}

export default function DerivationPathField({ onChange, styles }: Props): React.ReactElement {
	const [showAdvancedField, setShowAdvancedField] = useState(false);
	const [isValidPath, setIsValidPath] = useState(true);

	const toggleShowAdvancedField = (): void => {
		setShowAdvancedField(!showAdvancedField);
	};

	const onChangeText = useCallback((text: string): void => {
		try {
			const derivationPath = parseDerivationPath(text);

			onChange({
				derivationPassword: derivationPath.password || '',
				derivationPath: derivationPath.derivePath || '',
				isDerivationPathValid: true
			});
			setIsValidPath(true);
		} catch (e) {
			// wrong derivationPath
			onChange({
				derivationPassword: '',
				derivationPath: '',
				isDerivationPathValid: false
			});
			setIsValidPath(false);
		}
	}, [onChange])

	return (
		<>
			<TouchableWithoutFeedback onPress={toggleShowAdvancedField}>
				<View style={ownStyles.container}>
					<Text
						style={StyleSheet.flatten([styles.title, ownStyles.advancedText])}
					>
						ADVANCED
					</Text>
					<Icon
						color={colors.text.main}
						name={showAdvancedField ? 'arrow-drop-up' : 'arrow-drop-down'}
						size={20}
					/>
				</View>
			</TouchableWithoutFeedback>
			{showAdvancedField && (
				<TextInput
					onChangeText={onChangeText}
					placeholder="optional derivation path"
					style={StyleSheet.flatten([
						fontStyles.h2,
						isValidPath ? {} : ownStyles.invalidInput
					])}
				/>
			)}
		</>
	);
}

const ownStyles = StyleSheet.create({
	advancedText: {
		paddingBottom: 0
	},
	container: {
		alignItems: 'center',
		flexDirection: 'row'

	},
	invalidInput: {
		borderBottomColor: colors.signal.error
	}
});
