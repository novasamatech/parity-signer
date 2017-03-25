'use strict'

import React, { Component, PropTypes } from 'react'
import { View, StyleSheet } from 'react-native'
import QRCode from 'react-native-qrcode'
import AppStyles from '../styles'

export default class QrView extends Component {
  static propTypes = {
    text: PropTypes.string.isRequired
  }

  render () {
    return (
      <View style={AppStyles.view}>
        <View style={styles.rectangleContainer}>
          <QRCode
            value={this.props.text}
            size={250}
            bgColor='black'
            fgColor='white'
          />
        </View>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  rectangleContainer: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'transparent'
  }
})
