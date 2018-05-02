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
import { StyleSheet, AppState, Alert } from 'react-native'
import { Provider, connect } from 'react-redux'
import { Actions, Router, Scene } from 'react-native-router-flux'
import TabIcon from './TabIcon'
import IconChooser from '../containers/IconChooser'
import QrScanner from '../containers/QrScanner'
import AccountList from '../containers/AccountList'
import AccountNew from '../containers/AccountNew'
import AccountDetails from '../containers/AccountDetails'
import TxDetails from '../containers/TxDetails'
import { AccountEnterPin, AccountChangePin, AccountSetPin, AccountConfirmPin } from '../containers/AccountPin'
import { QrViewTransaction } from '../containers/QrView'
import DataDetails from '../containers/DataDetails'
import { loadAccounts, saveAccounts } from '../util/db'
import { setAccounts } from '../actions/accounts'
import store from '../util/store'

const ConnectedRouter = connect()(Router)

loadAccounts().then(accounts => {
  store.dispatch(setAccounts(accounts))
})

const styles = StyleSheet.create({
  tabbar: {
    backgroundColor: '#343E48'
  },
  navibar: {
    backgroundColor: '#343E48'
  },
  navibarTitle: {
    color: 'white'
  }
})

const accountScenes =
  <Scene>
    <Scene key='accountList' title='Accounts' component={AccountList}
      onRight={() => Actions.iconChooser()}
      rightTitle='Add'
      rightButtonTextStyle={styles.navibarTitle}
    />
    <Scene key='accountNew' component={AccountNew} title='New Account' hideTabBar
      onBack={() => Alert.alert('Do you want to cancel account creation?', undefined, [{
        text: 'Yes',
        onPress: () => {
          Actions.popTo('_right')
        }
      }, {
        text: 'No',
        onPress: () => {}
      }])}
      backTitle='Back'
      backButtonTextStyle={styles.navibarTitle}
      hideBackImage
    />
    <Scene key='accountChangePin' title='Current PIN' component={AccountChangePin}
      backTitle='Back'
      backButtonTextStyle={styles.navibarTitle}
      hideBackImage
    />
    <Scene key='accountSetPin' title='Set Account PIN' component={AccountSetPin}
      backTitle='Back'
      backButtonTextStyle={styles.navibarTitle}
      hideBackImage
    />
    <Scene key='accountConfirmPin' title='Confirm PIN' component={AccountConfirmPin}
      backTitle='Back'
      backButtonTextStyle={styles.navibarTitle}
      hideBackImage
    />
    <Scene key='accountDetails' title='Account Details' component={AccountDetails}
      backTitle='Back'
      backButtonTextStyle={styles.navibarTitle}
      hideBackImage
    />
    {/* <Scene key='qrViewAddress' title='Address QR' component={QrViewAddress}
      backTitle='Back'
      backButtonTextStyle={styles.navibarTitle}
      hideBackImage
    /> */}
    <Scene key='iconChooser' title='Choose an Icon' component={IconChooser}
      backTitle='Back'
      backButtonTextStyle={styles.navibarTitle}
      hideBackImage
    />
  </Scene>

const scenes =
  <Scene key='root'>
    <Scene key='tabs' tabs style={styles.tabbar}>
      <Scene key='_left' title='Scan Transaction QR' initial icon={TabIcon} navigationBarStyle={styles.navibar} titleStyle={styles.navibarTitle}>
        <Scene key='qrScan'
          component={QrScanner}
          title='Scan Transaction QR'
        />
        <Scene key='txDetails' component={TxDetails} title='Transaction Details'
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage
        />
        <Scene key='dataDetails' component={DataDetails} title='Sign Data'
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage
        />
        <Scene key='accountEnterPin' title='Enter PIN' component={AccountEnterPin}
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage
        />
        <Scene key='qrViewTx' title='Signature QR' component={QrViewTransaction}
          onRight={() => Actions.popTo('_left')}
          rightTitle='Done'
          rightButtonTextStyle={styles.navibarTitle}
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage
        />
      </Scene>
      <Scene key='_right' title='Accounts' icon={TabIcon} navigationBarStyle={styles.navibar} titleStyle={styles.navibarTitle}>
        {accountScenes.props.children}
      </Scene>
    </Scene>
  </Scene>

export default class App extends Component {
  render () {
    let actions = Actions.create(scenes)
    return (
      <Provider store={store}>
        <ConnectedRouter scenes={actions} />
      </Provider>
    )
  }

  componentDidMount () {
    AppState.addEventListener('change', this._handleAppStateChange)
  }

  componentWillUnmount () {
    AppState.removeEventListener('change', this._handleAppStateChange)
  }

  _handleAppStateChange = (appState) => {
    switch (appState) {
      case 'inactive':
        saveAccounts(store.getState().accounts.all)
        break
      case 'background':
        break
      case 'active':
        break
    }
  }
}
