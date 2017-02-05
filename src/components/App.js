import React, { Component } from 'react'
import { View, Text } from 'react-native'
import { Provider, connect } from 'react-redux'
import { createStore } from 'redux'
import { Actions, ActionConst, Router, Scene } from 'react-native-router-flux'
import reducers from '../reducers'
import TabIcon from './TabIcon'
import QrScanner from '../containers/QrScanner'

const ConnectedRouter = connect()(Router)
const store = createStore(reducers)

const scenes = Actions.create(
  <Scene key='root'>
    <Scene key='tabs' tabs>
      <Scene key='send' component={View} title='Send TX' icon={TabIcon}/>
      <Scene key='scan' component={QrScanner} title='Scan QR' icon={TabIcon} initial={true}/>
      <Scene key='accounts' component={View} title='Accounts' icon={TabIcon}/>
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

