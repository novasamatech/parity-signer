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

'use strict'

import React, { Component } from 'react'
import { Provider as UnstatedProvider, Subscribe, Container } from 'unstated'
import { View, Text, Image, StyleSheet, AppState, Alert } from 'react-native'
import { createStackNavigator, createTabNavigator, HeaderTitle, Header } from 'react-navigation'
import { Actions, Scene } from 'react-native-router-flux'
import Icon from 'react-native-vector-icons/MaterialIcons';
import { default as HomeHeader } from './components/Header'
import TabBarBottom from './components/TabBarBottom'
import TouchableItem from './components/TouchableItem'
import QrScanner from './screens/QrScanner'
import AccountList from './screens/AccountList'
import AccountNew from './screens/AccountNew'
import AccountDetails from './screens/AccountDetails'
import TxDetails from './screens/TxDetails'
import SignedTx from './screens/SignedTx'
import { AccountEnterPin, AccountChangePin, AccountSetPin, AccountConfirmPin } from './containers/AccountPin'
import colors from './colors'

console.log(Icon)

export default class App extends Component {
  render () {
    return (
      <UnstatedProvider>
          <Screens />
      </UnstatedProvider>
    )
  }

  componentDidMount () {
    AppState.addEventListener('change', this._handleAppStateChange)
  }

  componentWillUnmount () {
    AppState.removeEventListener('change', this._handleAppStateChange)
  }

  _handleAppStateChange = (appState) => {
    // TODO: handle
  }
}

const headerStyles = StyleSheet.create({
  headerStyle: {
    backgroundColor: colors.bg,
    height: 60,
    flexDirection: 'row',
    alignItems: 'center',
    padding: 14,
    borderBottomWidth: 0.5,
    borderBottomColor: colors.bg_text_sec,

  },
  logo: {
    width: 42,
    height: 42
  },
  headerTextLeft: {
    flex: 1,
    paddingLeft: 10,
    fontSize: 25,
    fontFamily: 'Roboto',
    color: colors.bg_text
  },
  headerSecureIcon: {
    marginLeft: 0,
    fontSize: 20,
    fontWeight: 'bold',
    paddingRight: 5,
    color: colors.bg_text_positive
  },
  headerTextRight: {
    marginLeft: 0,
    fontSize: 17,
    fontFamily: 'Roboto',
    fontWeight: 'bold',
    color: colors.bg_text_positive
  },
});

class HeaderLeftHome extends Component {
  render() {

    return <View
      style={{ flexDirection: 'row', alignItems: 'center' }}
      accessibilityComponentType="button"
      accessibilityTraits="button"
      testID="header-back"
      delayPressIn={0}
      onPress={ () => this.props.onPress && this.props.onPress() } >
      <Image source={require('../icon.png')} style={headerStyles.logo} />
      <Text style={headerStyles.headerTextLeft}>parity</Text>
    </View>
  }
}

const globalStackNavigationOptions = {
  headerTintColor: colors.card_bg,
  headerRight: (
    <View style={ { flexDirection: 'row' } }>
      <Icon style={headerStyles.headerSecureIcon} name='security' />
      <Text style={headerStyles.headerTextRight}>Secured</Text>
    </View>),
  headerStyle: {
    backgroundColor: colors.bg,
    height: 60,
    paddingTop: 0,
    paddingBottom: 0,
    padding: 14,
    borderBottomWidth: 0.5,
    borderBottomColor: colors.bg_text_sec,
  },
  headerTitleStyle: {
    display: 'none'
  },
  headerBackTitleStyle: {
    paddingLeft: 5,
    fontSize: 18
 }
}

const Screens = createStackNavigator ({
  Tabs: {
    screen: createTabNavigator ({
      Scanner: {
        screen: createStackNavigator ({
          QrScanner: {
            screen: QrScanner,
            navigationOptions: {
              headerLeft: <HeaderLeftHome />
            }
          },
          TxDetails: {
            screen: TxDetails
          },
          SignedTx: {
            screen: SignedTx
          }
        },
        {
          navigationOptions: globalStackNavigationOptions
        })
      },
      Accounts: {
        screen: createStackNavigator ({
          AccountList: {
            screen: AccountList,
            navigationOptions: {
              headerLeft: <HeaderLeftHome />
            }
          },
          AccountNew: {
            screen: AccountNew
          },
          AccountSetPin: {
            screen: AccountSetPin
          },
          AccountConfirmPin: {
            screen: AccountConfirmPin
          },
          AccountDetails: {
            screen: AccountDetails,
          }
        },
        {
          navigationOptions: globalStackNavigationOptions
        }
      )
    }
  }, {
    tabBarComponent: props => <TabBarBottom { ...props } />,
    tabBarPosition: 'bottom',
  })
  }
},
{
    headerMode: 'none',
})
