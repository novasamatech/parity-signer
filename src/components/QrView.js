'use strict'

import React, { Component, PropTypes } from 'react'
import { Text, View, StyleSheet } from 'react-native'
import QRCode from 'react-native-qrcode'

export default class QrView extends Component {
  static propTypes = {
    text: PropTypes.string.isRequired,
  }

  render() {
    return (
      <View style={styles.view}>
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
  view: {
    flex: 1,
    marginTop: 60,
    marginBottom: 50,
  },
  rectangleContainer: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'transparent',
  },
})
