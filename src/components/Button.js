// @flow

import React from 'react'
import PropTypes from 'prop-types'
import { View, Text, Platform, StyleSheet,
         TouchableNativeFeedback, TouchableOpacity } from 'react-native'
import colors from '../colors'

export default class Button extends React.Component<{
  title: string,
  onPress: () => any,
  textStyles?: ?StyleSheet.Styles,
  buttonStyles?: ?StyleSheet.Styles,
  disabled?: ?boolean,
}> {
  static propTypes = {
    title: PropTypes.string.isRequired,
    textStyles: Text.propTypes.style,
    buttonStyles: View.propTypes.style,
    disabled: PropTypes.bool,
    onPress: PropTypes.func.isRequired,
  };

  render() {
    const {
      onPress,
      title,
      disabled,
      textStyles,
      buttonStyles
    } = this.props;

    const finalTextStyles = [styles.text, textStyles];
    const finalButtonStyles = [styles.button, buttonStyles];

    if (disabled) {
      finalTextStyles.push(styles.textDisabled);
      finalButtonStyles.push(styles.buttonDisabled);
    }

    const Touchable = Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;
    return (
      <Touchable
        accessibilityComponentType="button"
        disabled={disabled}
        onPress={onPress}>
        <View style={finalButtonStyles}>
          <Text style={finalTextStyles} disabled={disabled}>{title}</Text>
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
  },
  text: {
    color: 'white',
    padding: 8,
    fontWeight: 'bold',
    fontSize: 20
  },
  buttonDisabled: {
    elevation: 0,
    backgroundColor: '#dfdfdf',
  },
  textDisabled: {
    color: '#a1a1a1',
  },
});
