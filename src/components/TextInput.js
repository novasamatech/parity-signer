import React from 'react';
import { TextInput as TextInputOrigin, Platform, StyleSheet } from 'react-native';
import colors from '../colors';

export default function TextInput(props) {
  return <TextInputOrigin {...props} style={[styles.input, props.style]} />;
}

const styles = StyleSheet.create({
  input: {
    fontSize: 24,
    height: 60,
    justifyContent: 'center',
    alignItems: 'center',
    elevation: 4,
    padding: 18,
    backgroundColor: colors.card_bg
  }
});
