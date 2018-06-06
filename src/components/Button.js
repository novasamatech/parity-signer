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

// @flow

import React from 'react';
import PropTypes from 'prop-types';
import {
  View,
  Text,
  Platform,
  StyleSheet,
  TouchableNativeFeedback,
  TouchableOpacity
} from 'react-native';
import colors from '../colors';

export default class Button extends React.Component<{
  title: string,
  onPress: () => any,
  textStyles?: ?StyleSheet.Styles,
  buttonStyles?: ?StyleSheet.Styles,
  disabled?: ?boolean
}> {
  static propTypes = {
    title: PropTypes.string.isRequired,
    textStyles: Text.propTypes.style,
    buttonStyles: View.propTypes.style,
    disabled: PropTypes.bool,
    onPress: PropTypes.func.isRequired
  };

  render() {
    const { onPress, title, disabled, textStyles, buttonStyles } = this.props;

    const finalTextStyles = [styles.text, textStyles];
    const finalButtonStyles = [styles.button, buttonStyles];

    if (disabled) {
      finalTextStyles.push(styles.textDisabled);
      finalButtonStyles.push(styles.buttonDisabled);
    }

    const Touchable =
      Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;
    return (
      <Touchable
        accessibilityComponentType="button"
        disabled={disabled}
        onPress={onPress}
      >
        <View style={finalButtonStyles}>
          <Text style={finalTextStyles} disabled={disabled}>
            {title}
          </Text>
        </View>
      </Touchable>
    );
  }
}

const styles = StyleSheet.create({
  button: {
    justifyContent: 'center',
    alignItems: 'center',
    elevation: 4,
    backgroundColor: colors.bg_text_sec,
    height: 60
  },
  text: {
    fontFamily: 'Manifold CF',
    color: 'white',
    padding: 8,
    fontWeight: 'bold',
    fontSize: 20
  },
  buttonDisabled: {
    elevation: 0,
    backgroundColor: '#dfdfdf'
  },
  textDisabled: {
    color: '#a1a1a1'
  }
});
