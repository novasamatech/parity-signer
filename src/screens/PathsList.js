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

import {
	NETWORK_LIST,
	NetworkProtocols,
	UnknownNetworkKeys
} from '../constants';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import {
	getPathsWithSubstrateNetwork,
	groupPaths
} from '../util/identitiesUtils';
import Button from '../components/Button';
import PathCard from '../components/PathCard';
import { PathDetailsView } from './PathDetails';
import testIDs from '../../e2e/testIDs';

import Separator from '../components/Separator';
import fontStyles from '../fontStyles';
import colors from '../colors';
import ButtonIcon from '../components/ButtonIcon';

function PathsList({ accounts, navigation }) {
	const networkKey = navigation.getParam(
		'networkKey',
		UnknownNetworkKeys.UNKNOWN
	);
	if (NETWORK_LIST[networkKey].protocol !== NetworkProtocols.SUBSTRATE) {
		return (
			<PathDetailsView
				networkKey={networkKey}
				path={networkKey}
				navigation={navigation}
				accounts={accounts}
			/>
		);
	}
	const { currentIdentity } = accounts.state;
	const paths = Array.from(currentIdentity.meta.keys());
	const listedPaths = getPathsWithSubstrateNetwork(paths, networkKey);
	const pathsGroups = groupPaths(listedPaths);
	const { navigate } = navigation;

	const renderSinglePath = pathsGroup => {
		const path = pathsGroup.paths[0];
		return (
			<PathCard
				key={path}
				testID={testIDs.PathList.pathCard + path}
				identity={currentIdentity}
				path={path}
				onPress={() => navigate('PathDetails', { path })}
			/>
		);
	};

	const renderGroupPaths = pathsGroup => (
		<>
			<View
				style={{
					backgroundColor: colors.bg,
					height: 64
				}}
			>
				<Separator
					shadow={true}
					shadowStyle={{ height: 16, marginTop: -16 }}
					style={{
						backgroundColor: 'transparent',
						height: 0,
						marginVertical: 0
					}}
				/>
				<View
					style={{
						alignItems: 'center',
						flexDirection: 'row',
						justifyContent: 'space-between',
						marginBottom: 20,
						marginTop: 16,
						paddingHorizontal: 16
					}}
				>
					<View>
						<Text style={fontStyles.t_prefix}>{pathsGroup.title}</Text>
						<Text style={fontStyles.t_codeS}>networkPath+pathsGroup.path</Text>
					</View>

					<ButtonIcon
						onPress={() => this.props.navigation.navigate('')}
						iconName="md-add"
						iconType="ionicon"
						style={{ opacity: 0.3 }}
						// onPress should lead to prefilled PathDerivation form
						// eslint-disable-next-line react/jsx-no-duplicate-props
						onPress={() =>
							navigation.navigate('PathDerivation', { networkKey })
						}
					/>
				</View>
			</View>
			{pathsGroup.paths.map(path => (
				<PathCard
					key={path}
					testID={testIDs.PathList.pathCard + path}
					identity={currentIdentity}
					path={path}
					onPress={() => navigate('PathDetails', { path })}
				/>
			))}
		</>
	);

	return (
		<ScrollView style={{ backgroundColor: colors.bg, flex: 1 }}>
			{pathsGroups.map(pathsGroup =>
				pathsGroup.paths.length === 1
					? renderSinglePath(pathsGroup)
					: renderGroupPaths(pathsGroup)
			)}
			<Button
				testID={testIDs.PathList.deriveButton}
				title="Create New Derivation"
				onPress={() => navigation.navigate('PathDerivation', { networkKey })}
			/>
			<Button title="Scan" onPress={() => navigate('QrScanner')} />
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(PathsList));
