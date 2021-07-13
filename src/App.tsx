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
import * as React from 'react';
import { StatusBar, LogBox } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { MenuProvider } from 'react-native-popup-menu';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import NavigationBar from 'react-native-navbar-color';

import { AppNavigator } from './screens';

import CustomAlert from 'components/CustomAlert';
import colors from 'styles/colors';
import '../ReactotronConfig';
import { AppProps, getLaunchArgs } from 'e2e/injections';
import { AlertStateContext, useAlertContext } from 'stores/alertContext';

export default function App(props: AppProps): React.ReactElement {
	getLaunchArgs(props);
	NavigationBar.setColor(colors.background.os);
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

	const alertContext = useAlertContext();

	return (
		<SafeAreaProvider>
			<AlertStateContext.Provider value={alertContext}>
				<MenuProvider backHandler={true}>
					<StatusBar
						barStyle="light-content"
						backgroundColor={colors.background.app}
					/>
					<CustomAlert />
					<NavigationContainer>
						<AppNavigator />
					</NavigationContainer>
				</MenuProvider>
			</AlertStateContext.Provider>
		</SafeAreaProvider>
	);
}
