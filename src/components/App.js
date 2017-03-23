'use strict'

import React, { Component } from 'react'
import { View, Text, StyleSheet, AppState } from 'react-native'
import { Provider, connect } from 'react-redux'
import { createStore } from 'redux'
import { Actions, ActionConst, Router, Scene } from 'react-native-router-flux'
import TabIcon from './TabIcon'
import QrScanner from '../containers/QrScanner'
import AccountList from '../containers/AccountList'
import AccountNew from '../containers/AccountNew'
import AccountDetails from '../containers/AccountDetails'
import TxDetails from '../containers/TxDetails'
import { AccountEnterPin, AccountSetPin, AccountConfirmPin } from '../containers/AccountPin'
import { QrViewTransaction, QrViewAddress } from '../containers/QrView'
import { loadAccounts, saveAccounts } from '../util/db'
import { setAccounts } from '../actions/accounts'
import store from '../util/store'

const ConnectedRouter = connect()(Router)

loadAccounts().then(accounts => {
  store.dispatch(setAccounts(accounts))
})

const styles = StyleSheet.create({
  tabbar: {
    backgroundColor: '#343E48',
  },
  navibar: {
    backgroundColor: '#343E48',
  },
  navibarTitle: {
    color: 'white'
  }
})

const scenes = Actions.create(
  <Scene key='root'>
    <Scene key='tabs' tabs style={styles.tabbar}>
      <Scene key='left' title='Scan QR' initial icon={TabIcon} navigationBarStyle={styles.navibar} titleStyle={styles.navibarTitle}>
        <Scene key='qrScan'
          component={QrScanner}
          title='Scan QR'
        />
        <Scene key='txDetails' component={TxDetails} title='Transaction Details'
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage={true}
        />
        <Scene key='accountEnterPin' title='Enter Pin' component={AccountEnterPin}
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage={true}
        />
        <Scene key='qrViewTx' title='QR Code' component={QrViewTransaction} rightTitle='Done'
          onRight={() => Actions.popTo('left')}
          rightButtonTextStyle={styles.navibarTitle}
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage={true}
        />
      </Scene>
      <Scene key='right' title='Accounts' icon={TabIcon} navigationBarStyle={styles.navibar} titleStyle={styles.navibarTitle}>
        <Scene key='accountList' title='Accounts' component={AccountList}
          rightTitle="Add" onRight={() => Actions.accountNew()} rightButtonTextStyle={styles.navibarTitle}/>
        <Scene key='accountNew' component={AccountNew} title='Add Account'
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage={true}
        />
        <Scene key='accountSetPin' title='Set Pin' component={AccountSetPin}
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage={true}
        />
        <Scene key='accountConfirmPin' title='Confirm Pin' component={AccountConfirmPin}
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage={true}
        />
        <Scene key='accountDetails' component={AccountDetails} title='Account Details'
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage={true}
        />
        <Scene key='qrViewAddress' title='QR Code' component={QrViewAddress}
          backTitle='Back'
          backButtonTextStyle={styles.navibarTitle}
          hideBackImage={true}
        />
      </Scene>
    </Scene>
  </Scene>
)

export default class App extends Component {
  constructor(props) {
    super(props)
  }

  render() {
    return (
      <Provider store={store}>
        <ConnectedRouter scenes={scenes}/>
      </Provider>
    )
  }

  componentDidMount() {
    AppState.addEventListener('change', this._handleAppStateChange);
  }

  componentWillUnmount() {
    AppState.removeEventListener('change', this._handleAppStateChange);
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

