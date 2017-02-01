import React, { Component } from 'react'
import { View, Text, StyleSheet } from 'react-native'

export default class TabBarItem extends Component {
  render() {
    const {
      name,
      selectedItem,
      text
    } = this.props

    const selected = selectedItem == name

    return (
      <View style={styles.item}>
        <Text style={selected ? styles.selected : styles.normal}>{text}</Text>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  item: {
    alignItems: 'center',
  },
  normal: {
    color: 'grey',
  },
  selected: {
    color: 'red',
  },
})
