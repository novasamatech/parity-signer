'use strict'

import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { Button, View, ScrollView, StyleSheet, TouchableOpacity } from 'react-native'
import AppStyles from '../styles'

import AccountIcon from './AccountIcon'

import { brainWalletAddress, words } from '../util/native'

export default class IconChooser extends Component {
  static propTypes = {
    onSelect: PropTypes.func.isRequired
  }

  state = {
    icons: []
  }

  refreshIcons = async () => {
    try {
      const icons = await Promise.all(Array(6).join(' ').split(' ').map(() => words()))
      this.setState({ icons })
    } catch (e) {
      console.error(e)
    }
  }

  componentDidMount () {
    this.refreshIcons()
  }

  render () {
    return (
      <ScrollView style={AppStyles.view}>
        <View style={styles.iconsContainer}>
          {
            this.state.icons.map(seed => (
              <Icon
                key={seed}
                seed={seed}
                onSelect={this.props.onSelect}
                />
            ))
          }
        </View>
        <Button
          onPress={this.refreshIcons}
          color='green'
          title='More'
        />
      </ScrollView>
    )
  }
}

class Icon extends Component {
  static propTypes = {
    seed: PropTypes.string.isRequired,
    onSelect: PropTypes.func.isRequired
  }

  state = {
    address: null
  }

  componentDidMount () {
    this.refreshAddress(this.props.seed)
  }

  componentWillReceiveProps (newProps) {
    const { seed } = this.props
    if (newProps.seed === seed) {
      return
    }

    this.refreshAddress(seed)
  }

  async refreshAddress (seed) {
    try {
      const address = await brainWalletAddress(seed)
      if (this.props.seed === seed) {
        this.setState({ address })
      }
    } catch (e) {
      console.error(e)
    }
  }

  onSelect = () => {
    this.props.onSelect(this.props.seed)
  }

  render () {
    const { seed } = this.props
    const { address } = this.state
    return (
      <View style={styles.icon} key={seed}>
        {address ? (
          <TouchableOpacity onPress={this.onSelect}>
            <AccountIcon style={AppStyles.icon} seed={'0x' + address} />
          </TouchableOpacity>
        ) : null}
      </View>
    )
  }
}

const styles = StyleSheet.create({
  iconsContainer: {
    justifyContent: 'center',
    flexDirection: 'row',
    flexWrap: 'wrap',
    marginBottom: 20
  },
  icon: {
    marginLeft: 15,
    marginRight: 15,
    marginTop: 10,
    width: 100,
    height: 100
  }
})
