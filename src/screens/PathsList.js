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
import { Text } from 'react-native';
import { UnknownNetworkKeys } from '../constants';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { getPathsWithNetwork } from '../util/identitiesUtils';

function PathsList({ accounts, navigation }) {
	const networkKey = navigation.getParam(
		'networkKey',
		UnknownNetworkKeys.UNKNOWN
	);
	const paths = Array.from(accounts.state.currentIdentity.meta.keys());
	const listedPath = getPathsWithNetwork(paths, networkKey);
	return (
		<>
			{listedPath.map(path => (
				<Text>{path}</Text>
			))}
		</>
	);
}

export default withAccountStore(withNavigation(PathsList));
