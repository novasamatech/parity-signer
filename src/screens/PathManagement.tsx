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

import PathCard from 'components/PathCard';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import TextInput from 'components/TextInput';
import React from 'react';
import { NavigationAccountIdentityProps } from 'types/props';
import { withCurrentIdentity } from 'utils/HOC';

function PathManagement({ accountsStore, route }: NavigationAccountIdentityProps<'PathManagement'>): React.ReactElement {
	const path = route.params.path ?? '';
	const { currentIdentity } = accountsStore.state;
	const pathName = currentIdentity.meta.get(path)?.name;

	return (
		<SafeAreaScrollViewContainer>
			<PathCard identity={currentIdentity}
				path={path} />
			<TextInput
				focus={true}
				label="Display Name"
				onChangeText={(name: string): void =>
					accountsStore.updatePathName(path, name)
				}
				placeholder="Enter a new account name"
				value={pathName}
			/>
		</SafeAreaScrollViewContainer>
	);
}

export default withCurrentIdentity(PathManagement);
