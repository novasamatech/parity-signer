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

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NavigationAccountProps } from 'types/props';
import { withAccountStore } from 'utils/HOC';
import TextInput from 'components/TextInput';
import PathCard from 'components/PathCard';

function PathManagement({
	accounts,
	route
}: NavigationAccountProps<'PathManagement'>): React.ReactElement {
	const path = route.params.path ?? '';
	const { currentIdentity } = accounts.state;
	const pathName = currentIdentity!.meta.get(path)?.name;

	return (
		<SafeAreaScrollViewContainer>
			<PathCard identity={currentIdentity!} path={path} />
			<TextInput
				label="Display Name"
				onChangeText={(name: string): Promise<void> =>
					accounts.updatePathName(path, name)
				}
				value={pathName}
				placeholder="Enter a new account name"
				focus={true}
			/>
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountStore(PathManagement);
