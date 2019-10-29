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
import Button from './Button';
import ButtonIcon from './ButtonIcon';
import Separator from './Separator';
import fontStyles from '../fontStyles';
import colors from '../colors';
import { Modal, View } from 'react-native';
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
							iconStyle={{ backgroundColor: 'rgba(0,0,0,0)' }}
							iconName="md-arrow-forward"
							iconType="ionicon"
							iconSize={18}
							textStyle={fontStyles.t_regular}
							style={{ opacity: 0.7, paddingLeft: 8 * 8 }}
						/>
						<ButtonIcon
							title="Backup Identity"
							onPress={() => {
								setVisible(false);
								//go to current IdentityBackup
							}}
							iconStyle={{ backgroundColor: 'rgba(0,0,0,0)' }}
							iconName="md-arrow-forward"
							iconType="ionicon"
							iconSize={18}
							textStyle={fontStyles.t_regular}
							style={{ opacity: 0.7, paddingLeft: 8 * 8 }}
						/>
						<Separator />
						<ButtonIcon
							title="Add new Identity"
							onPress={() => {
								setVisible(false);
								navigation.navigate('IdentityNew');
							}}
							iconName="md-finger-print"
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
							iconName="md-settings"
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
							iconStyle={{ backgroundColor: 'rgba(0,0,0,0)' }}
							iconName="md-arrow-forward"
							iconType="ionicon"
							iconSize={18}
							textStyle={fontStyles.t_regular}
							style={{ opacity: 0.7, paddingLeft: 8 * 8 }}
						/>
						<ButtonIcon
							title="Privacy Policy"
							onPress={() => {
								setVisible(false);
								navigation.navigate('PrivacyPolicy');
							}}
							iconStyle={{ backgroundColor: 'rgba(0,0,0,0)' }}
							iconName="md-arrow-forward"
							iconType="ionicon"
							iconSize={18}
							textStyle={fontStyles.t_regular}
							style={{ opacity: 0.7, paddingLeft: 8 * 8 }}
						/>
						<Button
							title="Close"
							onPress={() => {
								setVisible(false);
							}}
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
		paddingBottom: 8,
		paddingTop: 8
	},
	container: {
		backgroundColor: 'rgba(0,0,0,0.7)',
		flex: 1,
		justifyContent: 'center',
		paddingLeft: 16,
		paddingRight: 16
	}
};

export default withNavigation(IdentitiesSwitch);
