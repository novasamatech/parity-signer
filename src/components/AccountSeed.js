// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

'use strict';

import React, { Component } from 'react';
import PropTypes from 'prop-types';
import {
  View,
  Text,
  StyleSheet,
  KeyboardAvoidingView,
  Platform,
  LayoutAnimation,
  Keyboard,
  Dimensions,
  TouchableOpacity
} from 'react-native';
import { KeyboardTrackingView } from 'react-native-keyboard-tracking-view';
import DoneButton from 'react-native-keyboard-done-button';
import { AutoGrowingTextInput } from 'react-native-autogrow-textinput';
import TextInput from './TextInput';
import colors from '../colors';

const WORD_LIST = require('../../res/wordlist.json');

export default class AccountSeed extends Component {
  constructor(...args) {
    super(...args);
    this.suggestions = this.suggestions.bind(this);
    this.handleDelete = this.handleDelete.bind(this);
    this.keyboardShow = this.keyboardShow.bind(this);
    this.keyboardHide = this.keyboardHide.bind(this);
    this.keyboardShowListener = null;
    this.keyboardHideListener = null;
  }

  state = {
    bottom: 0,
    inputTop: 0
  };

  static propTypes = {
    onChangeTags: PropTypes.func.isRequired,
    onFocus: PropTypes.func.isRequired,
    value: PropTypes.string.isRequired
  };

  componentDidMount() {
    let keyboardShowEvent = 'keyboardWillShow';
    let keyboardHideEvent = 'keyboardWillHide';

    if (Platform.OS === 'android') {
      keyboardShowEvent = 'keyboardDidShow';
      keyboardHideEvent = 'keyboardDidHide';
    }

    this.keyboardShowListener = Keyboard.addListener(
      keyboardShowEvent,
      this.keyboardShow
    );
    this.keyboardHideListener = Keyboard.addListener(
      keyboardHideEvent,
      this.keyboardHide
    );
  }

  componentWillUnmount() {
    this.keyboardShowListener.remove();
    this.keyboardHideListener.remove();
  }

  keyboardShow(e) {
    console.log(e.endCoordinates.height);
    LayoutAnimation.easeInEaseOut();
    this.setState({
      bottom: e.endCoordinates.height
    });
  }

  keyboardHide(e) {
    LayoutAnimation.easeInEaseOut();
    this.setState({
      bottom: 0
    });
  }

  suggestions() {
    return WORD_LIST;
  }

  handleDelete() {}

  render() {
    const { onChangeTags } = this.props;
    const { height: screenHeight } = Dimensions.get('window');
    return (
      <View
        style={styles.body}
        ref="Input"
        onLayout={e => {
          this.refs.Input.measure((_, y, _a, _b, _c, top) => {
            console.log(screenHeight - top);
            this.setState({ inputTop: screenHeight - top });
          });
        }}
      >
        <AutoGrowingTextInput
          onChangeText={seed => {
            onChangeTags(seed);
          }}
          autoCapitalize={'none'}
          autoCorrect={false}
          editable={true}
          multiline={true}
          {...this.props}
          style={[styles.input, this.props.style]}
        />
      </View>
    );
  }
}

const styles = StyleSheet.create({
  input: {
    fontSize: 20,
    height: 40,
    justifyContent: 'center',
    alignItems: 'center',
    elevation: 4,
    padding: 10,
    backgroundColor: colors.card_bg
  },
  body: {
    backgroundColor: colors.card_bg
  },
  tags: {},
  tag: {}
});
