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

import React, { useMemo } from 'react';
import { Text, View } from 'react-native';

import { PathDetailsView } from './PathDetails';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import {
	NETWORK_LIST,
	NetworkProtocols,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { PathGroup } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { NavigationAccountProps } from 'types/props';
import { withAccountStore } from 'utils/HOC';
import {
	getPathsWithSubstrateNetworkKey,
	groupPaths,
	removeSlash
} from 'utils/identitiesUtils';
import ButtonNewDerivation from 'components/ButtonNewDerivation';
import PathCard from 'components/PathCard';
import Separator from 'components/Separator';
import fontStyles from 'styles/fontStyles';
import colors from 'styles/colors';
import { LeftScreenHeading } from 'components/ScreenHeading';

function PathsList({
	accounts,
	navigation,
	route
}: NavigationAccountProps<'PathsList'>): React.ReactElement {
	const networkKey = route.params.networkKey ?? UnknownNetworkKeys.UNKNOWN;
	const networkParams = NETWORK_LIST[networkKey];

	const { currentIdentity } = accounts.state;
	const isEthereumPath = isEthereumNetworkParams(networkParams);
	const isUnknownNetworkPath =
		networkParams.protocol === NetworkProtocols.UNKNOWN;
	const pathsGroups = useMemo((): PathGroup[] | null => {
		if (!currentIdentity || isEthereumPath) return null;
		const paths = Array.from(currentIdentity.meta.keys());
		const listedPaths = getPathsWithSubstrateNetworkKey(paths, networkKey);
		return groupPaths(listedPaths);
	}, [currentIdentity, isEthereumPath, networkKey]);

	if (!currentIdentity) return <View />;
	if (isEthereumNetworkParams(networkParams)) {
		return (
			<PathDetailsView
				networkKey={networkKey}
				path={networkKey}
				navigation={navigation}
				accounts={accounts}
			/>
		);
	}

	const { navigate } = navigation;
	const rootPath = `//${networkParams.pathId}`;

	const renderSinglePath = (pathsGroup: PathGroup): React.ReactElement => {
		const path = pathsGroup.paths[0];
		return (
			<PathCard
				key={path}
				testID={testIDs.PathsList.pathCard + path}
				identity={currentIdentity}
				path={path}
				onPress={(): void => navigate('PathDetails', { path })}
			/>
		);
	};

	const renderGroupPaths = (pathsGroup: PathGroup): React.ReactElement => (
		<View key={`group${pathsGroup.title}`} style={{ marginTop: 24 }}>
			<View
				style={{
					backgroundColor: colors.bg,
					height: 64,
					marginBottom: 14
				}}
			>
				<Separator
					shadow={true}
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
						marginTop: 16,
						paddingHorizontal: 16
					}}
				>
					<View>
						<Text style={fontStyles.t_prefix}>
							{removeSlash(pathsGroup.title)}
						</Text>
						<Text style={fontStyles.t_codeS}>
							{networkParams.pathId}
							{pathsGroup.title}
						</Text>
					</View>
				</View>
			</View>
			{pathsGroup.paths.map(path => (
				<View key={path} style={{ marginBottom: -8 }}>
					<PathCard
						key={path}
						testID={testIDs.PathsList.pathCard + path}
						identity={currentIdentity}
						path={path}
						onPress={(): void => navigate('PathDetails', { path })}
					/>
				</View>
			))}
		</View>
	);

	const subtitle =
		networkKey === UnknownNetworkKeys.UNKNOWN
			? ''
			: `//${networkParams.pathId}`;
	return (
		<SafeAreaScrollViewContainer testID={testIDs.PathsList.screen}>
			<LeftScreenHeading
				title={networkParams.title}
				subtitle={subtitle}
				hasSubtitleIcon={true}
				networkKey={networkKey}
			/>
			{(pathsGroups as PathGroup[]).map(pathsGroup =>
				pathsGroup.paths.length === 1
					? renderSinglePath(pathsGroup)
					: renderGroupPaths(pathsGroup)
			)}
			<ButtonNewDerivation
				testID={testIDs.PathsList.deriveButton}
				title="Derive New Account"
				onPress={(): void =>
					navigation.navigate('PathDerivation', {
						parentPath: isUnknownNetworkPath ? '' : rootPath
					})
				}
			/>
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountStore(PathsList);
