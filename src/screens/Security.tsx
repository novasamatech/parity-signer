// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading from 'components/ScreenHeading';
import React from 'react';

export default class Security extends React.PureComponent {
	render(): React.ReactElement {
		return (
			<SafeAreaScrollViewContainer>
				<ScreenHeading
					error={true}
					iconName="shield-off"
					iconType="feather"
					subtitle="A device is considered not secure if it has access to the internet or
					has any kind of connectivity enabled. Stylo is meant to be
					used on a device that will be kept offline at any time. Enabling any
					connectivity such as wifi, cellular network, bluetooth, NFC, usb is a
					threat to the safety of the private keys stored on the device."
					subtitleL={true}
					title="Insecure device"
				/>
			</SafeAreaScrollViewContainer>
		);
	}
}
