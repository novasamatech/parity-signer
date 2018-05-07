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
import { StyleSheet, AppState, Alert, SafeAreaView } from 'react-native'
import { Provider as ReduxProvider } from 'react-redux'
import { StackNavigator, TabNavigator } from 'react-navigation'
import { Actions, Scene } from 'react-native-router-flux'
import Header from './components/Header'
import TabBarBottom from './components/TabBarBottom'
import QrScanner from './screens/QrScanner'
import AccountList from './screens/AccountList'
import AccountNew from './screens/AccountNew'
import AccountDetails from './screens/AccountDetails'
import TxDetails from './screens/TxDetails'
import SignedTx from './screens/SignedTx'
import { AccountEnterPin, AccountChangePin, AccountSetPin, AccountConfirmPin } from './containers/AccountPin'
import colors from './colors'

const styles = StyleSheet.create({
  tabbar: {
    backgroundColor: '#343E48'
  },
  navibar: {
    backgroundColor: '#343E48'
  },
  navibarTitle: {
    color: 'white'
  },
  safeArea: {
    flex: 1,
    backgroundColor: colors.bg
  }
})

export default class App extends Component {
  render () {
    return (
      <UnstatedProvider>
        <ReduxProvider store={store}>
          <SafeAreaView style={styles.safeArea}>
            <Screens />
          </SafeAreaView>
        </ReduxProvider>
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

const Screens = StackNavigator ({
  Tabs: {
    screen: TabNavigator ({
      Scanner: {
        screen: StackNavigator ({
          QrScanner: {
            screen: QrScanner
          },
          TxDetails: {
            screen: TxDetails
          },
          SignedTx: {
            screen: SignedTx
          }
        })
      },
      Accounts: {
        screen: StackNavigator ({
          AccountList: {
            screen: AccountList
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
            screen: AccountDetails
          }
        },
        {
          headerMode: 'none'
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
    navigationOptions: {
      header: Header
    }
})
