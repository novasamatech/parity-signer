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
import testIDs from 'e2e/testIDs';
import React, { useState } from 'react';
import { StyleSheet, Text, TouchableOpacity, View } from 'react-native';
import Icon from 'react-native-vector-icons/AntDesign';
import fontStyles from 'styles/fontStyles';
import { passwordRegex } from 'utils/regex';

export default function PasswordInput({ onSubmitEditing, password, setPassword }: {
	password: string;
	setPassword: (newPassword: string) => void;
	onSubmitEditing: () => void;
}): React.ReactElement {
	const onPasswordChange = (newPassword: string): void => {
		if (passwordRegex.test(newPassword)) setPassword(newPassword);
	};

	const [isShow, setShow] = useState<boolean>(false);
	const togglePasswordInput = (): void => setShow(!isShow);

	return (
		<View style={styles.container}>
			<TouchableOpacity
				onPress={togglePasswordInput}
				style={styles.label}
				testID={testIDs.PathDerivation.togglePasswordButton}
			>
				<Text style={fontStyles.t_regular}>Add Optional Password</Text>
				<Icon
					name={isShow ? 'caretup' : 'caretdown'}
					style={styles.labelIcon}
				/>
			</TouchableOpacity>
			{isShow && (
				<>
					<TextInput
						onChangeText={onPasswordChange}
						onSubmitEditing={onSubmitEditing}
						placeholder="Optional password"
						returnKeyType="done"
						testID={testIDs.PathDerivation.passwordInput}
						value={password}
					/>
					<Text style={styles.hintText}>
						Password will be always needed when signing with this account.
					</Text>
				</>
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	container: { marginBottom: 16 },
	hintText: {
		...fontStyles.t_regular,
		paddingHorizontal: 16
	},
	label: {
		alignItems: 'center',
		flexDirection: 'row',
		marginBottom: 3,
		paddingHorizontal: 16
	},
	labelIcon: {
		paddingLeft: 8,
		...fontStyles.t_regular
	}
});
