// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.	If not, see <http://www.gnu.org/licenses/>.

import { StackNavigationProp } from '@react-navigation/stack';
import { useNavigation } from '@react-navigation/native';
import React, { useContext } from 'react';
import { ScrollView, StyleSheet, View } from 'react-native';
import { showMessage } from 'react-native-flash-message';

import { colors, fontStyles } from 'styles/index';
import OnBoardingView from 'modules/main/components/OnBoarding';
import ButtonIcon from 'components/ButtonIcon';
import Separator from 'components/Separator';
import NavigationTab from 'components/NavigationTab';
import { AccountsContext } from 'stores/AccountsContext';
import { Identity } from 'types/identityTypes';
import { NavigationProps } from 'types/props';
import { RootStackParamList } from 'types/routes';
import testIDs from 'e2e/testIDs';
import { getIdentitySeed, getIdentityName } from 'utils/identitiesUtils';

function ButtonWithArrow(props: {
	onPress: () => void;
	testID?: string;
	title: string;
}): React.ReactElement {
	return <ButtonIcon {...props} {...i_arrowOptions} />;
}

function Settings({}: NavigationProps<'Settings'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();
	const { currentIdentity, identities } = accountsStore.state;
	if (identities.length === 0) return <OnBoardingView />;
	if (!currentIdentity) return <View />;

	const renderIdentity = (identity: Identity): React.ReactElement => {
		const title = getIdentityName(identity, identities);
		const showRecoveryPhrase = async (
			targetIdentity: Identity
		): Promise<void> => {
			const seedPhrase = await getIdentitySeed(targetIdentity);
			navigation.navigate('ShowRecoveryPhrase', { seedPhrase });
		};

		return (
			<View key={identity.encryptedSeed}>
				<ButtonIcon
					title={title}
					iconType="antdesign"
					iconName="user"
					iconSize={24}
					style={styles.indentedButton}
					textStyle={fontStyles.h2}
				/>
				{currentIdentity.encryptedSeed !== identity.encryptedSeed ? (
					<ButtonWithArrow
						title="Select this wallet"
						onPress={(): void => {
							accountsStore.selectIdentity(identity);
							showMessage('Wallet switched.');
						}}
					/>
				) : null}
				<ButtonWithArrow
					title="Rename"
					onPress={(): void =>
						navigation.navigate('RenameWallet', {
							accountsStore,
							identity,
							navigation
						})
					}
				/>
				<ButtonWithArrow
					title="Delete"
					onPress={(): void =>
						navigation.navigate('DeleteWallet', {
							accountsStore,
							identity,
							navigation
						})
					}
				/>
				<ButtonWithArrow
					title="Show Key Phrase"
					onPress={(): Promise<void> => showRecoveryPhrase(identity)}
				/>
				<Separator style={{ marginBottom: 0 }} />
			</View>
		);
	};

	return (
		<>
			<View style={styles.card}>
				<ScrollView bounces={false}>
					{identities.map(renderIdentity)}
				</ScrollView>

				<ButtonIcon
					title="Add wallet"
					testID={testIDs.IdentitiesSwitch.addIdentityButton}
					onPress={(): void => navigation.navigate('CreateWallet')}
					iconName="plus"
					iconType="antdesign"
					iconSize={24}
					textStyle={fontStyles.t_big}
					style={styles.indentedButton}
				/>
			</View>
			<View style={styles.tab}>
				<NavigationTab />
			</View>
		</>
	);
}

const styles = StyleSheet.create({
	card: {
		backgroundColor: colors.background.app,
		borderRadius: 4,
		paddingBottom: 16,
		paddingTop: 16
	},
	container: {
		justifyContent: 'center',
		paddingHorizontal: 16
	},
	i_arrowStyle: {
		paddingLeft: 64,
		paddingTop: 0
	},
	indentedButton: {
		paddingLeft: 32
	},
	tab: {
		flex: 1,
		justifyContent: 'flex-end'
	}
});

const i_arrowOptions = {
	iconColor: colors.text.accent,
	iconName: 'arrowright',
	iconSize: 18,
	iconType: 'antdesign',
	style: styles.i_arrowStyle,
	textStyle: { ...fontStyles.a_text, color: colors.text.accent }
};

export default Settings;
