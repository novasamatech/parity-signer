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

import '../shim';
import 'utils/iconLoader';
import * as React from 'react';
import { StatusBar, LogBox } from 'react-native';
import { DefaultTheme, NavigationContainer } from '@react-navigation/native';
import { MenuProvider } from 'react-native-popup-menu';
import NavigationBar from 'react-native-navbar-color';
import FlashMessage from 'react-native-flash-message';

import { AppNavigator } from './screens';

import { colors, fonts } from 'styles/index';
import {
	useRegistriesStore,
	RegistriesContext
} from 'stores/RegistriesContext';
import { useNetworksContext, NetworksContext } from 'stores/NetworkContext';
import { useScannerContext, ScannerContext } from 'stores/ScannerContext';
import { useAccountContext, AccountsContext } from 'stores/AccountsContext';
import { SeedRefsContext, useSeedRefStore } from 'stores/SeedRefStore';
import '../ReactotronConfig';
import { AppProps, getLaunchArgs } from 'e2e/injections';

const navTheme = DefaultTheme;
navTheme.colors.background = colors.white;

export default function App(props: AppProps): React.ReactElement {
	getLaunchArgs(props);
	NavigationBar.setColor(colors.background.dark);
	if (global.inTest) {
		LogBox.ignoreAllLogs(true);
	} else if (__DEV__) {
		LogBox.ignoreLogs([
			'Warning: componentWillReceiveProps',
			'Warning: componentWillMount',
			'Warning: componentWillUpdate',
			'Sending `onAnimatedValueUpdate`',
			'MenuProviders',
			'Non-serializable values were found in the navigation state' // https://reactnavigation.org/docs/troubleshooting/#i-get-the-warning-non-serializable-values-were-found-in-the-navigation-state
		]);
	}

	const seedRefContext = useSeedRefStore();
	const networkContext = useNetworksContext();
	const accountsContext = useAccountContext();
	const scannerContext = useScannerContext();
	const registriesContext = useRegistriesStore();

	const renderStacks = (): React.ReactElement => {
		return <AppNavigator />;
	};

	return (
		<NetworksContext.Provider value={networkContext}>
			<AccountsContext.Provider value={accountsContext}>
				<ScannerContext.Provider value={scannerContext}>
					<RegistriesContext.Provider value={registriesContext}>
						<SeedRefsContext.Provider value={seedRefContext}>
							<MenuProvider backHandler={true}>
								<StatusBar
									barStyle="light-content"
									backgroundColor={colors.background.app}
								/>
								<NavigationContainer theme={navTheme}>
									{renderStacks()}
								</NavigationContainer>
								<FlashMessage
									position="top"
									style={{ backgroundColor: colors.background.accentDark }}
									textStyle={{ fontFamily: fonts.regular }}
									titleStyle={{ fontFamily: fonts.regular }}
									duration={3000}
								/>
							</MenuProvider>
						</SeedRefsContext.Provider>
					</RegistriesContext.Provider>
				</ScannerContext.Provider>
			</AccountsContext.Provider>
		</NetworksContext.Provider>
	);
}
