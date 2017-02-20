'use strict'

import React, { Component, PropTypes } from 'react'
import { TouchableHighlight, StyleSheet, View, Text } from 'react-native'

export default class AccountsListRow extends Component {
  static propTypes = {
    upperText: PropTypes.string.isRequired,
    lowerText: PropTypes.string.isRequired,
    onPress: PropTypes.func.isRequired,
  }

  render() {
    return (
      <TouchableHighlight style={styles.row} onPress={this.props.onPress} underlayColor='#0004'>
        <View style={{flexDirection: 'column'}}>
          <View style={{flexDirection: 'row'}}>
            <View style={styles.square}/>
            <View style={{flexDirection: 'column'}}>
              <Text style={styles.upperText} ellipsizeMode="middle" numberOfLines={1}>{this.props.upperText}</Text>
              <Text style={styles.lowerText} ellipsizeMode="middle" numberOfLines={1}>{this.props.lowerText}</Text>
            </View>
          </View>
          <View style={{height: 1, backgroundColor: '#ccc'}}/>
          <View style={{height: 1, backgroundColor: '#ddd'}}/>
        </View>
      </TouchableHighlight>
    )
  }
}

const styles = StyleSheet.create({
  row: {
    backgroundColor: '#F8F8F8',
  },
  square: {
    height: 60,
    width: 60,
    backgroundColor: '#D8D8D8',
    marginRight: 10,
  },
  upperText: {
    marginTop: 20,
    width: 200,
    fontSize: 16,
    color: '#888',
  },
  lowerText: {
    marginTop: 5,
    color: '#aaa',
    fontSize: 10,
  }
})
