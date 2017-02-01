'use strict';

import React, { Component } from 'react'
import {
  AppRegistry,
  StyleSheet,
  Text,
  TouchableOpacity,
  Vibration,
  View
} from 'react-native'


import Camera from 'react-native-camera';

export default class Scanner extends Component {
  _onPressCancel() {
    var self = this
    requestAnimationFrame(() => {
      // cancel logic
    })
  }

  _onBarCodeRead() {
    var self = this
    Vibration.vibrate([0, 500, 200, 500])
  }

  render() {
    var self = this

    var cancelButton = null;
    this.barCodeFlag = true;

    if (this.props.cancelButtonVisible) {
      cancelButton = <CancelButton onPress={this._onPressCancel} title={this.props.cancelButtonTitle} />;
    }

    return (
      <Camera onBarCodeRead={this._onBarCodeRead} style={styles.camera}>
        <View style={styles.rectangleContainer}>
          <View style={styles.rectangle}/>
        </View>
        {cancelButton}
      </Camera>
    )
  }

}

//var QRCodeScreen = React.createClass({

  //_onPressCancel: function() {
    //var $this = this;
    //requestAnimationFrame(function() {
      //$this.props.navigator.pop();
      //if ($this.props.onCancel) {
        //$this.props.onCancel();
      //}
    //});
  //},

  //_onBarCodeRead: function(result) {
    //var $this = this;

    //if (this.barCodeFlag) {
      //this.barCodeFlag = false;

      //setTimeout(function() {
        //VibrationIOS.vibrate();
        //$this.props.navigator.pop();
        //$this.props.onSucess(result.data);
      //}, 1000);
    //}
  //},

  //render: function() {
    //var cancelButton = null;
    //this.barCodeFlag = true;

    //if (this.props.cancelButtonVisible) {
      //cancelButton = <CancelButton onPress={this._onPressCancel} title={this.props.cancelButtonTitle} />;
    //}

    //return (
      //<Camera onBarCodeRead={this._onBarCodeRead} style={styles.camera}>
        //<View style={styles.rectangleContainer}>
          //<View style={styles.rectangle}/>
        //</View>
        //{cancelButton}
      //</Camera>
    //);
  //},
//});

var CancelButton = React.createClass({
  render: function() {
    return (
      <View style={styles.cancelButton}>
        <TouchableOpacity onPress={this.props.onPress}>
          <Text style={styles.cancelButtonText}>{this.props.title}</Text>
        </TouchableOpacity>
      </View>
    );
  },
});

var styles = StyleSheet.create({
  camera: {
    flex: 1,
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

  cancelButton: {
    flexDirection: 'row',
    justifyContent: 'center',
    backgroundColor: 'white',
    borderRadius: 3,
    padding: 15,
    width: 100,
    bottom: 10,
  },
  cancelButtonText: {
    fontSize: 17,
    fontWeight: '500',
    color: '#0097CE',
  },
});

//module.exports = QRCodeScreen;
