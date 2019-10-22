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

import '../shim';

import '@polkadot/types/injector';

import React, { Component } from 'react';
import { StatusBar, YellowBox } from 'react-native';
import {
	createAppContainer,
	createStackNavigator,
	HeaderBackButton,
	withNavigation
} from 'react-navigation';
import { Provider as UnstatedProvider } from 'unstated';
import { MenuProvider } from 'react-native-popup-menu';

import '../shim';
import Background from './components/Background';
import colors from './colors';
import fonts from './fonts';
import HeaderLeftHome from './components/HeaderLeftHome';
import SecurityHeader from './components/SecurityHeader';
import '../ReactotronConfig';
import About from './screens/About';
import AccountBackup from './screens/AccountBackup';
import AccountDetails from './screens/AccountDetails';
import AccountEdit from './screens/AccountEdit';
import AccountList from './screens/AccountList';
import AccountNetworkChooser from './screens/AccountNetworkChooser';
import AccountNew from './screens/AccountNew';
import AccountPin from './screens/AccountPin';
import AccountRecover from './screens/AccountRecover';
import AccountSettings from './screens/AccountSettings';
import { AccountUnlock, AccountUnlockAndSign } from './screens/AccountUnlock';
import Loading from './screens/Loading';
import MessageDetails from './screens/MessageDetails';
import NetworkSettings from './screens/NetworkSettings';
import PrivacyPolicy from './screens/PrivacyPolicy';
import QrScanner from './screens/QrScanner';
import Security from './screens/Security';
import Settings from './screens/Settings';
import SignedMessage from './screens/SignedMessage';
import SignedTx from './screens/SignedTx';
import TermsAndConditions from './screens/TermsAndConditions';
import TxDetails from './screens/TxDetails';

export default class App extends Component {
	constructor() {
		super();
		if (__DEV__) {
			YellowBox.ignoreWarnings([
				'Warning: componentWillReceiveProps',
				'Warning: componentWillMount',
				'Warning: componentWillUpdate'
			]);
		}
	}

	render() {
		return (
			<UnstatedProvider>
				<MenuProvider backHandler={true}>
					<StatusBar barStyle="light-content" />
					<Background />
					<ScreensContainer />
				</MenuProvider>
			</UnstatedProvider>
		);
	}
}

const globalStackNavigationOptions = {
	headerBackTitleStyle: {
		fontFamily: fonts.semiBold,
		fontSize: 20
	},
	headerRight: <SecurityHeader />,
	headerStyle: {
		backgroundColor: colors.bg,
		borderBottomColor: colors.bg_text_sec,
		borderBottomWidth: 0.5,
		height: 60,
		paddingBottom: 0,
		paddingTop: 0
	},
	headerTintColor: colors.card_bg,
	headerTitleStyle: {
		display: 'none'
	}
};

// A workaround for https://github.com/react-navigation/react-navigation/issues/88
const SecurityHeaderBackButton = withNavigation(
	class _HeaderBackButton extends Component {
		render() {
			const { navigation } = this.props;
			return (
				<HeaderBackButton
					{...this.props}
					titleStyle={globalStackNavigationOptions.headerBackTitleStyle}
					title="Back"
					tintColor={colors.card_bg}
					onPress={() => navigation.goBack(null)}
				/>
			);
		}
	}
);

/* eslint-disable sort-keys */
const Screens = createStackNavigator(
	{
		Loading: {
			screen: Loading
		},
		Security: {
			screen: createStackNavigator(
				{
					Security: {
						navigationOptions: {
							headerLeft: <SecurityHeaderBackButton />,
							headerRight: null
						},
						screen: Security
					}
				},
				{
					defaultNavigationOptions: globalStackNavigationOptions,
					headerMode: 'screen'
				}
			)
		},
		TocAndPrivacyPolicy: {
			screen: createStackNavigator(
				{
					TermsAndConditions: {
						navigationOptions: {
							headerLeft: <HeaderLeftHome />
						},
						screen: TermsAndConditions
					},
					PrivacyPolicy: {
						screen: PrivacyPolicy
					}
				},
				{
					defaultNavigationOptions: globalStackNavigationOptions,
					initialRouteParams: {
						isWelcome: true
					}
				}
			)
		},
		Welcome: {
			screen: createStackNavigator(
				{
					AccountList: {
						navigationOptions: {
							headerLeft: <HeaderLeftHome />
						},
						screen: AccountList
					},
					About: {
						screen: About
					},
					AccountBackup: {
						screen: AccountBackup
					},
					AccountDetails: {
						screen: AccountDetails
					},
					AccountEdit: {
						screen: AccountEdit
					},
					AccountNetworkChooser: {
						screen: AccountNetworkChooser
					},
					AccountNew: {
						screen: AccountNew
					},
					AccountPin: {
						screen: AccountPin
					},
					AccountRecover: {
						screen: AccountRecover
					},
					AccountSettings: {
						screen: AccountSettings
					},
					AccountUnlock: {
						screen: AccountUnlock
					},
					AccountUnlockAndSign: {
						screen: AccountUnlockAndSign
					},
					MessageDetails: {
						screen: MessageDetails
					},
					NetworkSettings: {
						screen: NetworkSettings
					},
					QrScanner: {
						screen: QrScanner
					},
					Settings: {
						screen: Settings
					},
					SignedMessage: {
						screen: SignedMessage
					},
					SignedTx: {
						screen: SignedTx
					},
					TxDetails: {
						screen: TxDetails
					}
				},
				{
					defaultNavigationOptions: globalStackNavigationOptions,
					initialRouteParams: {
						isWelcome: true
					}
				}
			)
		}
	},
	{
		defaultNavigationOptions: globalStackNavigationOptions,
		headerMode: 'none',
		mode: 'card'
	}
);

const ScreensContainer = createAppContainer(Screens);
