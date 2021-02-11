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

import '../shim';
import 'utils/iconLoader';
import '../ReactotronConfig';

import { NavigationContainer } from '@react-navigation/native';
import CustomAlert from 'components/CustomAlert';
import { AppProps, getLaunchArgs } from 'e2e/injections';
import * as React from 'react';
import { LogBox,StatusBar, StyleSheet, View } from 'react-native';
import NavigationBar from 'react-native-navbar-color';
import { MenuProvider } from 'react-native-popup-menu';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { AlertStateContext, useAlertContext } from 'stores/alertContext';
import { GlobalState, GlobalStateContext, useGlobalStateContext } from 'stores/globalStateContext';
import { RegistriesContext, useRegistriesStore } from 'stores/RegistriesContext';
import { ScannerContext,useScannerContext } from 'stores/ScannerContext';
import { SeedRefsContext, useSeedRefStore } from 'stores/SeedRefStore';
import colors from 'styles/colors';

import { useAccountContext } from './context/AccountsContext';
import { AccountsContext, NetworksContextProvider } from './context';
import { AppNavigator, ScreenStack, TocAndPrivacyPolicyNavigator } from './screens';

export default function App(props: AppProps): React.ReactElement {
	getLaunchArgs(props);
	NavigationBar.setColor(colors.background.os);

	if (__DEV__) {
		LogBox.ignoreLogs([
			'Warning: componentWillReceiveProps',
			'Warning: componentWillMount',
			'Warning: componentWillUpdate',
			'Sending `onAnimatedValueUpdate`',
			'MenuProviders',
			'Non-serializable values were found in the navigation state' // https://reactnavigation.org/docs/troubleshooting/#i-get-the-warning-non-serializable-values-were-found-in-the-navigation-state
		]);
	}

	const alertContext = useAlertContext();
	const globalContext: GlobalState = useGlobalStateContext();
	const seedRefContext = useSeedRefStore();
	// const networkContext = useNetworksContext();
	const accountsContext = useAccountContext();
	const scannerContext = useScannerContext();
	const registriesContext = useRegistriesStore();

	const renderStacks = (): React.ReactElement => {
		if (globalContext.dataLoaded) {
			return globalContext.policyConfirmed ? (
				<AppNavigator />
			) : (
				<TocAndPrivacyPolicyNavigator />
			);
		} else {
			return (
				<ScreenStack.Navigator>
					<ScreenStack.Screen name="Empty">
						{(navigationProps: any): React.ReactElement => (
							<View style={emptyScreenStyles}
								{...navigationProps} />
						)}
					</ScreenStack.Screen>
				</ScreenStack.Navigator>
			);
		}
	};

	return (
		<SafeAreaProvider>
			<NetworksContextProvider>
				<AccountsContext.Provider value={accountsContext}>
					<ScannerContext.Provider value={scannerContext}>
						<RegistriesContext.Provider value={registriesContext}>
							<GlobalStateContext.Provider value={globalContext}>
								<AlertStateContext.Provider value={alertContext}>
									<SeedRefsContext.Provider value={seedRefContext}>
										<MenuProvider backHandler={true}>
											<StatusBar
												backgroundColor={colors.background.app}
												barStyle="light-content"
											/>
											<CustomAlert />
											<NavigationContainer>
												{renderStacks()}
											</NavigationContainer>
										</MenuProvider>
									</SeedRefsContext.Provider>
								</AlertStateContext.Provider>
							</GlobalStateContext.Provider>
						</RegistriesContext.Provider>
					</ScannerContext.Provider>
				</AccountsContext.Provider>
			</NetworksContextProvider>
		</SafeAreaProvider>
	);
}

const emptyScreenStyles = StyleSheet.create({
	body: {
		backgroundColor: colors.background.app,
		flex: 1,
		flexDirection: 'column',
		padding: 20
	}
});
