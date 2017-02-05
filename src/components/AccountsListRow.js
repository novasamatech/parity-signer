import React, { Component, PropTypes } from 'react'
import { TouchableHighlight, StyleSheet, View, Text } from 'react-native'

export default class AccountsListRow extends Component {
  static propTypes = {
    text: PropTypes.string.isRequired
  }

  render() {
    return (
      <TouchableHighlight>
        <View>
          <View style={styles.row}>
            <Text style={styles.text}>{this.props.text}</Text>
          </View>
        </View>
      </TouchableHighlight>
    )
  }
}

const styles = StyleSheet.create({
  row: {
    flexDirection: 'row',
    justifyContent: 'center',
    padding: 10,
    backgroundColor: '#F6F6F6',
  },
  text: {
    flex: 1,
  }
})
