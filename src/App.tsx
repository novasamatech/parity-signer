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

import '../shim';
import 'utils/iconLoader';
import * as React from 'react';
import { StatusBar, StyleSheet, View, YellowBox } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { Provider as UnstatedProvider } from 'unstated';
import { MenuProvider } from 'react-native-popup-menu';
import { SafeAreaProvider } from 'react-native-safe-area-context';

import {
	AppNavigator,
	TocAndPrivacyPolicyNavigator,
	ScreenStack
} from './screens';

import colors from 'styles/colors';
import '../ReactotronConfig';
import { AppProps, getLaunchArgs } from 'e2e/injections';
import { GlobalState, GlobalStateContext } from 'stores/globalStateContext';
import { loadToCAndPPConfirmation } from 'utils/db';
import { migrateAccounts, migrateIdentity } from 'utils/migrationUtils';

export default function App(props: AppProps): React.ReactElement {
	getLaunchArgs(props);
	if (__DEV__) {
		YellowBox.ignoreWarnings([
			'Warning: componentWillReceiveProps',
			'Warning: componentWillMount',
			'Warning: componentWillUpdate',
			'Sending `onAnimatedValueUpdate`',
			'Non-serializable values were found in the navigation state' // https://reactnavigation.org/docs/troubleshooting/#i-get-the-warning-non-serializable-values-were-found-in-the-navigation-state
		]);
	}

	const [policyConfirmed, setPolicyConfirmed] = React.useState<boolean>(false);
	const [dataLoaded, setDataLoaded] = React.useState<boolean>(false);
	React.useEffect(() => {
		const loadPolicyConfirmationAndMigrateData = async (): Promise<void> => {
			const tocPP = await loadToCAndPPConfirmation();
			setPolicyConfirmed(tocPP);
			if (!tocPP) {
				await migrateAccounts();
				await migrateIdentity();
			}
		};
		setDataLoaded(true);
		loadPolicyConfirmationAndMigrateData();
	}, []);

	const globalContext: GlobalState = {
		dataLoaded,
		policyConfirmed,
		setDataLoaded,
		setPolicyConfirmed
	};

	const renderStacks = (): React.ReactElement => {
		if (dataLoaded) {
			return policyConfirmed ? (
				<AppNavigator />
			) : (
				<TocAndPrivacyPolicyNavigator />
			);
		} else {
			return (
				<ScreenStack.Navigator>
					<ScreenStack.Screen name="Empty">
						{(navigationProps: any): React.ReactElement => (
							<View style={emptyScreenStyles} {...navigationProps} />
						)}
					</ScreenStack.Screen>
				</ScreenStack.Navigator>
			);
		}
	};

	return (
		<SafeAreaProvider>
			<UnstatedProvider>
				<MenuProvider backHandler={true}>
					<StatusBar barStyle="light-content" backgroundColor={colors.bg} />
					<GlobalStateContext.Provider value={globalContext}>
						<NavigationContainer>{renderStacks()}</NavigationContainer>
					</GlobalStateContext.Provider>
				</MenuProvider>
			</UnstatedProvider>
		</SafeAreaProvider>
	);
}

const emptyScreenStyles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		padding: 20
	}
});
