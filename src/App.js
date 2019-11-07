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
import LegacyAccountBackup from './screens/LegacyAccountBackup';
import AccountDetails from './screens/AccountDetails';
import AccountEdit from './screens/AccountEdit';
import AccountNetworkChooser from './screens/AccountNetworkChooser';
import AccountNew from './screens/AccountNew';
import AccountPin from './screens/AccountPin';
import AccountRecover from './screens/AccountRecover';
import { AccountUnlock, AccountUnlockAndSign } from './screens/AccountUnlock';
import LegacyAccountList from './screens/LegacyAccountList';
import Loading from './screens/Loading';
import IdentityBackup from './screens/IdentityBackup';
import IdentityManagement from './screens/IdentityManagement';
import IdentityNew from './screens/IdentityNew';
import IdentityPin from './screens/IdentityPin';
import MessageDetails from './screens/MessageDetails';
import PathDerivation from './screens/PathDerivation';
import PathDetails from './screens/PathDetails';
import PathsList from './screens/PathsList';
import PathManagement from './screens/PathManagement';
import PrivacyPolicy from './screens/PrivacyPolicy';
import QrScanner from './screens/QrScanner';
import Security from './screens/Security';
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
					defaultNavigationOptions: globalStackNavigationOptions
				}
			)
		},
		Welcome: {
			screen: createStackNavigator(
				{
					AccountNetworkChooser: {
						navigationOptions: {
							headerLeft: <HeaderLeftHome />
						},
						screen: AccountNetworkChooser
					},
					AccountPin: {
						screen: AccountPin
					},
					AccountUnlock: {
						screen: AccountUnlock
					},
					About: {
						screen: About
					},
					AccountDetails: {
						screen: AccountDetails
					},
					AccountEdit: {
						screen: AccountEdit
					},
					AccountNew: {
						screen: AccountNew
					},
					AccountRecover: {
						screen: AccountRecover
					},
					AccountUnlockAndSign: {
						screen: AccountUnlockAndSign
					},
					LegacyAccountBackup: {
						screen: LegacyAccountBackup
					},
					LegacyAccountList: {
						screen: LegacyAccountList
					},
					IdentityBackup: {
						screen: IdentityBackup
					},
					IdentityManagement: {
						screen: IdentityManagement
					},
					IdentityNew: {
						screen: IdentityNew
					},
					IdentityPin: {
						screen: IdentityPin
					},
					MessageDetails: {
						screen: MessageDetails
					},
					PathDerivation: {
						screen: PathDerivation
					},
					PathDetails: {
						screen: PathDetails
					},
					PathsList: {
						screen: PathsList
					},
					PathManagement: {
						screen: PathManagement
					},
					QrScanner: {
						screen: QrScanner
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
					defaultNavigationOptions: globalStackNavigationOptions
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
