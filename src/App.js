/* eslint-disable */

// Copyright 2015-2017 Parity Technologies (UK) Ltd.
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
import React, { Component } from 'react';
import { StatusBar } from 'react-native';
import {
  createAppContainer,
  createStackNavigator,
  HeaderBackButton,
  withNavigation
} from 'react-navigation';
import { Provider as UnstatedProvider } from 'unstated';
import { MenuProvider } from 'react-native-popup-menu';

import '../ReactotronConfig';
import colors from './colors';
import Background from './components/Background';
import HeaderLeftHome from './components/HeaderLeftHome';
import SecurityHeader from './components/SecurityHeader';
import About from './screens/About';
import AccountBackup from './screens/AccountBackup';
import AccountDetails from './screens/AccountDetails';
import AccountEdit from './screens/AccountEdit';
import AccountList from './screens/AccountList';
import AccountNetworkChooser from './screens/AccountNetworkChooser';
import AccountNew from './screens/AccountNew';
import AccountPin from './screens/AccountPin';
import AccountRecover from './screens/AccountRecover';
import { AccountUnlock, AccountUnlockAndSign } from './screens/AccountUnlock';
import Loading from './screens/Loading';
import MessageDetails from './screens/MessageDetails';
import PrivacyPolicy from './screens/PrivacyPolicy';
import QrScanner from './screens/QrScanner';
import Security from './screens/Security';
import SignedMessage from './screens/SignedMessage';
import SignedTx from './screens/SignedTx';
import TermsAndConditions from './screens/TermsAndConditions';
import TxDetails from './screens/TxDetails';

export default class App extends Component {
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
  headerTintColor: colors.card_bg,
  headerRight: <SecurityHeader />,
  headerStyle: {
    backgroundColor: colors.bg,
    height: 60,
    paddingTop: 0,
    paddingBottom: 0,
    padding: 14,
    borderBottomWidth: 0.5,
    borderBottomColor: colors.bg_text_sec
  },
  headerTitleStyle: {
    display: 'none'
  },
  headerBackTitleStyle: {
    fontSize: 20,
    fontWeight: '500',
    fontFamily: 'Manifold CF'
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

const Screens = createStackNavigator(
  {
    Loading: {
      screen: Loading
    },
    Security: {
      screen: createStackNavigator(
        {
          Security: {
            screen: Security,
            navigationOptions: {
              headerTintColor: colors.card_bg,
              headerLeft: <SecurityHeaderBackButton />,
              headerRight: null
            }
          }
        },
        {
          headerMode: 'screen',
          navigationOptions: globalStackNavigationOptions
        }
      )
    },
    TocAndPrivacyPolicy: {
      screen: createStackNavigator(
        {
          TermsAndConditions: {
            screen: TermsAndConditions,
            navigationOptions: {
              headerLeft: <HeaderLeftHome />
            }
          },
          PrivacyPolicy: {
            screen: PrivacyPolicy
          }
        },
        {
          navigationOptions: globalStackNavigationOptions,
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
            screen: AccountList,
            navigationOptions: {
              headerLeft: <HeaderLeftHome />
            }
          },
          AccountNetworkChooser: {
            screen: AccountNetworkChooser
          },
          AccountNew: {
            screen: AccountNew
          },
          AccountRecover: {
            screen: AccountRecover
          },
          AccountBackup: {
            screen: AccountBackup
          },
          AccountPin: {
            screen: AccountPin
          },
          QrScanner: {
            screen: QrScanner,
          },
          TxDetails: {
            screen: TxDetails
          },
          AccountUnlockAndSign: {
            screen: AccountUnlockAndSign
          },
          SignedTx: {
            screen: SignedTx
          },
          SignedMessage: {
            screen: SignedMessage
          },
          MessageDetails: {
            screen: MessageDetails
          },
          About: {
            screen: About
          },
          AccountDetails: {
            screen: AccountDetails
          },
          AccountUnlock: {
            screen: AccountUnlock
          },
          AccountEdit: {
            screen: AccountEdit
          }
        },
        {
          navigationOptions: globalStackNavigationOptions,
          initialRouteParams: {
            isWelcome: true
          }
        }
      )
    },
  },
  {
    headerMode: 'none',
    mode: 'card'
  }
);

const ScreensContainer = createAppContainer(Screens);
