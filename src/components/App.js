import React, { Component } from 'react'
import { View, Text, StyleSheet } from 'react-native'
import { Provider, connect } from 'react-redux'
import { createStore } from 'redux'
import { Actions, ActionConst, Router, Scene } from 'react-native-router-flux'
import reducers from '../reducers'
import TabIcon from './TabIcon'
import QrScanner from '../containers/QrScanner'
import Signer from '../containers/Signer'
import Accounts from '../containers/Accounts'
import SelectAccount from '../containers/SelectAccount'
import NewAccount from '../containers/NewAccount'
import Send from '../components/Send'
import Account from '../containers/Account'
import ConfirmTransaction from '../containers/ConfirmTransaction'
import { EnterPin, SetPin, ConfirmPin } from '../containers/Pin'

const ConnectedRouter = connect()(Router)
const store = createStore(reducers)

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
        <Scene key='scan' component={QrScanner} title='Scan QR' rightTitle="Scanned" onRight={() => Actions.confirm()} rightButtonTextStyle={styles.navibarTitle}/>
        <Scene key='confirm' component={ConfirmTransaction} title='Sign Tx'/>
        <Scene key='select' title='Select Account' component={SelectAccount}/>
        <Scene key='enterPin' title='Enter Pin' component={EnterPin}/>
      </Scene>
      <Scene key='right' title='Accounts' icon={TabIcon} navigationBarStyle={styles.navibar} titleStyle={styles.navibarTitle}>
        <Scene key='accounts' title='Accounts' component={Accounts} rightTitle="Add" onRight={() => Actions.add()}
          rightButtonTextStyle={styles.navibarTitle}/>
        <Scene key='add' component={NewAccount} title='Add Account'/>
        <Scene key='setPin' title='Set Pin' component={SetPin}/>
        <Scene key='confirmPin' title='Confirm Pin' component={ConfirmPin}/>
        <Scene key='details' component={Account} title='Account Details'/>
        <Scene key='send' component={Send} title='Send TX'/>
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
}

