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

import React, { useContext, useState, useEffect } from 'react';
import { FlatList, ScrollView, StyleSheet, Text, View } from 'react-native';

import { PathDetailsView } from './PathDetails';

import { NetworksContext } from 'stores/NetworkContext';
import { PathGroup } from 'types/identityTypes';
import PathGroupCard from 'components/PathGroupCard';
import { useUnlockSeed } from 'utils/navigationHelpers';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { NavigationAccountIdentityProps } from 'types/props';
import {
	getPathsWithSubstrateNetworkKey,
	groupPaths
} from 'utils/identitiesUtils';
import QRScannerAndDerivationTab from 'components/QRScannerAndDerivationTab';
import PathCard from 'components/PathCard';
import Separator from 'components/Separator';
import { LeftScreenHeading } from 'components/ScreenHeading';
import OnBoardingView from 'components/OnBoarding';
import { getAllSeedNames, getNetwork, getIdentitiesForSeed } from 'utils/native';
import TouchableItem from 'components/TouchableItem';
import fontStyles from 'styles/fontStyles';
import colors from 'styles/colors';
import Identicon from '@polkadot/reactnative-identicon';
import AntIcon from 'react-native-vector-icons/AntDesign';


export default function PathsList({
	navigation,
	route
}: NavigationAccountIdentityProps<'PathsList'>): React.ReactElement {
	const networkKey = route.params.networkKey;
	const [rootSeed, setRootSeed] = useState('');
	const [rootSeedList, setRootSeedList] = useState([]);
	const [network, setNetwork] = useState();
	const [paths, setPaths] = useState([]);

	const { navigate } = navigation;
	
	useEffect(() => {
		const populatePathsList = async function (networkKeyRef: string): Promise<void> {
			console.log(networkKeyRef);
			const networkInfo = await getNetwork(networkKeyRef);
			console.log(networkInfo);
			setNetwork(networkInfo);
			const seedList = await getAllSeedNames();
			setRootSeedList(seedList);
			console.log(seedList);
			if (seedList) setRootSeed(seedList[0]);
		}
		populatePathsList(networkKey);
	}, [networkKey]);

	useEffect(() => {
		const fetchPaths = async function (networkKeyRef: string, rootSeedRef: string): Promise<void> {
			const fetched = await getIdentitiesForSeed(rootSeedRef, networkKeyRef);
			setPaths(fetched);
		}
		if(rootSeed) fetchPaths(networkKey, rootSeed);
	}, [networkKey, rootSeed])

	const renderSeed = ({ item }: { item: string }): ReactElement => {
		return (
			<TouchableItem
				onPress={() => setRootSeed(item)}
				style={styles.card}
			>
				<Text style={styles.seedLabel}>{item}</Text>
			</TouchableItem>
		);
	};
	
	const renderIdentity = ({ item }): ReactElement => {
		return (
			<TouchableItem
				onPress={onTapIdentity}
				style={styles.card}
			>
				<View style={styles.content}>
					<Identicon value={'0'} size={40} />
					<View style={{ paddingHorizontal: 10 }}>
						<Text style={styles.textLabel}>{item.name}</Text>
						<View style={{flexDirection: 'row'}}>
							<AntIcon 
								name="user"
								size={fontStyles.i_small.fontSize}
								color={colors.signal.main}
							/>
							<Text style={styles.textLabel}>{item.path}</Text>
							{item.hasPassword === 'true' ? (
								<AntIcon name="lock" style={styles.iconLock} />
							) : (
								<View />
							)}
						</View>
					</View>
				</View>
			</TouchableItem>
		);
	};

	const onTapDeriveButton = (): Promise<void> =>
		unlockWithoutPassword({
			name: 'PathDerivation',
			params: { parentPath: rootPath }
		});

	const onTapNewSeedButton = (): Promise<void> => {
		navigation.navigate('RootSeedNew', { false });
	};

	const onTapIdentity = (): Promise<void> => {};

	if (rootSeed) {
		return (
			<SafeAreaViewContainer>
				<LeftScreenHeading
					title={network ? network.title : ''}
					hasSubtitleIcon={true}
					networkKey={networkKey}
				/>
				<Separator style={{ backgroundColor: 'transparent' }} />
				<View style={{flexDirection: 'row'}}>
					<FlatList horizontal={true} 
						data={rootSeedList}
						renderItem={renderSeed}
						keyExtractor={item => item}
					/>
					<TouchableItem
						onPress={onTapNewSeedButton}
						style={{...styles.card, alignItems: 'center', height: 72}}
					>
						<Text style={styles.icon}>+</Text>
						<Text style={styles.textLabel}>New</Text>
						<Text style={styles.textLabel}>seed</Text>
					</TouchableItem>
				</View>
				<Separator style={{ backgroundColor: 'transparent' }} />
				<FlatList
					data={paths}
					renderItem={renderIdentity}
					keyExtractor={item => item.path}
				/>
				<QRScannerAndDerivationTab
					derivationTestID={testIDs.PathsList.deriveButton}
					title="Derive"
					onPress={onTapDeriveButton}
				/>
			</SafeAreaViewContainer>
		);
	} else {
		return <OnBoardingView />
	}
}

const styles = StyleSheet.create({
	card: {
		borderColor: colors.background.os,
		borderWidth: 1,
		paddingLeft: 16,
		paddingRight: 16
	},
	content: {
		alignItems: 'center',
		backgroundColor: colors.background.card,
		flexDirection: 'row',
		paddingLeft: 8,
		paddingVertical: 8
	},
	icon: {
		...fontStyles.i_large,
		color: colors.signal.main,
		fontWeight: 'bold'
	},
	seedLabel: {
		...fontStyles.a_text,
		justifyContent: 'center',
		fontSize: 32
	},
	textLabel: {
		...fontStyles.a_text
	}
});
