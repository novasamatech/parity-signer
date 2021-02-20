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

import NetInfo from '@react-native-community/netinfo';
import { useNavigation } from '@react-navigation/native';
import AccountCard from 'components/AccountCard';
import QrScannerTab from 'components/QrScannerTab';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import React, { useContext, useEffect, useState } from 'react';
import { ScrollView, StyleSheet, Text,TouchableWithoutFeedback, View } from 'react-native';
import Icon from 'react-native-vector-icons/Feather';
import colors from 'styles/colors';
import { LegacyAccount } from 'types/identityTypes';

import { AccountsContext } from '../context';

function LegacyAccountList(): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const [isConnected, setIsConnected] = useState(false);
	const navigation = useNavigation();

	useEffect(() =>
		NetInfo.addEventListener(state => {
			setIsConnected(state.isConnected);
		}),
	[]);

	const onAccountSelected = async (key: string): Promise<void> => {
		await accountsStore.select(key);
		navigation.navigate('AccountDetails');
	};

	const { accounts } = accountsStore.state;

	const renderAccountCard = ({ address, name, networkKey }: LegacyAccount): React.ReactElement => (
		<AccountCard
			address={address}
			key={address}
			networkKey={networkKey}
			onPress={(): Promise<void> => onAccountSelected(address)}
			style={{ paddingBottom: 0 }}
			title={name}
		/>
	);

	return (
		<SafeAreaViewContainer>
			{isConnected && (
				<TouchableWithoutFeedback
					onPress={(): void => navigation.navigate('Security')}
				>
					<View style={styles.insecureDeviceBanner}>
						<Icon
							color={colors.text.white}
							name="shield-off"
							size={22}
						/>
						<Text style={styles.warningText}>Insecure device</Text>
					</View>
				</TouchableWithoutFeedback>
			)}
			<ScrollView
				style={styles.content}
				testID={testIDs.AccountListScreen.accountList}
			>
				{accounts.map(renderAccountCard)}
			</ScrollView>
			<QrScannerTab />
		</SafeAreaViewContainer>
	);
}

export default LegacyAccountList;

const styles = StyleSheet.create({
	content: {
		flex: 1,
		paddingBottom: 40
	},
	insecureDeviceBanner: {
		alignItems: 'center',
		backgroundColor: colors.signal.error,
		display: 'flex',
		flexDirection: 'row',
		justifyContent: 'center',
		padding: 5
	},
	warningText: {
		color: colors.text.white,
		marginLeft: 5
	}
});
