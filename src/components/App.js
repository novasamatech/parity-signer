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
import NewAccount from '../containers/NewAccount'

const ConnectedRouter = connect()(Router)
const store = createStore(reducers)

const scenes = Actions.create(
  <Scene key='root'>
    <Scene key='tabs' tabs>
      <Scene key='send' component={View} title='Send TX' icon={TabIcon}/>
      <Scene key='mid' title='Scan QR' initial icon={TabIcon}>
        <Scene key='scan' component={QrScanner} title='Scan QR'/>
        <Scene key='signer' component={Signer} title='Sign Tx'/>
      </Scene>
      <Scene key='accounts' title='Accounts' icon={TabIcon}>
        <Scene key='accountsList' title='Accounts' component={Accounts} rightTitle="Add" onRight={() => Actions.add()}/>
        <Scene key='add' component={NewAccount} title='Add Account'/>
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
