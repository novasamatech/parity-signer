import React, { Component } from 'react'
import { View, Text, Button, StyleSheet } from 'react-native'
import debounce from 'debounce'
import NewAccountInput from '../components/NewAccountInput'
import { words } from '../actions/random'
import { keypairFromPhrase, toAddress } from '../actions/crypto'

//const addAccount = (dispatch) => {
  //return {
    //onAddAccount: (keypair) => {

    //}
  //}
//}

export default class NewAccount extends Component {
  constructor(props) {
    super(props)

    const seed = words()

    this.state = {
      seed: seed,
      keypair: keypairFromPhrase(seed),
    }
  }

  render() {
    var self = this;
    return (
      <View style={styles.view}>
        <NewAccountInput seed={this.state.seed} onChangeText={
          debounce((text) => self.setState({keypair: keypairFromPhrase(text)}), 100)
        }/>
        <Text>0x{toAddress(this.state.keypair)}</Text>
        <Button
          onPress={() => {}}
          title="Add Account"
          color="#841584"
          accessibilityLabel="Press to add new account"
        />
      </View>
    )
  }
}

const styles = StyleSheet.create({
  view: {
    flex: 1,
    marginTop: 60,
    marginBottom: 50,
    padding: 10
  },
})
