import React, { Component, PropTypes } from 'react'
import { Text } from 'react-native'

export default class TabIcon extends Component {
  static propTypes = {
    title: PropTypes.string.isRequired,
  }

  render() {
    return (
      <Text style={{color: this.props.selected ? 'red' :'black'}}>{this.props.title}</Text>
    )
  }
}
