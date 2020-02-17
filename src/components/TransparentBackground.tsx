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

import React from 'react';
import {
	Modal,
	StyleSheet,
	TouchableWithoutFeedback,
	View,
	ViewStyle
} from 'react-native';

interface Props {
	animationType: 'none' | 'slide' | 'fade';
	setVisible: (isVisible: boolean) => void;
	style?: ViewStyle;
	testID?: string;
	visible: boolean;
	children: any;
}

export default function TransparentBackground({
	children,
	visible,
	setVisible,
	testID,
	style,
	animationType
}: Props): React.ReactElement {
	return (
		<Modal
			animationType={animationType}
			visible={visible}
			transparent={true}
			onRequestClose={(): void => setVisible(false)}
		>
			<TouchableWithoutFeedback
				style={{ flex: 1 }}
				onPressIn={(): void => setVisible(false)}
			>
				<View
					testID={testID}
					style={StyleSheet.flatten([styles.container, style])}
				>
					{children}
				</View>
			</TouchableWithoutFeedback>
		</Modal>
	);
}

const styles = StyleSheet.create({
	container: {
		backgroundColor: 'rgba(0,0,0,0.8)',
		flex: 1
	}
});
