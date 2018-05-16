import React, { Component } from 'react'
import { View, StyleSheet } from 'react-native'
import colors from '../colors'

export default class Background extends Component {
  render() {
    const lines = (new Array(100)).fill(0).map((_, i) => (
      <View key={i} style={ styles.line } />
    ))

    console.log(lines)
    return (
      <View style={ styles.bg }>
        <View { ...this.props }>{ this.props.children }</View>
        <View style={ styles.lines }>
        { lines }
        </View>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  bg: {
    position: 'relative',
    flex: 1,
  },
  lines: {
    backgroundColor: 'transparent',
    position: 'absolute',
    zIndex: -1000,
    transform: [{ rotate: '-30deg' }, { translateX: -300}, { translateY: -3000}, {scale: 0.2}]
  },
  line: {
    zIndex: -1000,
    height: 60,
    width: 4000,
    borderBottomWidth: 2,
    borderBottomColor: colors.bg_text_sec_2,
    backgroundColor: colors.bg,
  },
});
