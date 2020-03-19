// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

import { StackNavigationProp } from '@react-navigation/stack';
import React, { useState } from 'react';
import { ScrollView, StyleSheet, View } from 'react-native';
import { useNavigation } from '@react-navigation/native';

import ButtonIcon from './ButtonIcon';
import Separator from './Separator';
import TransparentBackground from './TransparentBackground';

import { RootStackParamList } from 'types/routes';
import AccountsStore from 'stores/AccountsStore';
import testIDs from 'e2e/testIDs';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { withAccountStore } from 'utils/HOC';
import { getIdentityName } from 'utils/identitiesUtils';
import {
	navigateToLegacyAccountList,
	resetNavigationTo,
	resetNavigationWithNetworkChooser,
	unlockSeedPhrase
} from 'utils/navigationHelpers';
import { Identity } from 'types/identityTypes';

function IdentitiesSwitch({
	accounts
}: {
	accounts: AccountsStore;
}): React.ReactElement {
	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();
	const [visible, setVisible] = useState(false);
	const { currentIdentity, identities } = accounts.state;
	// useEffect(() => {
	// 	const firstLogin: boolean = identities.length === 0;
	// 	if (currentIdentity === null && !firstLogin) {
	// 		setVisible(true);
	// 	}
	// }, [currentIdentity, identities]);

	const closeModalAndNavigate = <RouteName extends keyof RootStackParamList>(
		screenName: RouteName,
		params?: RootStackParamList[RouteName]
	): void => {
		setVisible(false);
		// @ts-ignore
		navigation.navigate(screenName, params);
	};

	const onIdentitySelectedAndNavigate = async <
		RouteName extends keyof RootStackParamList
	>(
		identity: Identity,
		screenName: RouteName,
		params?: RootStackParamList[RouteName]
	): Promise<void> => {
		await accounts.selectIdentity(identity);
		setVisible(false);
		if (screenName === 'AccountNetworkChooser') {
			resetNavigationTo(navigation, screenName, params);
		} else if (screenName === 'IdentityBackup') {
			const seedPhrase = await unlockSeedPhrase(navigation);
			resetNavigationWithNetworkChooser(navigation, screenName, {
				isNew: false,
				seedPhrase
			});
		} else {
			resetNavigationWithNetworkChooser(navigation, screenName, params);
		}
	};

	const onLegacyListClicked = async (): Promise<void> => {
		await accounts.resetCurrentIdentity();
		setVisible(false);
		navigateToLegacyAccountList(navigation);
	};

	const renderIdentityOptions = (identity: Identity): React.ReactElement => {
		return (
			<>
				<ButtonIcon
					title="Manage Identity"
					onPress={(): Promise<void> =>
						onIdentitySelectedAndNavigate(identity, 'IdentityManagement')
					}
					iconBgStyle={styles.i_arrowBg}
					iconType="antdesign"
					iconName="arrowright"
					iconSize={18}
					testID={testIDs.IdentitiesSwitch.manageIdentityButton}
					textStyle={fontStyles.t_regular}
					style={styles.i_arrowStyle}
				/>
				<ButtonIcon
					title="Show Recovery Phrase"
					onPress={(): Promise<void> =>
						onIdentitySelectedAndNavigate(identity, 'IdentityBackup')
					}
					iconBgStyle={styles.i_arrowBg}
					iconType="antdesign"
					iconName="arrowright"
					iconSize={18}
					textStyle={fontStyles.t_regular}
					style={styles.i_arrowStyle}
				/>
			</>
		);
	};

	const renderCurrentIdentityCard = (): React.ReactNode => {
		if (!currentIdentity) return;

		const currentIdentityTitle = getIdentityName(currentIdentity, identities);

		return (
			<>
				<ButtonIcon
					title={currentIdentityTitle}
					onPress={(): Promise<void> =>
						onIdentitySelectedAndNavigate(
							currentIdentity,
							'AccountNetworkChooser'
						)
					}
					iconType="antdesign"
					iconName="user"
					iconSize={40}
					style={{ paddingLeft: 16 }}
					textStyle={fontStyles.h1}
				/>
				{renderIdentityOptions(currentIdentity)}
				<Separator style={{ marginBottom: 0 }} />
			</>
		);
	};

	const renderSettings = (): React.ReactElement => {
		return (
			<>
				<ButtonIcon
					title="About"
					onPress={(): void => closeModalAndNavigate('About')}
					iconType="antdesign"
					iconName="info"
					iconSize={24}
					textStyle={fontStyles.t_big}
					style={styles.indentedButton}
				/>
				<ButtonIcon
					title="Terms and Conditions"
					onPress={(): void => closeModalAndNavigate('TermsAndConditions')}
					iconBgStyle={styles.i_arrowBg}
					iconType="antdesign"
					iconName="arrowright"
					iconSize={18}
					textStyle={fontStyles.t_regular}
					style={styles.i_arrowStyle}
				/>
				<ButtonIcon
					title="Privacy Policy"
					onPress={(): void => closeModalAndNavigate('PrivacyPolicy')}
					iconBgStyle={styles.i_arrowBg}
					iconType="antdesign"
					iconName="arrowright"
					iconSize={18}
					textStyle={fontStyles.t_regular}
					style={styles.i_arrowStyle}
				/>
			</>
		);
	};

	const renderNonSelectedIdentity = (
		identity: Identity
	): React.ReactElement => {
		const title = getIdentityName(identity, identities);

		return (
			<ButtonIcon
				dropdown={false}
				renderDropdownElement={(): React.ReactElement =>
					renderIdentityOptions(identity)
				}
				title={title}
				onPress={(): Promise<void> =>
					onIdentitySelectedAndNavigate(identity, 'AccountNetworkChooser')
				}
				key={identity.encryptedSeed}
				iconType="antdesign"
				iconName="user"
				iconSize={24}
				style={styles.indentedButton}
				textStyle={fontStyles.h2}
			/>
		);
	};

	const renderIdentities = (): React.ReactNode => {
		// if no identity or the only one we have is the selected one

		if (!identities.length || (identities.length === 1 && currentIdentity))
			return;

		const identitiesToShow = currentIdentity
			? identities.filter(
					identity => identity.encryptedSeed !== currentIdentity.encryptedSeed
			  )
			: identities;

		return (
			<>
				<ScrollView
					bounces={false}
					style={{
						maxHeight: 180,
						paddingVertical: identities.length > 5 ? 8 : 0
					}}
				>
					{identitiesToShow.map(renderNonSelectedIdentity)}
				</ScrollView>
				{identities.length > 5 && (
					<Separator
						shadow={true}
						style={{ backgroundColor: 'transparent', marginTop: 0 }}
						shadowStyle={{ opacity: 0.9 }}
					/>
				)}
			</>
		);
	};

	return (
		<View>
			<ButtonIcon
				onPress={(): void => setVisible(!visible)}
				iconName="user"
				iconType="antdesign"
				iconBgStyle={{ backgroundColor: 'transparent' }}
				testID={testIDs.IdentitiesSwitch.toggleButton}
			/>

			<TransparentBackground
				testID={testIDs.IdentitiesSwitch.modal}
				visible={visible}
				setVisible={setVisible}
				style={styles.container}
				animationType="none"
			>
				<View style={styles.card}>
					{renderCurrentIdentityCard()}
					{renderIdentities()}
					{accounts.getAccounts().size > 0 && (
						<>
							<ButtonIcon
								title="Legacy Accounts"
								onPress={onLegacyListClicked}
								iconName="solution1"
								iconType="antdesign"
								iconSize={24}
								textStyle={fontStyles.t_big}
								style={styles.indentedButton}
							/>
							<Separator />
						</>
					)}

					<ButtonIcon
						title="Add Identity"
						testID={testIDs.IdentitiesSwitch.addIdentityButton}
						onPress={(): void => closeModalAndNavigate('IdentityNew')}
						iconName="plus"
						iconType="antdesign"
						iconSize={24}
						textStyle={fontStyles.t_big}
						style={styles.indentedButton}
					/>

					<Separator />
					{__DEV__ && (
						<View>
							<ButtonIcon
								title="Add legacy account"
								onPress={(): void => closeModalAndNavigate('AccountNew')}
								iconName="plus"
								iconType="antdesign"
								iconSize={24}
								textStyle={fontStyles.t_big}
								style={styles.indentedButton}
							/>
							<Separator />
						</View>
					)}

					{renderSettings()}
				</View>
			</TransparentBackground>
		</View>
	);
}

const styles = StyleSheet.create({
	card: {
		backgroundColor: colors.bg,
		borderRadius: 5,
		paddingBottom: 16,
		paddingTop: 8
	},
	container: {
		justifyContent: 'center',
		marginTop: -24,
		paddingLeft: 16,
		paddingRight: 16
	},
	i_arrowBg: {
		backgroundColor: 'rgba(0,0,0,0)',
		marginRight: -3
	},
	i_arrowStyle: {
		opacity: 0.7,
		paddingBottom: 6,
		paddingLeft: 64,
		paddingTop: 0
	},
	indentedButton: {
		paddingLeft: 32
	}
});

export default withAccountStore(IdentitiesSwitch);
