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
import ButtonIcon from './ButtonIcon';
import Separator from './Separator';
import fontStyles from '../fontStyles';
import colors from '../colors';
import { ScrollView, Modal, View } from 'react-native';
import { withNavigation } from 'react-navigation';

function IdentitiesSwitch({ navigation }) {
	const [visible, setVisible] = useState(false);

	return (
		<View>
			<ButtonIcon
				onPress={() => setVisible(!visible)}
				iconName="md-finger-print"
				iconType="ionicon"
			/>
			<Modal animationType="fade" visible={visible} transparent={true}>
				<View style={styles.container}>
					<View style={styles.headerStyle}>
						<ButtonIcon
							onPress={() => {
								setVisible(false);
							}}
							iconName="md-close"
							iconType="ionicon"
							iconBgStyle={{ backgroundColor: colors.card_bgSolid }}
						/>
					</View>
					<View style={styles.card}>
						<ButtonIcon
							title="Current Identity Title"
							onPress={() => {
								setVisible(false);
								//go to current Identity
							}}
							iconName="md-finger-print"
							iconType="ionicon"
							iconSize={40}
							style={{ paddingLeft: 8 * 2 }}
							textStyle={fontStyles.h1}
						/>
						<ButtonIcon
							title="Manage Identity"
							onPress={() => {
								setVisible(false);
								//go to current IdentityManage
							}}
							iconBgStyle={styles.i_arrowBg}
							iconName="ios-arrow-round-forward"
							iconType="ionicon"
							iconSize={24}
							textStyle={fontStyles.t_regular}
							style={styles.i_arrowStyle}
						/>
						<ButtonIcon
							title="Backup Identity"
							onPress={() => {
								setVisible(false);
								//go to current IdentityBackup
							}}
							iconBgStyle={styles.i_arrowBg}
							iconName="ios-arrow-round-forward"
							iconType="ionicon"
							iconSize={24}
							textStyle={fontStyles.t_regular}
							style={styles.i_arrowStyle}
						/>

						{
							// if [identities].lenghts > 4
							// <Separator />
							// else: scrollView
						}

						<Separator style={{ marginBottom: 0 }} />
						<ScrollView style={styles.idListContent}>
							<ButtonIcon
								title="Legacy Seeds"
								onPress={() => {
									setVisible(false);
									navigation.navigate('IdentityNew');
								}}
								iconName="ios-snow"
								iconType="ionicon"
								iconSize={24}
								textStyle={fontStyles.h2}
								style={{ paddingLeft: 8 * 4 }}
							/>
						</ScrollView>
						<Separator shadow={true} style={{ marginTop: 0 }} />

						<ButtonIcon
							title="Add new Identity"
							onPress={() => {
								setVisible(false);
								navigation.navigate('IdentityNew');
							}}
							iconName="ios-add"
							iconType="ionicon"
							iconSize={24}
							textStyle={fontStyles.t_big}
							style={{ paddingLeft: 8 * 4 }}
						/>
						<Separator />
						<ButtonIcon
							title="Settings"
							onPress={() => {
								setVisible(false);
								// go to Settings;
							}}
							iconName="ios-cog"
							iconType="ionicon"
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
							iconName="ios-arrow-round-forward"
							iconType="ionicon"
							iconSize={24}
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
							iconName="ios-arrow-round-forward"
							iconType="ionicon"
							iconSize={24}
							textStyle={fontStyles.t_regular}
							style={styles.i_arrowStyle}
						/>
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
		marginTop: 16,
		paddingBottom: 8,
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
		justifyContent: 'center',
		paddingRight: 62
	},
	i_arrowBg: {
		backgroundColor: 'rgba(0,0,0,0)',
		width: 12
	},
	i_arrowStyle: {
		marginTop: 0,
		opacity: 0.7,
		paddingLeft: 8 * 8
	},
	idListContent: {
		maxHeight: 200,
		paddingBottom: 8,
		paddingTop: 8
	}
};

export default withNavigation(IdentitiesSwitch);
