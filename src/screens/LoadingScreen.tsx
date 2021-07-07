// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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
import { StyleSheet, ActivityIndicator, Text } from 'react-native';

//import testIDs from 'e2e/testIDs';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';

type LoadingScreenParams = {
	infoText: string;
};

function LoadingScreen({ infoText }: LoadingScreenParams): React.ReactElement {
	return (
		<SafeAreaViewContainer style={styles.background}>
			<ActivityIndicator
				animating={true}
				color="red"
				size="large"
				style={styles.indicator}
			/>
			<Text style={fontStyles.quote}>Please wait</Text>
			<Text style={fontStyles.a_text}>{infoText}</Text>
		</SafeAreaViewContainer>
	);
}

const styles = StyleSheet.create({
	background: {
		backgroundColor: colors.background.app,
		borderRadius: 4,
		paddingBottom: 16,
		paddingTop: 8
	},
	indicator: {
		margin: 15
	}
});

export default LoadingScreen;
