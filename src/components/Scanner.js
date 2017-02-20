'use strict';

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, StatusBar } from 'react-native'
import Camera from 'react-native-camera';
import AppStyles from '../styles'

export default class Scanner extends Component {
  static propTypes = {
    onBarCodeRead: PropTypes.func.isRequired,
  }

  render() {
    return (
      <Camera onBarCodeRead={this.props.onBarCodeRead} style={AppStyles.view}>
        <StatusBar barStyle='light-content'/>
        <View style={styles.rectangleContainer}>
          <View style={styles.rectangle}>
            <View style={styles.innerRectangle}/>
          </View>
        </View>
      </Camera>
    )
  }
}

const styles = StyleSheet.create({
  rectangleContainer: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'transparent',
  },

  rectangle: {
    borderWidth: 2,
    borderRadius: 25,
    alignItems: 'center',
    justifyContent: 'center',
    height: 250,
    width: 250,
    borderColor: '#ccc',
    backgroundColor: 'transparent',
  },

  innerRectangle: {
    height: 248,
    width: 248,
    borderWidth: 2,
    borderRadius: 25,
    borderColor: '#ddd',
    backgroundColor: 'transparent',
  },
});
