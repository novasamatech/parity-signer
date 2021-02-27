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
import styles from 'modules/main/styles';
import React from 'react';
import { Text, View } from 'react-native';
import fontStyles from 'styles/fontStyles';

export default function NoCurrentIdentity(): React.ReactElement {
	return (
		<SafeAreaScrollViewContainer contentContainerStyle={styles.scrollContent}>
			<View style={styles.onboardingWrapper}>
				<Text style={fontStyles.quote}>
					Select one of your identity to get started.
				</Text>
			</View>
		</SafeAreaScrollViewContainer>
	);
}
