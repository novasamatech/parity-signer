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

import React, { useState } from 'react';
import { FlatList, Modal, View } from 'react-native';
import { withNavigation } from 'react-navigation';

import ButtonIcon from './ButtonIcon';
import colors from '../colors';
import fontStyles from '../fontStyles';
import Separator from './Separator';
import { withAccountStore } from '../util/HOC';
import { getIdentityName } from '../util/identitiesUtils';

function IdentitiesSwitch({ navigation, accounts }) {
	const [visible, setVisible] = useState(false);
	//TODO to be removed before merge
	console.log('identities are', accounts.state.identities);
	const { currentIdentity, identities } = accounts.state;

	const onIdentitySelected = async identity => {
		setVisible(false);
		await accounts.selectIdentity(identity);
		navigation.navigate('AccountNetworkChooser');
	};

	const renderIdentityOptions = identity => {
		return (
			<>
				<ButtonIcon
					title="Manage Identity"
					onPress={async () => {
						setVisible(false);
						await accounts.selectIdentity(identity);
						navigation.navigate('IdentityManagement');
					}}
					iconBgStyle={styles.i_arrowBg}
					iconType="antdesign"
					iconName="arrowright"
					iconSize={18}
					textStyle={fontStyles.t_regular}
					style={styles.i_arrowStyle}
				/>
				<ButtonIcon
					title="Show Recovery Phrase"
					onPress={async () => {
						setVisible(false);
						await accounts.selectIdentity(identity);
						navigation.navigate('IdentityBackup', { isNew: false });
					}}
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
					onPress={() => onIdentitySelected(currentIdentity)}
					iconType="antdesign"
					iconName="user"
					iconSize={40}
					style={{ paddingLeft: 8 * 2 }}
					textStyle={fontStyles.h1}
				/>
				{renderIdentityOptions(currentIdentity)}
				<Separator />
			</>
		);
	};

	const renderSettings = () => {
		return (
			<>
				<ButtonIcon
					title="Settings"
					onPress={() => {
						setVisible(false);
						// go to Settings;
					}}
					iconType="antdesign"
					iconName="setting"
					iconSize={24}
					textStyle={fontStyles.t_big}
					style={{ paddingLeft: 8 * 4 }}
				/>
				<ButtonIcon
					title="Terms and Conditions"
					onPress={() => {
						setVisible(false);
						navigation.navigate('TermsAndConditions');
					}}
					iconBgStyle={styles.i_arrowBg}
					iconType="antdesign"
					iconName="arrowright"
					iconSize={18}
					textStyle={fontStyles.t_regular}
					style={styles.i_arrowStyle}
				/>
				<ButtonIcon
					title="Privacy Policy"
					onPress={() => {
						setVisible(false);
						navigation.navigate('PrivacyPolicy');
					}}
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

	const renderNonSelectedIdentity = ({ item, index }) => {
		const identity = item;
		const title = identity.name || `identity_${index.toString()}`;

		return (
			<ButtonIcon
				dropdown={false}
				renderDropdownElement={() => renderIdentityOptions(identity)}
				title={title}
				onPress={() => onIdentitySelected(identity)}
				iconType="antdesign"
				iconName="user"
				iconSize={24}
				style={{ paddingLeft: 8 * 4 }}
				textStyle={fontStyles.h1}
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
			<FlatList
				data={identitiesToShow}
				renderItem={renderNonSelectedIdentity}
				keyExtractor={item => item.encryptedSeed}
				style={{ flexGrow: 0 }}
			/>
		);
	};

	return (
		<View>
			<ButtonIcon
				onPress={() => setVisible(!visible)}
				iconName="user"
				iconType="antdesign"
			/>
			<Modal
				animationType="fade"
				visible={visible}
				transparent={true}
				onRequestClose={() => setVisible(false)}
			>
				<View style={styles.container}>
					<View style={styles.headerStyle}>
						<ButtonIcon
							onPress={() => {
								setVisible(false);
							}}
							iconName="close"
							iconType="antdesign"
							iconBgStyle={{ backgroundColor: colors.card_bgSolid }}
						/>
					</View>
					<View style={styles.card}>
						{renderCurrentIdentityCard()}
						{renderIdentities()}
						{accounts.getAccounts().size > 0 && (
							<ButtonIcon
								title="Legacy Accounts"
								onPress={() => {
									setVisible(false);
									navigation.navigate('LegacyAccountList');
								}}
								iconName="solution1"
								iconType="antdesign"
								iconSize={24}
								textStyle={fontStyles.t_big}
								style={{ paddingLeft: 8 * 4 }}
							/>
						)}
						<Separator />
						<ButtonIcon
							title="Add new Identity"
							onPress={() => {
								setVisible(false);
								navigation.navigate('IdentityNew');
							}}
							iconName="plus"
							iconType="antdesign"
							iconSize={24}
							textStyle={fontStyles.t_big}
							style={{ paddingLeft: 8 * 4 }}
						/>
						<Separator />
						{renderSettings()}
					</View>
				</View>
			</Modal>
		</View>
	);
}

const styles = {
	card: {
		backgroundColor: colors.bg,
		borderRadius: 5,
		marginTop: 8,
		paddingBottom: 16,
		paddingTop: 8
	},
	container: {
		backgroundColor: 'rgba(0,0,0,0.8)',
		flex: 1,
		paddingLeft: 16,
		paddingRight: 16
	},
	headerStyle: {
		alignItems: 'flex-end',
		height: 60,
		justifyContent: 'center'
	},
	i_arrowBg: {
		backgroundColor: 'rgba(0,0,0,0)',
		width: 12
	},
	i_arrowStyle: {
		marginBottom: 6,
		marginTop: 0,
		opacity: 0.7,
		paddingLeft: 8 * 8
	}
};

export default withAccountStore(withNavigation(IdentitiesSwitch));
