'use strict'

import React, { Component, PropTypes } from 'react'
import { Image, View, StyleSheet } from 'react-native'
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
    seed: PropTypes.string.isRequired
  }

  state = {}

  componentDidMount () {
    displayIcon(this, this.props.seed)
  }

  componentWillReceiveProps (newProps) {
    if (newProps.seed !== this.props.seed) {
      displayIcon(this, newProps.seed)
    }
  }

  render () {
    return (
      <View style={styles.identicon}>
        <Image
          style={this.props.style || {}}
          source={{uri: this.state.icon}}
        />
      </View>
    )
  }
}

const styles = StyleSheet.create({
  identicon: {
    alignItems: 'center'
  }
})
