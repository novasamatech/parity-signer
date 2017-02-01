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
import Scanner from './src/scanner'
import TabBarItem from './src/tabbaritem'

export default class App extends Component {
  constructor(props) {
    super(props)
    this.state = {
      page: 'scan'
    };
  }

  renderView(page) {
    switch(page) {
      case 'send': return (<Text>Send</Text>)
      case 'scan': return (<Scanner></Scanner>)
      case 'accounts': return (<Text>Accounts</Text>)
    }
  }

  render() {
    var self = this
    return (
      <View style={styles.container}>
        <Tabs selected='this.state.page' style={{backgroundColor:'white'}}
          onSelect={el=>this.setState({page:el.props.name})} selectedStyle={{color:'red'}}>
          <TabBarItem name='send' text='Send TX' selectedItem={self.state.page}/>
          <TabBarItem name='scan' text='Scan QR' selectedItem={self.state.page}/>
          <TabBarItem name='accounts' text='Accounts' selectedItem={self.state.page}/>
        </Tabs>
        <View style={styles.scanner}>
          {this.renderView(this.state.page)}
        </View>
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
  scanner: {
    flex: 1,
    flexDirection: 'row',
    marginTop: 20,
    marginBottom: 50
  },
});
