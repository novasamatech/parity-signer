'use strict'

import React, { Component, PropTypes } from 'react'
import { TouchableHighlight, StyleSheet, View, Text } from 'react-native'
import AppStyles from '../styles'
import AccountIcon from './AccountIcon'

export default class AccountListRow extends Component {
  static propTypes = {
    upperText: PropTypes.string.isRequired,
    lowerText: PropTypes.string.isRequired,
    onPress: PropTypes.func.isRequired
  }

  render () {
    return (
      <TouchableHighlight style={styles.row} onPress={this.props.onPress} underlayColor='#0004'>
        <View style={{flexDirection: 'column'}}>
          <View style={styles.innerRow}>
            <AccountIcon style={[AppStyles.icon, styles.icon]} seed={this.props.lowerText} />
            <View style={styles.accountDetails}>
              <Text style={styles.upperText} ellipsizeMode='middle' numberOfLines={1}>{this.props.upperText}</Text>
              <Text style={styles.lowerText} ellipsizeMode='middle' numberOfLines={1}>{this.props.lowerText}</Text>
            </View>
          </View>
          <View style={{height: 1, backgroundColor: '#ccc'}} />
          <View style={{height: 1, backgroundColor: '#ddd'}} />
        </View>
      </TouchableHighlight>
    )
  }
}

const styles = StyleSheet.create({
  row: {
    backgroundColor: '#F8F8F8'
  },
  innerRow: {
    padding: 5,
    flexDirection: 'row'
  },
  accountDetails: {
    flexDirection: 'column',
    justifyContent: 'center'
  },
  icon: {
    height: 60,
    width: 60,
    marginRight: 10,
    marginBottom: 0
  },
  upperText: {
    fontSize: 16,
    color: '#888'
  },
  lowerText: {
    marginTop: 5,
    color: '#aaa',
    fontSize: 10
  }
})
