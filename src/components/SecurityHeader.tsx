// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import React from 'react';
import { StyleSheet, View } from 'react-native';

import IdentitiesSwitch from 'components/IdentitiesSwitch';

function SecurityHeader(): React.ReactElement {
	return (
		<View style={styles.body}>
			<IdentitiesSwitch />
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		flexDirection: 'row',
		justifyContent: 'center'
	}
});

export default SecurityHeader;
