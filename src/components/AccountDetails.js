'use strict'

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, ScrollView, Text, Button, TouchableOpacity } from 'react-native'
import Prompt  from 'react-native-prompt'
import AppStyles from '../styles'
import AccountIcon from './AccountIcon'
import QrView from './QrView';

export default class AccountDetails extends Component {
  static propTypes = {
    account: PropTypes.shape({
      address: PropTypes.string.isRequired
    }).isRequired,
    onNameChange: PropTypes.func.isRequired,
    onDelete: PropTypes.func.isRequired
  }

  state = {
    isEditing: false
  }

  startEdit = () => {
    this.setEditing(true)
  }

  cancelEdit = () => {
    this.setEditing(false)
  }

  finishEdit = (newName) => {
    this.setEditing(false)
    this.props.onNameChange(this.props.account, newName)
  }

  setEditing(isEditing) {
    this.setState({ isEditing })
  }

  renderPrompt () {
    return (
      <Prompt
        title='Account Name'
        defaultValue={this.props.account.name}
        visible={this.state.isEditing}
        onCancel={this.cancelEdit}
        onSubmit={this.finishEdit}
        />
    );
  }

  render () {
    return (
      <ScrollView style={AppStyles.view}>
        <View style={styles.identicon}>
          <AccountIcon style={AppStyles.icon} seed={'0x' + this.props.account.address} />
        </View>

        <TouchableOpacity style={styles.wrapper}
          onPress={this.startEdit}
          >
          <View>
            <Text style={AppStyles.hintText}>Name</Text>
            <Text selectable style={AppStyles.valueText}>{this.props.account.name ? this.props.account.name : 'no name'}</Text>
            { this.renderPrompt() }
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
            style={[styles.actionButtonContainer, {opacity: 0.0}]}
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
  identicon: {
    alignItems: 'center'
  },
  wrapper: {
    borderRadius: 5
  },
  qr: {
    padding: 10,
    marginTop: 20
  },
  deleteText: {
    textAlign: 'right',
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
