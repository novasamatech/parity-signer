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

import React, { ReactElement, useContext, useState, useEffect } from 'react';
import { BackHandler, FlatList, View } from 'react-native';
import { useFocusEffect } from '@react-navigation/native';
import { Icon } from 'react-native-elements';

import { NetworkCard } from 'components/NetworkCard';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { IdentityHeading } from 'components/ScreenHeading';
import testIDs from 'e2e/testIDs';
import { AlertStateContext } from 'stores/alertContext';
import { NavigationAccountIdentityProps } from 'types/props';
import QrScannerTab from 'components/QrScannerTab';
import { getAllNetworks } from 'utils/native';
import TouchableItem from 'components/TouchableItem';
import colors from 'styles/colors';

type Network = {
	color: string;
	key: string;
	logo: string;
	order: number;
	secondaryColor: string;
	title: string;
};

export default function NetworkSelector({
	navigation,
	route
}: NavigationAccountIdentityProps<'Main'>): React.ReactElement {
	const isNew = route.params?.isNew ?? false;
	const [networkList, setNetworkList] = useState<Array<Network>>([]);

	const { setAlert } = useContext(AlertStateContext);

	useEffect(() => {
		const fetchNetworkList = async function (): Promise<void> {
			const networkListFetch = await getAllNetworks();
			//This is where we check if the user has accepted TOC and PP
			if (Object.keys(networkListFetch).length === 0) {
				console.log('go to TOC');
				navigation.navigate('TermsAndConditions', { policyConfirmed: false });
			}
			setNetworkList(networkListFetch);
		};
		fetchNetworkList();
	}, []);

	const renderScreenHeading = (): React.ReactElement => {
		return <IdentityHeading title={'Select network'} />;
	};

	const onNetworkChosen = async (networkKey: string): Promise<void> => {
		navigation.navigate('PathsList', { networkKey });
	};

	const renderNetwork = ({item, index, separators}: {item: Network, index: number, separators: any}): ReactElement => {
		console.log(item);
		return (
			<View style={{ flexDirection: 'row'}}>
				<View style={{flex: 8}}>
				<NetworkCard
					network={item}
					onPress={(): Promise<void> => onNetworkChosen(item.key)}
				/>
				</View>
				<View style={{flex:2, alignItems: 'center', justifyContent: 'center'}}>
					<TouchableItem 
						onPress={(): Promise<void> => navigation.navigate('NetworkDetails', {networkKey: item.key})}
						style={{alignItems: 'center', justifyContent: 'center'}}
					>
						<Icon name={"settings"} type={"feather"} color={colors.text.main} />
					</TouchableItem>
				</View>
			</View>
		);
	};

	return (
		<SafeAreaViewContainer>
			{renderScreenHeading()}
			<FlatList
				bounces={false}
				data={networkList}
				renderItem={renderNetwork}
				testID={testIDs.Main.chooserScreen}
			/>
			<QrScannerTab />
		</SafeAreaViewContainer>
	);
}
