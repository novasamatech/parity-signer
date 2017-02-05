import React, { Component, PropTypes } from 'react'
import { Text } from 'react-native'

export default class TabIcon extends Component {
  render() {
    return (
      <Text style={{color: this.props.selected ? 'red' :'black'}}>{this.props.title}</Text>
    )
  }
}

TabIcon.propTypes = {
  title: PropTypes.string.isRequired,
}
