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

import Identicon from '@polkadot/reactnative-identicon';
import PropTypes from 'prop-types';
import React, { useEffect, useState } from 'react';
import { Image, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import colors from '../colors';
import { NetworkProtocols } from '../constants';
import { blockiesIcon } from '../util/native';

export default function AccountIcon(props) {
	AccountIcon.propTypes = {
		address: PropTypes.string.isRequired,
		network: PropTypes.object,
		protocol: PropTypes.string.isRequired
	};

	const { address, protocol, style, network } = props;
	const [ethereumIconUri, setEthereumIconUri] = useState('');

	useEffect(() => {
		if (protocol === NetworkProtocols.ETHEREUM) {
			loadEthereumIcon(address);
		}
	}, []);

	const loadEthereumIcon = function(ethereumAddress) {
		blockiesIcon('0x' + ethereumAddress)
			.then(uri => {
				setEthereumIconUri(uri);
			})
			.catch(console.error);
	};

	if (address === '') {
		return (
			<View>
				<Image
					source={network.logo}
					style={{
						backgroundColor: colors.bg_text_sec,
						borderRadius: 40,
						height: 40,
						width: 40
					}}
				/>
			</View>
		);
	}
	if (address === 'new') {
		return (
			<View style={{ height: 40, width: 40 }}>
				<Icon name="add" color={colors.bg_text} size={32} />
			</View>
		);
	} else if (address === 'existed') {
		return (
			<View style={{ height: 40, width: 40 }}>
				<Icon name="arrow-back" color={colors.bg_text} size={32} />
			</View>
		);
	}
	if (protocol === NetworkProtocols.SUBSTRATE) {
		return <Identicon value={address} size={style.width || 40} />;
	} else if (protocol === NetworkProtocols.ETHEREUM && ethereumIconUri) {
		return (
			<Image
				source={{ uri: ethereumIconUri }}
				style={style || { height: 40, width: 40 }}
			/>
		);
	} else {
		// if there's no protocol or it's unknown we return a warning
		return <Icon color={colors.bg_text} name={'error'} size={44} />;
	}
}
