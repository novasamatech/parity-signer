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
			const seedListUnsorted = await getAllSeedNames();
			const seedList = seedListUnsorted.sort();
			setRootSeedList(seedList);
			console.log(seedList);
			if (seedList) setRootSeed(seedList[0]);
		}
		populatePathsList(networkKey);
	}, [networkKey]);

	useEffect(() => {
		const fetchPaths = async function (networkKeyRef: string, rootSeedRef: string): Promise<void> {
			const fetched = await getIdentitiesForSeed(rootSeedRef, networkKeyRef);
			const sorted = fetched.sort((a, b) => { return a.path>b.path});
			setPaths(fetched);
		}
		if(rootSeed) fetchPaths(networkKey, rootSeed);
	}, [networkKey, rootSeed])

	const renderSeed = ({ item }: { item: string }): ReactElement => {
		const active = item === rootSeed;
		return (
			<TouchableItem
				onPress={() => setRootSeed(item)}
				style={active ? styles.seedActive : styles.seed}
			>
				<Text style={active ? styles.seedLabelActive : styles.seedLabelInactive}>{item}</Text>
			</TouchableItem>
		);
	};
	
	const renderIdentity = ({ item }): ReactElement => {
		return (
			<View style={{...styles.content, justifyContent: 'space-between'}}>
				<TouchableItem
					onPress={onTapIdentity}
					style={styles.card}
				>
					<View style={{flexDirection: 'row'}}>
						<Identicon value={'0'} size={40} />
						<View style={{ paddingHorizontal: 10 }}>
							<Text style={styles.textLabel}>{item.name}</Text>
							<View style={{flexDirection: 'row'}}>
								<Text style={{...styles.derivationText, fontWeight: 'bold'}}>{rootSeed}</Text>
								<Text style={styles.derivationText}>{item.path}</Text>
								{item.hasPassword === 'true' ? (
									<AntIcon name="lock" style={styles.icon} />
								) : (
									<View />
								)}
							</View>
							<Text
								style={styles.authorAddressText}
								numberOfLines={1}
								adjustFontSizeToFit
							>
								{'123456789'}
							</Text>

						</View>
					</View>
				</TouchableItem>
				<TouchableItem
					onPress={onTapDeriveButton}
					style={{...styles.card, alignItems: 'center'}}
				>
					<Text style={styles.icon}>+</Text>
					<Text style={styles.textLabel}>Derive</Text>
				</TouchableItem>
			</View>
		);
	};

	const onTapDeriveButton = (): Promise<void> => {};

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
						style={{...styles.card, alignItems: 'center', height: 54}}
					>
						<Text style={styles.icon}>+</Text>
						<Text style={styles.textLabel}>New</Text>
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
		paddingLeft: 16,
		paddingRight: 16
	},
	cardActive: {
		backgroundColor: colors.background.cardActive,
		borderColor: colors.border.light,
		borderWidth: 1,
		paddingLeft: 16,
		paddingRight: 16
	},
	content: {
		alignItems: 'center',
		backgroundColor: colors.background.card,
		flexDirection: 'row',
	},
	derivationText: {
		...fontStyles.t_codeS,
		color: colors.signal.main,
		textAlign: 'left'
	},
	icon: {
		...fontStyles.i_large,
		color: colors.signal.main,
		fontWeight: 'bold'
	},
	seed: {
		backgroundColor: colors.background.card,
		borderColor: colors.border.light,
		borderWidth: 1,
		paddingLeft: 16,
		paddingRight: 16
	},
	seedActive: {
		backgroundColor: colors.background.cardActive,
		borderColor: colors.border.light,
		borderWidth: 1,
		paddingLeft: 16,
		paddingRight: 16
	},
	seedLabelActive: {
		...fontStyles.t_label,
		justifyContent: 'center',
		fontSize: 32
	},
	seedLabelInactive: {
		...fontStyles.t_label,
		color: colors.text.main,
		justifyContent: 'center',
		fontSize: 32
	},
	textLabel: {
		...fontStyles.a_text
	}
});
