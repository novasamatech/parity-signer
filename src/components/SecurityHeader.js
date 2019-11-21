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

import NetInfo from '@react-native-community/netinfo';
import React, { useEffect, useState } from 'react';
import { View } from 'react-native';
import { withNavigation } from 'react-navigation';

import colors from '../colors';
import IdentitiesSwitch from '../components/IdentitiesSwitch';
import ButtonIcon from './ButtonIcon';

function SecurityHeader({ navigation }) {
	const [isConnected, setIsConnected] = useState(false);

	useEffect(
		() =>
			NetInfo.addEventListener(state => {
				setIsConnected(state.isConnected);
			}),
		[]
	);

	return (
		<View
			style={{
				alignItems: 'center',
				flexDirection: 'row',
				paddingRight: 16
			}}
		>
			{isConnected && (
				<ButtonIcon
					onPress={() => navigation.navigate('Security')}
					iconName="shield-off"
					iconType="feather"
					iconColor={colors.bg_alert}
					iconBgStyle={{ backgroundColor: 'transparent', marginTop: -3 }}
				/>
			)}
			<IdentitiesSwitch />
		</View>
	);
}

export default withNavigation(SecurityHeader);
