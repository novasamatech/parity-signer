// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import React from 'react';
import { Text, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import styles from 'modules/main/styles';
import { fontStyles } from 'styles';

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
