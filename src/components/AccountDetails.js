// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

'use strict'

import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { StyleSheet, View, ScrollView, Text, TextInput, TouchableOpacity } from 'react-native'
import AppStyles from '../styles'
import AccountIcon from './AccountIcon'
import QrView from './QrView'

export default class AccountDetails extends Component {
  static propTypes = {
    account: PropTypes.shape({
      address: PropTypes.string.isRequired
    }).isRequired,
    onNameChange: PropTypes.func.isRequired,
    onChangePin: PropTypes.func.isRequired,
    onDelete: PropTypes.func.isRequired
  }

  state = {
    isEditing: false,
    name: this.props.account.name
  }

  startEdit = () => {
    this.setEditing(true)
    this.setState({
      name: this.props.account.name
    })
  }

  cancelEdit = () => {
    this.setEditing(false)
  }

  finishEdit = () => {
    this.setEditing(false)
    this.props.onNameChange(this.props.account, this.state.name)
  }

  updateName = (name) => {
    this.setState({ name })
  }

  setEditing (isEditing) {
    this.setState({ isEditing })
  }

  render () {
    return (
      <ScrollView style={AppStyles.view}>
        <AccountIcon style={AppStyles.icon} seed={'0x' + this.props.account.address} />
        <TouchableOpacity
          style={styles.wrapper}
          onLongPress={this.startEdit}
          >
          <View>
            <Text style={AppStyles.hintText}>Name</Text>
            { this.state.isEditing
              ? (
                <TextInput
                  style={[AppStyles.valueText, AppStyles.valueTextInput]}
                  value={this.state.name}
                  autoFocus
                  onChangeText={this.updateName}
                  onEndEditing={this.cancelEdit}
                  onSubmitEditing={this.finishEdit}
                />
              ) : (
                <Text style={AppStyles.valueText}>{this.props.account.name ? this.props.account.name : 'no name'}</Text>
              )
            }
          </View>
        </TouchableOpacity>

        <View>
          <Text style={AppStyles.hintText}>Address</Text>
          <Text selectable style={AppStyles.valueText}>0x{this.props.account.address}</Text>
        </View>

        <View style={styles.qr}>
          <QrView text={this.props.account.address} />
        </View>

        <View style={[styles.actionsContainer, AppStyles.buttonContainer]}>
          <TouchableOpacity
            style={styles.actionButtonContainer}
            onPress={() => this.props.onChangePin(this.props.account)}
            >
            <Text style={styles.changePinText}>Change PIN</Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={styles.actionButtonContainer}
            onPress={() => this.props.onDelete(this.props.account)}
            >
            <Text style={styles.deleteText}>Delete</Text>
          </TouchableOpacity>
        </View>
      </ScrollView>
    )
  }
}

const styles = StyleSheet.create({
  wrapper: {
    borderRadius: 5
  },
  qr: {
    padding: 10,
    marginTop: 20
  },
  deleteText: {
    textAlign: 'right'
  },
  changePinText: {
    textAlign: 'left',
    color: 'green'
  },
  actionsContainer: {
    marginTop: 40,
    flexDirection: 'row'
  },
  actionButtonContainer: {
    flex: 1
  }
})
