'use strict'

import React, { Component, PropTypes } from 'react'
import { Image } from 'react-native'
import { blockiesIcon } from '../util/native'

async function displayIcon (self, seed) {
  try {
    let icon = await blockiesIcon(seed)
    self.setState({
      icon: icon
    })
  } catch (e) {
    console.log(e)
  }
}

export default class AccountIcon extends Component {
  static propTypes = {
    seed: PropTypes.string.isRequired,
  }

  state = {}

  componentDidMount () {
    displayIcon(this, this.props.seed)
  }

  render () {
    return (
      <Image
        style={this.props.style || {}}
        source={{uri: this.state.icon}}
      />
    )
  }
}

