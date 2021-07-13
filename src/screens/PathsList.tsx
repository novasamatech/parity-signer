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

import React, { useState, useEffect } from 'react';
import { FlatList, StyleSheet, Text, View } from 'react-native';
import Identicon from '@polkadot/reactnative-identicon';
import AntIcon from 'react-native-vector-icons/AntDesign';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { NavigationAccountIdentityProps } from 'types/props';
import QRScannerAndDerivationTab from 'components/QRScannerAndDerivationTab';
import Separator from 'components/Separator';
import OnBoardingView from 'components/OnBoarding';
import {
	getAllSeedNames,
	getNetwork,
	getIdentitiesForSeed,
	suggestNPlusOne,
	deleteIdentity
} from 'utils/native';
import TouchableItem from 'components/TouchableItem';
import fontStyles from 'styles/fontStyles';
import colors from 'styles/colors';
import { CardSeparator } from 'components/CardSeparator';
import QrView from 'components/QrView';
import Button from 'components/Button';
import { NetworkCard } from 'components/NetworkCard';

export default function PathsList({
	navigation,
	route
}: NavigationAccountIdentityProps<'PathsList'>): React.ReactElement {
	const networkKey = route.params.networkKey;
	const [rootSeed, setRootSeed] = useState('');
	const [rootSeedList, setRootSeedList] = useState([]);
	const [network, setNetwork] = useState();
	const [paths, setPaths] = useState([]);
	const [activeAddress, setActiveAddress] = useState();
	const [showQR, setShowQR] = useState(false);

	const { navigate } = navigation;

	useEffect(() => {
		const populatePathsList = async function (
			networkKeyRef: string
		): Promise<void> {
			console.log(networkKeyRef);
			const networkInfo = await getNetwork(networkKeyRef);
			console.log(networkInfo);
			setNetwork(networkInfo);
			const seedListUnsorted = await getAllSeedNames();
			const seedList = seedListUnsorted.sort();
			setRootSeedList(seedList);
			console.log(seedList);
			if (seedList) setRootSeed(seedList[0]);
		};
		populatePathsList(networkKey);
	}, [networkKey]);

	useEffect(() => {
		const fetchPaths = async function (
			networkKeyRef: string,
			rootSeedRef: string
		): Promise<void> {
			const fetched = await getIdentitiesForSeed(rootSeedRef, networkKeyRef);
			const sorted = fetched.sort((a, b) => {
				return a.path > b.path;
			});
			setPaths(fetched);
		};
		if (rootSeed) fetchPaths(networkKey, rootSeed);
	}, [networkKey, rootSeed, navigation]);

	const renderSeed = ({ item }: { item: string }): ReactElement => {
		const active = item === rootSeed;
		return (
			<TouchableItem
				onPress={() => setRootSeed(item)}
				style={active ? styles.seedActive : styles.seed}
			>
				<Text
					style={active ? styles.seedLabelActive : styles.seedLabelInactive}
				>
					{item}
				</Text>
			</TouchableItem>
		);
	};

	const renderIdentity = ({ item }): ReactElement => {
		const active = item === activeAddress;
		return (
			<View>
				<View style={active ? styles.contentActive : styles.content}>
					<TouchableItem
						onPress={() =>
							activeAddress === item
								? setActiveAddress('')
								: setActiveAddress(item)
						}
						style={styles.card}
					>
						<View style={{ flexDirection: 'row' }}>
							<Identicon
								value={item.ss58}
								size={40}
								style={{ paddingTop: 10 }}
							/>
							<View style={{ paddingHorizontal: 10 }}>
								<Text style={styles.textLabel}>{item.name}</Text>
								<View style={{ flexDirection: 'row' }}>
									<Text
										style={{ ...styles.derivationText, fontWeight: 'bold' }}
									>
										{rootSeed}
									</Text>
									<Text style={styles.derivationText}>{item.path}</Text>
									{item.hasPassword === 'true' ? (
										<AntIcon name="lock" style={styles.derivationText} />
									) : (
										<View />
									)}
								</View>
								<Text
									style={styles.authorAddressText}
									numberOfLines={1}
									adjustFontSizeToFit
								>
									{item.ss58}
								</Text>
							</View>
						</View>
					</TouchableItem>
				</View>
				{active ? (
					<View style={styles.contentActive}>
						<TouchableItem
							onPress={onTapDeleteButton}
							style={{ ...styles.card, alignItems: 'center' }}
						>
							<Text style={styles.icon}>del</Text>
							<Text style={styles.textLabel}>Delete</Text>
						</TouchableItem>
						<TouchableItem
							onPress={() => setShowQR(true)}
							style={{ ...styles.card, alignItems: 'center' }}
						>
							<Text style={styles.icon}>QR</Text>
							<Text style={styles.textLabel}>Export</Text>
						</TouchableItem>
						<TouchableItem
							onPress={onTapIncrementButton}
							style={{ ...styles.card, alignItems: 'center' }}
						>
							<Text style={styles.icon}>/+1</Text>
							<Text style={styles.textLabel}>Increment</Text>
						</TouchableItem>
						<TouchableItem
							onPress={onTapDeriveButton}
							style={{ ...styles.card, alignItems: 'center' }}
						>
							<Text style={styles.icon}>/name</Text>
							<Text style={styles.textLabel}>Derive</Text>
						</TouchableItem>
					</View>
				) : (
					<View />
				)}
			</View>
		);
	};

	const onTapDeleteButton = (): Promise<void> => {
		deleteIdentity(activeAddress.publicKey, networkKey);
		setPaths(paths.filter(item => item !== activeAddress));
		setActiveAddress();
	};
	const onTapIncrementButton = async function (): Promise<void> {
		const suggestion = await suggestNPlusOne(
			activeAddress.path,
			rootSeed,
			networkKey
		);
		navigation.navigate('PathDerivation', {
			networkKey: networkKey,
			path: suggestion,
			seedName: rootSeed
		});
	};
	const onTapDeriveButton = (): Promise<void> => {
		navigation.navigate('PathDerivation', {
			networkKey: networkKey,
			path: activeAddress.path,
			seedName: rootSeed
		});
	};

	const onTapNewSeedButton = (): Promise<void> => {
		navigation.navigate('RootSeedNew', { isBackup: false });
	};

	const onTapIdentity = (): Promise<void> => {
		return;
	};

	if (showQR) {
		return (
			<SafeAreaViewContainer>
				<Text style={styles.addressName}>{activeAddress.name}</Text>
				<QrView
					data={`substrate:${activeAddress.ss58}:${networkKey}:${activeAddress.name}`}
				/>
				<Button onPress={() => setShowQR(false)} title={'DONE'} />
			</SafeAreaViewContainer>
		);
	} else if (rootSeed) {
		return (
			<SafeAreaViewContainer>
				<NetworkCard
					network={network}
					onPress={(): Promise<void> => {
						navigation.goBack();
					}}
				/>
				<View style={{ flexDirection: 'row' }}>
					<FlatList
						horizontal={true}
						data={rootSeedList}
						renderItem={renderSeed}
						keyExtractor={item => item}
					/>
					<TouchableItem
						onPress={onTapNewSeedButton}
						style={{
							alignItems: 'center',
							backgroundColor: colors.background.card,
							borderColor: colors.border.light,
							borderWidth: 2,
							paddingLeft: 8,
							paddingRight: 8
						}}
					>
						<Text style={styles.icon}>+ seed</Text>
					</TouchableItem>
				</View>
				<Separator style={{ backgroundColor: 'transparent' }} />
				<FlatList
					data={paths}
					renderItem={renderIdentity}
					keyExtractor={item => item.path}
					ItemSeparatorComponent={CardSeparator}
				/>
				<QRScannerAndDerivationTab
					derivationTestID={testIDs.PathsList.deriveButton}
					title="Derive"
					onPress={onTapDeriveButton}
				/>
			</SafeAreaViewContainer>
		);
	} else {
		return <OnBoardingView />;
	}
}

const styles = StyleSheet.create({
	addressName: {
		...fontStyles.t_codeS,
		fontSize: 20,
		textAlign: 'center'
	},
	authorAddressText: {
		...fontStyles.t_codeS,
		color: colors.text.faded,
		fontSize: 10
	},
	card: {
		paddingBottom: 8,
		paddingLeft: 16,
		paddingRight: 16,
		paddingTop: 8
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
		justifyContent: 'space-between'
	},
	contentActive: {
		alignItems: 'center',
		backgroundColor: colors.background.cardActive,
		flexDirection: 'row',
		justifyContent: 'space-between'
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
		justifyContent: 'center',
		paddingLeft: 16,
		paddingRight: 16
	},
	seedActive: {
		backgroundColor: colors.background.cardActive,
		borderColor: colors.border.light,
		justifyContent: 'center',
		paddingLeft: 16,
		paddingRight: 16
	},
	seedLabelActive: {
		...fontStyles.t_label,
		fontSize: 20,
		justifyContent: 'center'
	},
	seedLabelInactive: {
		...fontStyles.t_label,
		color: colors.text.main,
		fontSize: 20,
		justifyContent: 'center'
	},
	textLabel: {
		...fontStyles.a_text
	}
});
