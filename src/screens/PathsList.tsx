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
import PathGroupCard from 'components/PathGroupCard';
import QRScannerAndDerivationTab from 'components/QRScannerAndDerivationTab';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { LeftScreenHeading } from 'components/ScreenHeading';
import Separator from 'components/Separator';
import { UnknownNetworkKeys } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import React, { useContext, useMemo } from 'react';
import { ScrollView } from 'react-native';
import { PathGroup } from 'types/identityTypes';
import { isEthereumNetwork, isUnknownNetworkParams } from 'types/networkTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { withCurrentIdentity } from 'utils/HOC';
import { getPathsWithSubstrateNetworkKey, groupPaths } from 'utils/identitiesUtils';
import { useUnlockSeed } from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';

import { NetworksContext } from '../context';
import { PathDetailsView } from './PathDetails';

function PathsList({ accountsStore, navigation, route }: NavigationAccountIdentityProps<'PathsList'>): React.ReactElement {
	const networkKey = route.params.networkKey ?? UnknownNetworkKeys.UNKNOWN;
	const networkContextState = useContext(NetworksContext);
	const { getNetwork, networks } = networkContextState;
	const networkParams = getNetwork(networkKey);

	const { currentIdentity } = accountsStore.state;
	const isEthereumPath = isEthereumNetwork(networkParams);
	const isUnknownNetworkPath = isUnknownNetworkParams(networkParams);
	const pathsGroups = useMemo((): PathGroup[] | null => {
		if (!currentIdentity || isEthereumPath) return null;
		const listedPaths = getPathsWithSubstrateNetworkKey(currentIdentity,
			networkKey,
			networkContextState);

		return groupPaths(listedPaths, networks);
	}, [
		currentIdentity,
		isEthereumPath,
		networkKey,
		networkContextState,
		networks
	]);
	const { isSeedRefValid } = useSeedRef(currentIdentity.encryptedSeed);
	const { unlockWithoutPassword } = useUnlockSeed(isSeedRefValid);

	if (isEthereumNetwork(networkParams)) {
		return (
			<PathDetailsView
				accountsStore={accountsStore}
				navigation={navigation}
				networkKey={networkKey}
				path={networkKey}
			/>
		);
	}

	const { navigate } = navigation;
	const rootPath = `//${networkParams.pathId}`;

	const onTapDeriveButton = (): Promise<void> =>
		unlockWithoutPassword({
			name: 'PathDerivation',
			params: { parentPath: isUnknownNetworkPath ? '' : rootPath }
		});

	const renderSinglePath = (pathsGroup: PathGroup): React.ReactElement => {
		const path = pathsGroup.paths[0];

		return (
			<PathCard
				identity={currentIdentity}
				key={path}
				onPress={(): void => {console.log('PathDetails', path); navigate('PathDetails', { path })}}
				path={path}
				testID={testIDs.PathsList.pathCard + path}
			/>
		);
	};

	return (
		<SafeAreaViewContainer>
			<ScrollView testID={testIDs.PathsList.screen}>
				<LeftScreenHeading
					hasSubtitleIcon={true}
					networkKey={networkKey}
					title={networkParams.title}
				/>
				{(pathsGroups as PathGroup[]).map(pathsGroup =>
					pathsGroup.paths.length === 1 ? (
						renderSinglePath(pathsGroup)
					) : (
						<PathGroupCard
							accountsStore={accountsStore}
							currentIdentity={currentIdentity}
							key={pathsGroup.title}
							networkParams={networkParams}
							pathGroup={pathsGroup}
						/>
					))}
				<Separator style={{ backgroundColor: 'transparent' }} />
			</ScrollView>
			<QRScannerAndDerivationTab
				derivationTestID={testIDs.PathsList.deriveButton}
				onPress={onTapDeriveButton}
				title="Derive New Account"
			/>
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(PathsList);
