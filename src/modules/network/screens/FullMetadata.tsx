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

// This screen is not functional yet

import React, {
	ReactElement /*, useContext, useEffect, useState*/
} from 'react';
import { StyleSheet } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
//import { NetworksContext } from 'stores/NetworkContext';
import { NavigationProps } from 'types/props';
//import { getSubstrateNetworkKeyByPathId } from 'utils/identitiesUtils';
//import { getMetadata } from 'utils/db';
//import { useFullMetadataHook } from 'modules/network/networksHooks';
import colors from 'styles/colors';
import fonts from 'styles/fonts';

export default function FullMetadata({}: //	navigation,
//	route
NavigationProps<'NetworkSettings'>): React.ReactElement {
	/*const networkPathId = route.params.pathId;
	const { networks } = useContext(NetworksContext);
	const [savedMetadata, setSavedMetadata] = useState<string>('');
	const networkKey = getSubstrateNetworkKeyByPathId(networkPathId, networks);
	const metadataHandle = networks.get(networkKey).metadata;
	const [metadataReady, setMetadataReady] = useState<bool>(false);

	useEffect(() => {
		const getSavedMetadata = async function (): Promise<void> {
			const newSavedMetadata = await getMetadata(metadataHandle);
			setSavedMetadata(newSavedMetadata);
			setMetadataReady(true);
		};
		getSavedMetadata();
	}, [setSavedMetadata, setMetadataReady, metadataHandle]);
	console.log(typeof savedMetadata);
	console.log(metadataReady);

	function showFullMetadata(): React.ReactNode {
		if (metadataReady) {
			return <Text>{savedMetadata}</Text>;
		} else {
			return;
		}
	}
			{showFullMetadata()} //call it below
*/
	return <SafeAreaScrollViewContainer style={styles.body} />;
}

const styles = StyleSheet.create({
	body: {
		padding: 20
	},
	bodyContent: {
		paddingBottom: 40
	},
	descSecondary: {
		color: colors.background.app,
		flex: 1,
		fontFamily: fonts.bold,
		fontSize: 14,
		paddingBottom: 20
	},
	descTitle: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 10,
		textAlign: 'center'
	}
});
