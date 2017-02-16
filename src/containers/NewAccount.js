import React, { Component } from 'react'
import { View, Text, TextInput, Button, StyleSheet } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import debounce from 'debounce'
import NewAccountInput from '../components/NewAccountInput'
import { words } from '../actions/random'
import { keypairFromPhrase, toAddress } from '../actions/crypto'
import { addAccount } from '../actions/accounts'

const mapDispatchToProps = (dispatch) => {
  return {
    addAccount: (account) => {
      const address = toAddress(account.keypair)
      dispatch(addAccount({
        address: address,
        name: account.name,
      }))
      Actions.pop()
    }
  }
}

export class NewAccount extends Component {
  constructor(props) {
    super(props)

    const seed = words()

    this.state = {
      seed: seed,
      keypair: keypairFromPhrase(seed),
      name: '',
    }
  }

  render() {
    var self = this;
    return (
      <View style={styles.view}>
        <Text style={styles.hint}>name</Text>
        <TextInput
          placeholder='My Account'
          value={this.state.name}
          style={styles.input}
          editable={true}
          multiline={false}
          returnKeyType='next'
          numberOfLines={1}
          fontSize={16}
          autoFocus={true}
          onChangeText={(text) => {this.setState({name: text})}}
        />
        <Text style={styles.hint}>brain wallet seed</Text>
        <NewAccountInput seed={this.state.seed} onChangeText={
          debounce((text) => self.setState({keypair: keypairFromPhrase(text)}), 100)
        }/>
        <Text style={styles.hint} adjustsFontSizeToFit={true}>0x{toAddress(this.state.keypair)}</Text>
        <Button
          onPress={() => this.props.addAccount({
            keypair: this.state.keypair,
            name: this.state.name
          })}
          title="Add Account"
          color="green"
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
    padding: 20
  },
  hint: {
    marginBottom: 20,
  },
  input: {
    height: 20,
    marginBottom: 20,
  }
})

export default connect(
  undefined,
  mapDispatchToProps
)(NewAccount)

