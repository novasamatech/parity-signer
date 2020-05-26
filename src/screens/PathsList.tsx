// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import { navigateToPathDerivation } from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NETWORK_LIST, UnknownNetworkKeys } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { PathGroup } from 'types/identityTypes';
import {
	isEthereumNetworkParams,
	isUnknownNetworkParams
} from 'types/networkSpecsTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { withAccountStore, withCurrentIdentity } from 'utils/HOC';
import {
	getPathsWithSubstrateNetworkKey,
	groupPaths,
	removeSlash
} from 'utils/identitiesUtils';
import ButtonNewDerivation from 'components/ButtonNewDerivation';
import PathCard from 'components/PathCard';
import Separator from 'components/Separator';
import fontStyles from 'styles/fontStyles';
import { LeftScreenHeading } from 'components/ScreenHeading';
import QrScannerTab from 'components/QrScannerTab';

function PathsList({
	accounts,
	navigation,
	route
}: NavigationAccountIdentityProps<'PathsList'>): React.ReactElement {
	const networkKey = route.params.networkKey ?? UnknownNetworkKeys.UNKNOWN;
	const networkParams = NETWORK_LIST[networkKey];

	const { currentIdentity } = accounts.state;
	const isEthereumPath = isEthereumNetworkParams(networkParams);
	const isUnknownNetworkPath = isUnknownNetworkParams(networkParams);
	const pathsGroups = useMemo((): PathGroup[] | null => {
		if (!currentIdentity || isEthereumPath) return null;
		const listedPaths = getPathsWithSubstrateNetworkKey(
			currentIdentity,
			networkKey
		);
		return groupPaths(listedPaths);
	}, [currentIdentity, isEthereumPath, networkKey]);
	const { isSeedRefValid } = useSeedRef(currentIdentity.encryptedSeed);

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
		<View key={`group${pathsGroup.title}`} style={{ marginVertical: 16 }}>
			<Separator
				shadow={true}
				style={{
					height: 0,
					marginVertical: 0
				}}
			/>
			<View
				style={{
					marginVertical: 16,
					paddingHorizontal: 16
				}}
			>
				<Text style={fontStyles.t_prefix}>{removeSlash(pathsGroup.title)}</Text>
				<Text style={fontStyles.t_codeS}>
					{networkParams.pathId}
					{pathsGroup.title}
				</Text>
			</View>
			{pathsGroup.paths.map(path => (
				<PathCard
					key={path}
					testID={testIDs.PathsList.pathCard + path}
					identity={currentIdentity}
					path={path}
					onPress={(): void => navigate('PathDetails', { path })}
				/>
			))}
		</View>
	);

	const subtitle =
		networkKey === UnknownNetworkKeys.UNKNOWN
			? ''
			: `//${networkParams.pathId}`;
	return (
		<>
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
			</SafeAreaScrollViewContainer>
			<ButtonNewDerivation
				testID={testIDs.PathsList.deriveButton}
				title="Derive New Account"
				onPress={(): Promise<void> =>
					navigateToPathDerivation(
						navigation,
						isUnknownNetworkPath ? '' : rootPath,
						isSeedRefValid
					)
				}
			/>
			<QrScannerTab />
		</>
	);
}

export default withAccountStore(withCurrentIdentity(PathsList));
