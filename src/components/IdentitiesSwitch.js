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
import { Modal, Text, View } from 'react-native';

export default function IdentitiesSwitch(props) {
	const [visible, setVisible] = useState(false);

	return (
		<View>
			<Button title="identities" onPress={() => setVisible(!visible)} />
			<Modal animationType="fade" visible={visible} transparent={true}>
				<View style={styles.container}>
					<Text>Identities Switch</Text>
					<Button
						title="Close"
						onPress={() => {
							setVisible(false);
						}}
					/>
				</View>
			</Modal>
		</View>
	);
}

const styles = {
	container: {
		alignItems: 'center',
		backgroundColor: 'rgba(0,0,0,0.7)',
		flex: 1,
		justifyContent: 'center'
	}
};
