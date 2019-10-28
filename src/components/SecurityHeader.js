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
import React from 'react';
import { StyleSheet, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import { withNavigation } from 'react-navigation';

import colors from '../colors';
import fonts from '../fonts';
import TouchableItem from './TouchableItem';
import IdentitiesSwitch from '../components/IdentitiesSwitch';

class SecurityHeader extends React.Component {
	state = {
		isConnected: false
	};

	componentDidMount() {
		this.unsubscribe = NetInfo.addEventListener(state => {
			this.setState({ isConnected: state.isConnected });
		});
	}

	componentWillUnmount() {
		this.unsubscribe();
	}

	render() {
		const { isConnected } = this.state;

		if (!isConnected) {
			return null;
		}

		return (
			<TouchableItem onPress={() => this.props.navigation.navigate('Security')}>
				<View
					style={{
						alignItems: 'center',
						flexDirection: 'row',
						paddingRight: 14
					}}
				>
					<Icon style={styles.headerSecureIcon} name="security" />
					<IdentitiesSwitch />
				</View>
			</TouchableItem>
		);
	}
}

const styles = StyleSheet.create({
	headerSecureIcon: {
		color: colors.bg_alert,
		fontFamily: fonts.bold,
		fontSize: 20,
		marginLeft: 0,
		paddingRight: 5
	}
});

export default withNavigation(SecurityHeader);
