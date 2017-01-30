/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 * @flow
 */

import React, { Component } from 'react'
import {
  AppRegistry,
  StyleSheet,
  Text,
  View
} from 'react-native'

import Tabs from 'react-native-tabs'
import Scanner from './scanner'
//import Camera from 'react-native-camera'

export default class App extends Component {
  constructor(props) {
    super(props)
    this.state = {
      page: 'send'
    };
  }

  renderView(page) {
    switch(page) {
      case 'send': return (<Text>Send</Text>)
      case 'scan': return (<Scanner></Scanner>)
      case 'accounts': return (<Text>Accounts</Text>)
      default: return (<Text>Dupa {page} D</Text>)
    }
  }

  render() {
    var self = this;
    return (
      <View style={styles.container}>
        <Tabs selected='this.state.page' style={{backgroundColor:'white'}}
          onSelect={el=>this.setState({page:el.props.name})}>
          <Text name='send'>Send TX</Text>
          <Text name='scan'>Scan QR</Text>
          <Text name='accounts'>Accounts</Text>
        </Tabs>
          {this.renderView(this.state.page)}
      </View>
    );
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#F5FCFF',
  },
  welcome: {
    fontSize: 20,
    textAlign: 'center',
    margin: 10,
  },
  instructions: {
    textAlign: 'center',
    color: '#333333',
    marginBottom: 5,
  },
});
