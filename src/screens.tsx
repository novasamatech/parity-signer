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

import NetworkDetails from 'modules/network/screens/NetworkDetails';
import NetworkSettings from 'modules/network/screens/NetworkSettings';
import PinNew from 'modules/unlock/screens/PinNew';
import PinUnlock from 'modules/unlock/screens/PinUnlock';
import PinUnlockWithPassword from 'modules/unlock/screens/PinUnlockWithPassword';
import HeaderLeftHome from 'components/HeaderLeftHome';
import SecurityHeader from 'components/SecurityHeader';
import testIDs from 'e2e/testIDs';
import About from 'screens/About';
import AccountDetails from 'screens/AccountDetails';
import AccountEdit from 'screens/AccountEdit';
import Main from 'modules/main/screens/Main';
import AccountNew from 'screens/AccountNew';
import AccountPin from 'screens/AccountPin';
import { AccountUnlock, AccountUnlockAndSign } from 'screens/AccountUnlock';
import IdentityBackup from 'screens/IdentityBackup';
import IdentityManagement from 'screens/IdentityManagement';
import IdentityNew from 'screens/IdentityNew';
import LegacyAccountBackup from 'screens/LegacyAccountBackup';
import LegacyAccountList from 'screens/LegacyAccountList';
import LegacyNetworkChooser from 'screens/LegacyNetworkChooser';
import MetadataManagement from 'modules/network/screens/MetadataManagement';
import FullMetadata from 'modules/network/screens/FullMetadata';
import MetadataSaving from 'screens/MetadataSaving';
import PathDerivation from 'screens/PathDerivation';
import PathDetails from 'screens/PathDetails';
import PathManagement from 'screens/PathManagement';
import PathSecret from 'screens/PathSecret';
import PathsList from 'screens/PathsList';
import PrivacyPolicy from 'screens/PrivacyPolicy';
import QrScanner from 'modules/sign/screens/QrScanner';
import FastQrScanner from 'screens/FastQrScanner';
import Security from 'screens/Security';
import DetailsMessage from 'modules/sign/screens/DetailsMessage';
import SignedMessage from 'modules/sign/screens/SignedMessage';
import DetailsTx from 'modules/sign/screens/DetailsTx';
import SignedTx from 'modules/sign/screens/SignedTx';
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
		<ScreenStack.Screen name="AccountDetails" component={AccountDetails} />
		<ScreenStack.Screen name="AccountEdit" component={AccountEdit} />
		<ScreenStack.Screen name="AccountPin" component={AccountPin} />
		<ScreenStack.Screen name="AccountUnlock" component={AccountUnlock} />
		<ScreenStack.Screen name="AccountNew" component={AccountNew} />
		<ScreenStack.Screen
			name="AccountUnlockAndSign"
			component={AccountUnlockAndSign}
		/>
		<ScreenStack.Screen
			name="LegacyAccountBackup"
			component={LegacyAccountBackup}
		/>
		<ScreenStack.Screen
			name="LegacyAccountList"
			component={LegacyAccountList}
		/>
		<ScreenStack.Screen
			name="LegacyNetworkChooser"
			component={LegacyNetworkChooser}
		/>
		<ScreenStack.Screen name="IdentityBackup" component={IdentityBackup} />
		<ScreenStack.Screen
			name="IdentityManagement"
			component={IdentityManagement}
		/>
		<ScreenStack.Screen name="IdentityNew" component={IdentityNew} />
		<ScreenStack.Screen name="NetworkDetails" component={NetworkDetails} />
		<ScreenStack.Screen name="NetworkSettings" component={NetworkSettings} />
		<ScreenStack.Screen
			name="MetadataManagement"
			component={MetadataManagement}
		/>
		<ScreenStack.Screen name="FullMetadata" component={FullMetadata} />
		<ScreenStack.Screen name="MetadataSaving" component={MetadataSaving} />
		<ScreenStack.Screen name="PathDerivation" component={PathDerivation} />
		<ScreenStack.Screen name="PathDetails" component={PathDetails} />
		<ScreenStack.Screen name="PathsList" component={PathsList} />
		<ScreenStack.Screen name="PathSecret" component={PathSecret} />
		<ScreenStack.Screen name="PathManagement" component={PathManagement} />
		<ScreenStack.Screen name="PinNew" component={PinNew} />
		<ScreenStack.Screen name="PinUnlock" component={PinUnlock} />
		<ScreenStack.Screen
			name="PinUnlockWithPassword"
			component={PinUnlockWithPassword}
		/>
		<ScreenStack.Screen name="QrScanner" component={QrScanner} />
		<ScreenStack.Screen name="FastQrScanner" component={FastQrScanner} />
		<ScreenStack.Screen name="Security" component={Security} />
		<ScreenStack.Screen name="DetailsMessage" component={DetailsMessage} />
		<ScreenStack.Screen name="SignedMessage" component={SignedMessage} />
		<ScreenStack.Screen name="DetailsTx" component={DetailsTx} />
		<ScreenStack.Screen name="SignedTx" component={SignedTx} />
		<ScreenStack.Screen
			name="TermsAndConditions"
			component={TermsAndConditions}
		/>
		<ScreenStack.Screen name="PrivacyPolicy" component={PrivacyPolicy} />
	</ScreenStack.Navigator>
);

export const TocAndPrivacyPolicyNavigator = (): React.ReactElement => (
	<ScreenStack.Navigator
		initialRouteName="TermsAndConditions"
		screenOptions={{
			...globalStackNavigationOptions,
			headerRight: (): React.ReactNode => null
		}}
	>
		<ScreenStack.Screen
			name="TermsAndConditions"
			component={TermsAndConditions}
		/>
		<ScreenStack.Screen name="PrivacyPolicy" component={PrivacyPolicy} />
	</ScreenStack.Navigator>
);
