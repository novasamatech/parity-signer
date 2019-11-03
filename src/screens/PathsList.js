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
import { ScrollView, Text, View } from 'react-native';
import { UnknownNetworkKeys } from '../constants';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { getPathsWithNetwork, groupPaths } from '../util/identitiesUtils';
import Button from '../components/Button';
import PathCard from '../components/PathCard';

function PathsList({ accounts, navigation }) {
	const networkKey = navigation.getParam(
		'networkKey',
		UnknownNetworkKeys.UNKNOWN
	);
	const { currentIdentity } = accounts.state;
	const paths = Array.from(currentIdentity.meta.keys());
	const listedPaths = getPathsWithNetwork(paths, networkKey);
	const pathsGroups = groupPaths(listedPaths);
	const { navigate } = navigation;

	const renderSinglePath = pathsGroup => {
		const path = pathsGroup.paths[0];
		return (
			<PathCard
				identity={currentIdentity}
				path={path}
				onPress={() => navigate('PathDetails', { path })}
			/>
		);
	};

	const renderGroupPaths = pathsGroup => (
		<View>
			<Text>{pathsGroup.title}</Text>
			{pathsGroup.paths.map(path => (
				<PathCard
					identity={currentIdentity}
					path={path}
					onPress={() => navigate('PathDetails', { path })}
				/>
			))}
		</View>
	);

	return (
		<ScrollView>
			{pathsGroups.map(pathsGroup =>
				pathsGroup.paths.length === 1
					? renderSinglePath(pathsGroup)
					: renderGroupPaths(pathsGroup)
			)}
			<Button
				title="Create New Derivation"
				onPress={() => navigation.navigate('PathDerivation', { networkKey })}
			/>
			<Button title="Scan" onPress={() => {}} />
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(PathsList));
