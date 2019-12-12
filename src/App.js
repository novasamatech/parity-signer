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

import React, { Component, PureComponent } from 'react';
import { Platform, StatusBar, View, YellowBox } from 'react-native';
import {
	createAppContainer,
	createStackNavigator,
	createSwitchNavigator,
	HeaderBackButton,
	withNavigation
} from 'react-navigation';
import NavigationBar from 'react-native-navbar-color';
import { createMaterialBottomTabNavigator } from 'react-navigation-material-bottom-tabs';
import { Provider as UnstatedProvider } from 'unstated';
import { MenuProvider } from 'react-native-popup-menu';

import '../shim';
import Background from './components/Background';
import colors from './colors';
import Icon from 'react-native-vector-icons/Ionicons';
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
import LegacyNetworkChooser from './screens/LegacyNetworkChooser';
import testIDs from '../e2e/testIDs';

const getLaunchArgs = props => {
	if (Platform.OS === 'ios') {
		if (props.launchArgs && props.launchArgs.includes('-detoxServer')) {
			return (global.inTest = true);
		}
	} else {
		if (props.launchArgs && props.launchArgs.hasOwnProperty('detoxServer')) {
			return (global.inTest = true);
		}
	}
	global.inTest = false;
};

export default class App extends Component {
	constructor(props) {
		super();
		getLaunchArgs(props);
		if (__DEV__) {
			YellowBox.ignoreWarnings([
				'Warning: componentWillReceiveProps',
				'Warning: componentWillMount',
				'Warning: componentWillUpdate'
			]);
		}
	}
	componentDidMount() {
		NavigationBar.setColor('black');
	}

	render() {
		return (
			<UnstatedProvider>
				<MenuProvider backHandler={true}>
					<StatusBar barStyle="light-content" backgroundColor={colors.bg} />
					<Background />
					<ScreensContainer />
				</MenuProvider>
			</UnstatedProvider>
		);
	}
}

const globalStackNavigationOptions = ({ navigation }) => {
	const isFirstScreen = navigation.dangerouslyGetParent().state.index === 0;
	return {
		headerBackTitleStyle: {
			color: colors.bg_text_sec
		},
		headerLeft: isFirstScreen ? <HeaderLeftHome /> : <HeaderLeftWithBack />,
		headerRight: <SecurityHeader />,
		headerStyle: {
			backgroundColor: colors.bg,
			borderBottomColor: colors.bg,
			borderBottomWidth: 0,
			elevation: 0,
			height: 60,
			paddingBottom: 0,
			paddingTop: 0
		},
		headerTintColor: colors.bg_text_sec,
		headerTitleStyle: {
			display: 'none'
		}
	};
};

const HeaderLeftWithBack = withNavigation(
	class _HeaderBackButton extends PureComponent {
		render() {
			const { navigation } = this.props;
			return (
				<View
					style={{ flexDirection: 'row' }}
					testID={testIDs.Header.headerBackButton}
				>
					<HeaderBackButton
						{...this.props}
						accessibilityComponentType="button"
						accessibilityTraits="button"
						delayPressIn={0}
						titleStyle={globalStackNavigationOptions.headerBackTitleStyle}
						title="Back"
						tintColor={colors.bg_text}
						onPress={() => navigation.goBack()}
					/>
				</View>
			);
		}
	}
);

/* eslint-disable sort-keys */
const tocAndPrivacyPolicyScreens = {
	TermsAndConditions: {
		navigationOptions: {
			headerRight: null
		},
		screen: TermsAndConditions
	},
	PrivacyPolicy: {
		navigationOptions: {
			headerRight: null
		},
		screen: PrivacyPolicy
	}
};

const Screens = createSwitchNavigator(
	{
		Loading: {
			screen: Loading
		},
		TocAndPrivacyPolicy: createStackNavigator(tocAndPrivacyPolicyScreens, {
			defaultNavigationOptions: globalStackNavigationOptions
		}),
		Welcome: {
			screen: createMaterialBottomTabNavigator(
				{
					Accounts: {
						screen: createStackNavigator(
							{
								AccountNetworkChooser: {
									screen: AccountNetworkChooser
								},
								AccountPin: {
									screen: AccountPin
								},
								AccountUnlock: {
									screen: AccountUnlock
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
								LegacyNetworkChooser: {
									screen: LegacyNetworkChooser
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
								}
							},
							{
								defaultNavigationOptions: globalStackNavigationOptions
							}
						),
						navigationOptions: {
							tabBarLabel: 'Accounts',
							tabBarIcon: ({ tintColor }) => (
								<Icon
									style={[{ color: tintColor }]}
									size={25}
									name={'ios-key'}
								/>
							)
						}
					},
					Profile: {
						screen: createStackNavigator(
							{
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
						),
						navigationOptions: {
							tabBarLabel: 'Scanner',
							tabBarIcon: ({ tintColor }) => (
								<Icon
									style={[{ color: tintColor }]}
									size={25}
									name={'ios-qr-scanner'}
								/>
							)
						}
					},
					Settings: {
						screen: createStackNavigator(
							{
								About: {
									screen: About
								},
								Security: {
									navigationOptions: {
										headerRight: null
									},
									screen: Security
								},
								...tocAndPrivacyPolicyScreens
							},
							{
								defaultNavigationOptions: globalStackNavigationOptions
							}
						),
						navigationOptions: {
							tabBarLabel: 'Settings',
							tabBarIcon: ({ tintColor }) => (
								<Icon
									style={[{ color: tintColor }]}
									size={25}
									name={'ios-settings'}
								/>
							)
						}
					}
				},
				{
					activeColor: colors.bg_button,
					barStyle: { backgroundColor: '#000000' },
					inactiveColor: colors.card_bg_text_sec,
					initialRouteName: 'Accounts',
					shifting: true
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
