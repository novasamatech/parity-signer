import React, { Component, PropTypes } from 'react'
import { Text, StyleSheet } from 'react-native'

export default class TabIcon extends Component {
  static propTypes = {
    title: PropTypes.string.isRequired,
  }

  render() {
    return (
      <Text style={this.props.selected ? styles.selected: styles.normal}>{this.props.title}</Text>
    )
  }
}

const styles = StyleSheet.create({
  normal: {
    color: 'white',
  },
  selected: {
    fontWeight: 'bold',
    color: 'green',
  }
})

