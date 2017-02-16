import React, { Component, PropTypes } from 'react'
import { TouchableHighlight, StyleSheet, View, Text } from 'react-native'

export default class AccountsListRow extends Component {
  static propTypes = {
    text: PropTypes.string.isRequired
  }

  render() {
    return (
      <TouchableHighlight style={styles.row}>
        <View style={{flexDirection: 'row'}}>
          <View style={styles.square}/>
          <Text style={styles.text} fontSize={16} ellipsizeMode="middle" numberOfLines={1}>0x{this.props.text}</Text>
        </View>
      </TouchableHighlight>
    )
  }
}

const styles = StyleSheet.create({
  row: {
    flexDirection: 'row',
    backgroundColor: '#F8F8F8',
    borderBottomWidth: 1,
    borderColor: '#F0F0F0',
  },
  square: {
    height: 60,
    width: 60,
    backgroundColor: '#D8D8D8',
    marginRight: 10,
  },
  text: {
    marginTop: 20,
    width: 200,
    //flex: 1,
  }
})
