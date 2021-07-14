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

import {
	useNavigation,
	useNavigationState,
	useRoute
} from '@react-navigation/native';
import {
	CardStyleInterpolators,
	createStackNavigator,
	HeaderBackButton
} from '@react-navigation/stack';
import * as React from 'react';
import { View } from 'react-native';

import NetworkDetails from 'screens/NetworkDetails';
import HeaderLeftHome from 'components/HeaderLeftHome';
import SecurityHeader from 'components/SecurityHeader';
import testIDs from 'e2e/testIDs';
import About from 'screens/About';
import Main from 'screens/Main';
import SeedBackup from 'screens/SeedBackup';
import RootSeedNew from 'screens/RootSeedNew';
import PathDerivation from 'screens/PathDerivation';
import PathsList from 'screens/PathsList';
import PrivacyPolicy from 'screens/PrivacyPolicy';
import FastQrScanner from 'screens/FastQrScanner';
import Security from 'screens/Security';
import DetailsTx from 'screens/DetailsTx';
import SignedTx from 'screens/SignedTx';
import TermsAndConditions from 'screens/TermsAndConditions';
import colors from 'styles/colors';
import { headerHeight, horizontalPadding } from 'styles/containerStyles';
import { RootStackParamList } from 'types/routes';

export const ScreenStack = createStackNavigator<RootStackParamList>();

const HeaderLeft = (): React.ReactElement => {
	const route = useRoute();
	const isFirstRouteInParent = useNavigationState(
		state => state.routes[0].key === route.key
	);
	return isFirstRouteInParent ? <HeaderLeftHome /> : <HeaderLeftWithBack />;
};

const globalStackNavigationOptions = {
	//more transition animations refer to: https://reactnavigation.org/docs/en/stack-navigator.html#animations
	cardStyleInterpolator: CardStyleInterpolators.forHorizontalIOS,
	headerBackTitleStyle: {
		color: colors.text.main
	},
	headerBackTitleVisible: false,
	headerLeft: (): React.ReactElement => <HeaderLeft />,
	headerLeftContainerStyle: {
		height: headerHeight,
		paddingLeft: 8
	},
	headerRight: (): React.ReactElement => <SecurityHeader />,
	headerRightContainerStyle: {
		height: headerHeight,
		paddingRight: horizontalPadding
	},
	headerStyle: {
		backgroundColor: colors.background.app,
		borderBottomColor: colors.background.app,
		borderBottomWidth: 0,
		elevation: 0,
		height: headerHeight,
		shadowColor: 'transparent'
	},
	headerTintColor: colors.text.main,
	headerTitle: (): React.ReactNode => null
};

const HeaderLeftWithBack = (): React.ReactElement => {
	const navigation = useNavigation();
	return (
		<View testID={testIDs.Header.headerBackButton}>
			<HeaderBackButton
				labelStyle={globalStackNavigationOptions.headerBackTitleStyle}
				labelVisible={false}
				tintColor={colors.text.main}
				onPress={(): void => navigation.goBack()}
			/>
		</View>
	);
};

export const AppNavigator = (): React.ReactElement => (
	<ScreenStack.Navigator
		initialRouteName="Main"
		screenOptions={globalStackNavigationOptions}
	>
		<ScreenStack.Screen name="Main" component={Main} />
		<ScreenStack.Screen name="About" component={About} />
		<ScreenStack.Screen name="SeedBackup" component={SeedBackup} />
		<ScreenStack.Screen name="RootSeedNew" component={RootSeedNew} />
		<ScreenStack.Screen name="NetworkDetails" component={NetworkDetails} />
		<ScreenStack.Screen name="PathDerivation" component={PathDerivation} />
		<ScreenStack.Screen name="PathsList" component={PathsList} />
		<ScreenStack.Screen name="FastQrScanner" component={FastQrScanner} />
		<ScreenStack.Screen name="Security" component={Security} />
		<ScreenStack.Screen name="DetailsTx" component={DetailsTx} />
		<ScreenStack.Screen name="SignedTx" component={SignedTx} />
		<ScreenStack.Screen
			name="TermsAndConditions"
			component={TermsAndConditions}
		/>
		<ScreenStack.Screen name="PrivacyPolicy" component={PrivacyPolicy} />
	</ScreenStack.Navigator>
);
