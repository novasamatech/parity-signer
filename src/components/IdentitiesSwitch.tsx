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

import { StackNavigationProp } from '@react-navigation/stack';
import React, { useState } from 'react';
import { ScrollView, StyleSheet, View } from 'react-native';
import { useNavigation } from '@react-navigation/native';

import ButtonIcon from './ButtonIcon';
import Separator from './Separator';
import TransparentBackground from './TransparentBackground';

import { RootStackParamList } from 'types/routes';
import testIDs from 'e2e/testIDs';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import {
	resetNavigationTo,
	resetNavigationWithNetworkChooser
} from 'utils/navigationHelpers';
import { Identity } from 'types/identityTypes';

//TODO: rename this screen and clean up

function ButtonWithArrow(props: {
	onPress: () => void;
	testID?: string;
	title: string;
}): React.ReactElement {
	return <ButtonIcon {...props} {...i_arrowOptions} />;
}

function IdentitiesSwitch({}: Record<string, never>): React.ReactElement {
	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();
	const [visible, setVisible] = useState(false);
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
		// @ts-ignore: https://github.com/react-navigation/react-navigation/pull/8389/files breaks things
		navigation.navigate(screenName, params);
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
				<ButtonWithArrow
					title="Terms and Conditions"
					onPress={(): void => closeModalAndNavigate('TermsAndConditions')}
				/>
				<ButtonWithArrow
					title="Privacy Policy"
					onPress={(): void => closeModalAndNavigate('PrivacyPolicy')}
				/>
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
				style={{ paddingHorizontal: 6 }}
				iconSize={26}
			/>

			<TransparentBackground
				testID={testIDs.IdentitiesSwitch.modal}
				visible={visible}
				setVisible={setVisible}
				style={styles.container}
				animationType="none"
			>
				<View style={styles.card}>
					{renderSettings()}
				</View>
			</TransparentBackground>
		</View>
	);
}

const styles = StyleSheet.create({
	card: {
		backgroundColor: colors.background.app,
		borderRadius: 4,
		paddingBottom: 16,
		paddingTop: 8
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
	}
});

const i_arrowOptions = {
	iconColor: colors.signal.main,
	iconName: 'arrowright',
	iconSize: fontStyles.i_medium.fontSize,
	iconType: 'antdesign',
	style: styles.i_arrowStyle,
	textStyle: { ...fontStyles.a_text, color: colors.signal.main }
};

export default IdentitiesSwitch;
