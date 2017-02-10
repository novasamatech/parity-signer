'use strict';

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View } from 'react-native'
import Camera from 'react-native-camera';

export default class Scanner extends Component {
  static propTypes = {
    onBarCodeRead: PropTypes.func.isRequired,
  }

  render() {
    return (
      <Camera onBarCodeRead={this.props.onBarCodeRead} style={styles.view}>
        <View style={styles.rectangleContainer}>
          <View style={styles.rectangle}/>
        </View>
      </Camera>
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

  rectangle: {
    height: 250,
    width: 250,
    borderWidth: 2,
    borderColor: '#00FFFF',
    backgroundColor: 'transparent',
  },
});
