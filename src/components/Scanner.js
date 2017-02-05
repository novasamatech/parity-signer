'use strict';

import React, { Component, PropTypes } from 'react'
import {
  StyleSheet,
  View
} from 'react-native'
import Camera from 'react-native-camera';

export default class Scanner extends Component {
  render() {
    return (
      <Camera onBarCodeRead={this.props.onBarCodeRead} style={styles.camera}>
        <View style={styles.rectangleContainer}>
          <View style={styles.rectangle}/>
        </View>
      </Camera>
    )
  }
}

Scanner.propTypes = {
  onBarCodeRead: PropTypes.func.isRequired,
}

const styles = StyleSheet.create({
  camera: {
    flex: 1,
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
