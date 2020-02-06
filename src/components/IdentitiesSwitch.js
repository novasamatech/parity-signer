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

'use strict';

import React, { useState } from 'react';
import { FlatList, View } from 'react-native';
import { withNavigation, ScrollView } from 'react-navigation';

import ButtonIcon from './ButtonIcon';
import colors from '../colors';
import fontStyles from '../fontStyles';
import Separator from './Separator';
import { withAccountStore } from '../util/HOC';
import { getIdentityName } from '../util/identitiesUtils';
import testIDs from '../../e2e/testIDs';
import {
	navigateToLegacyAccountList,
	resetNavigationTo,
	resetNavigationWithNetworkChooser
} from '../util/navigationHelpers';
import TransparentBackground from './TransparentBackground';

function IdentitiesSwitch({ navigation, accounts }) {
	const defaultVisible = navigation.getParam('isSwitchOpen', false);
	const [visible, setVisible] = useState(defaultVisible);
	const { currentIdentity, identities } = accounts.state;

	const closeModalAndNavigate = (screenName, params) => {
		setVisible(false);
		navigation.navigate(screenName, params);
	};

	const onIdentitySelectedAndNavigate = async (
		identity,
		screenName,
		params
	) => {
		await accounts.selectIdentity(identity);
		setVisible(false);
		if (screenName === 'AccountNetworkChooser') {
			resetNavigationTo(navigation, screenName, params);
		} else {
			resetNavigationWithNetworkChooser(navigation, screenName, params);
		}
	};

	const onLegacyListClicked = async () => {
		await accounts.resetCurrentIdentity();
		setVisible(false);
		navigateToLegacyAccountList(navigation);
	};

	const renderIdentityOptions = identity => {
		return (
			<>
				<ButtonIcon
					title="Manage Identity"
					onPress={() =>
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
					onPress={() =>
						onIdentitySelectedAndNavigate(identity, 'IdentityBackup', {
							isNew: false
						})
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

	const renderCurrentIdentityCard = () => {
		if (!currentIdentity) return;

		const currentIdentityTitle = getIdentityName(currentIdentity, identities);

		return (
			<>
				<ButtonIcon
					title={currentIdentityTitle}
					onPress={() =>
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

	const renderSettings = () => {
		return (
			<>
				<ButtonIcon
					title="About"
					onPress={() => closeModalAndNavigate('About')}
					iconType="antdesign"
					iconName="info"
					iconSize={24}
					textStyle={fontStyles.t_big}
					style={styles.indentedButton}
				/>
				<ButtonIcon
					title="Terms and Conditions"
					onPress={() =>
						closeModalAndNavigate('TermsAndConditions', {
							disableButtons: true
						})
					}
					iconBgStyle={styles.i_arrowBg}
					iconType="antdesign"
					iconName="arrowright"
					iconSize={18}
					textStyle={fontStyles.t_regular}
					style={styles.i_arrowStyle}
				/>
				<ButtonIcon
					title="Privacy Policy"
					onPress={() => closeModalAndNavigate('PrivacyPolicy')}
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

	const renderNonSelectedIdentity = ({ item }) => {
		const identity = item;
		const title = getIdentityName(identity, identities);

		return (
			<ButtonIcon
				dropdown={false}
				renderDropdownElement={() => renderIdentityOptions(identity)}
				title={title}
				onPress={() =>
					onIdentitySelectedAndNavigate(identity, 'AccountNetworkChooser')
				}
				iconType="antdesign"
				iconName="user"
				iconSize={24}
				style={styles.indentedButton}
				textStyle={fontStyles.h2}
			/>
		);
	};

	const renderIdentities = () => {
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
				<ScrollView style={{ maxHeight: 180 }}>
					<FlatList
						data={identitiesToShow}
						renderItem={renderNonSelectedIdentity}
						keyExtractor={item => item.encryptedSeed}
						style={{ paddingVertical: identities.length > 5 ? 8 : 0 }}
					/>
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
				onPress={() => setVisible(!visible)}
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
						onPress={() => closeModalAndNavigate('IdentityNew')}
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
								onPress={() => closeModalAndNavigate('AccountNew')}
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

const styles = {
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
};

export default withAccountStore(withNavigation(IdentitiesSwitch));
