// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

// This screen should show full contents of chosen metadata

import React, { useContext, useEffect, useState } from 'react';
import { ActivityIndicator, StyleSheet, Text, View } from 'react-native';
import { TypeRegistry } from '@polkadot/types';
import { Metadata } from '@polkadot/metadata';
import { expandMetadata } from '@polkadot/metadata/decorate';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NetworksContext } from 'stores/NetworkContext';
import { NavigationProps } from 'types/props';
import { getSubstrateNetworkKeyByPathId } from 'utils/identitiesUtils';
import { getMetadata } from 'utils/db';
//import { useFullMetadataHook } from 'modules/network/networksHooks';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import fontStyles from 'styles/fontStyles';

export default function FullMetadata({
	route
}: NavigationProps<'FullMetadataViewer'>): React.ReactElement {
	const networkPathId = route.params.pathId;
	const { networks } = useContext(NetworksContext);
	const [savedMetadata, setSavedMetadata] = useState<string>('');
	const networkKey = getSubstrateNetworkKeyByPathId(networkPathId, networks);
	const metadataHandle = networks.get(networkKey)!.metadata;
	const [metadataReady, setMetadataReady] = useState<boolean>(false);

	useEffect(() => {
		const getSavedMetadata = async function (): Promise<void> {
			const newSavedMetadata = await getMetadata(metadataHandle);
			const registry = new TypeRegistry();
			const metadata = new Metadata(registry, newSavedMetadata);
			const decorated = expandMetadata(registry, metadata);
			setSavedMetadata(JSON.stringify(decorated));
			setMetadataReady(true);
		};
		getSavedMetadata();
	}, [setSavedMetadata, setMetadataReady, metadataHandle]);

	function showFullMetadata(): React.ReactNode {
		if (metadataReady) {
			return <Text style={styles.titleText}>{savedMetadata}</Text>;
		} else {
			return (
				<View>
					<ActivityIndicator
						animating={true}
						color="red"
						size="large"
						style={styles.indicator}
					/>
					<Text style={fontStyles.quote}>"Reading metadata"</Text>
				</View>
			);
		}
	}

	return (
		<SafeAreaScrollViewContainer style={styles.body}>
			{showFullMetadata()}
		</SafeAreaScrollViewContainer>
	);
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
	},
	indicator: {
		margin: 15
	},
	titleText: {
		...fontStyles.t_codeS,
		color: colors.text.main,
		paddingHorizontal: 16
	}
});
