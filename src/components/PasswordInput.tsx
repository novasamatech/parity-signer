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

import React from 'react';

import TextInput from 'components/TextInput';
import { passwordRegex } from 'utils/regex';

export default function PasswordInput({
	password,
	setPassword,
	testID,
	onSubmitEditing
}: {
	password: string;
	setPassword: (newPassword: string) => void;
	testID?: string;
	onSubmitEditing: () => void;
}): React.ReactElement {
	const onPasswordChange = (newPassword: string): void => {
		if (passwordRegex.test(newPassword)) setPassword(newPassword);
	};

	return (
		<TextInput
			onChangeText={onPasswordChange}
			testID={testID}
			label="Advanced Option"
			onSubmitEditing={onSubmitEditing}
			placeholder="Optional password"
			value={password}
		/>
	);
}
