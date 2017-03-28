'use strict'

import React, { Component, PropTypes } from 'react'
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
        <TouchableOpacity style={styles.wrapper}
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
