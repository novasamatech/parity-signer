'use strict'

import React, { Component, PropTypes } from 'react'
import { Text } from 'react-native'
import AppStyles from '../styles'

export default class AccountAddress extends Component {
  static propTypes = {
    address: PropTypes.string.isRequired
  }

  state = {
    address: this.props.address
  }

  componentWillReceiveProps (newProps) {
    if (newProps.address !== this.props.address) {
      this.setState({
        address: newProps.address
      })
    }
  }

  render () {
    return (
      <Text selectable style={AppStyles.valueText}>0x{this.state.address}</Text>
    )
  }
}
