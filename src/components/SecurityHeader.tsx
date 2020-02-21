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

import NetInfo from '@react-native-community/netinfo';
import React, { useEffect, useState } from 'react';
import { StyleSheet, View } from 'react-native';
import { NavigationInjectedProps, withNavigation } from 'react-navigation';

import ButtonIcon from './ButtonIcon';

import testIDs from 'e2e/testIDs';
import colors from 'styles/colors';
import IdentitiesSwitch from 'components/IdentitiesSwitch';
import { navigateToQrScanner } from 'utils/navigationHelpers';

function SecurityHeader({
	navigation
}: NavigationInjectedProps): React.ReactElement<NavigationInjectedProps> {
	const [isConnected, setIsConnected] = useState(false);

	useEffect(
		() =>
			NetInfo.addEventListener(state => {
				setIsConnected(state.isConnected);
			}),
		[]
	);

	return (
		<View style={styles.body}>
			{isConnected && (
				<ButtonIcon
					onPress={(): boolean => navigation.navigate('Security')}
					iconName="shield-off"
					iconType="feather"
					iconColor={colors.bg_alert}
					iconBgStyle={styles.securityIconBgStyle}
				/>
			)}
			<ButtonIcon
				onPress={(): void => navigateToQrScanner(navigation)}
				iconName="qrcode-scan"
				iconType="material-community"
				iconBgStyle={styles.scannerIconBgStyle}
				testID={testIDs.SecurityHeader.scanButton}
			/>
			<IdentitiesSwitch />
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		alignItems: 'center',
		flexDirection: 'row',
		paddingRight: 16
	},
	scannerIconBgStyle: { backgroundColor: 'transparent' },
	securityIconBgStyle: { backgroundColor: 'transparent', marginTop: -3 }
});

export default withNavigation(SecurityHeader);
