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
import { StyleSheet, ScrollView, View, Text } from 'react-native'
import { Subscribe } from 'unstated'
import ScannerStore from '../stores/ScannerStore'
import AccountsStore from '../stores/AccountsStore'
import Button from '../components/Button'
import AccountCard from '../components/AccountCard'
import TxDetailsCard from '../components/TxDetailsCard'
import AppStyles from '../styles'
import AccountPrettyAddress from '../components/AccountPrettyAddress'
import colors from '../colors';

const orUnknown = (value = 'Unknown') => value

export default class TxDetails extends Component {
  render() {
    return <Subscribe to={[ScannerStore, AccountsStore]}>{
      (scannerStore, accounts) => {
        const txRequest = scannerStore.getTXRequest()
        if (txRequest) {
          const sender = accounts.getByAddress(txRequest.data.account)
          return <TxDetailsView
            { ...scannerStore.getTx() }
            sender = { sender.address }
            senderName = { sender.name }
            dataToSign = { scannerStore.getDataToSign() }
            onNext = { async () => {
              if (!sender) {
                scannerStore.setErrorMsg(`No account with address ${txRequest.data.account} found in your wallet`)
                return
              }
              try {
                await scannerStore.signData(sender)
                this.props.navigation.navigate('SignedTx')
              } catch (e) {
                console.log(e)
                scannerStore.setErrorMsg(e.message)
              }
            }} />
        } else {
          return null
        }
      }
    }
    </Subscribe>
  }
}

export class TxDetailsView extends Component {
  static propTypes = {
    onNext: PropTypes.func.isRequired,
    dataToSign: PropTypes.string.isRequired,
    sender: PropTypes.string.isRequired,
    action: PropTypes.string,
    value: PropTypes.string,
    nonce: PropTypes.string,
    gas: PropTypes.string,
    gasPrice: PropTypes.string,
    data: PropTypes.string,
    isSafe: PropTypes.bool.isRequired,
    senderName: PropTypes.string.isRequired,
    recipientName: PropTypes.string
  }

  render () {
    return (
      <ScrollView contentContainerStyle={ styles.bodyContent } style={ styles.body }>
        <Text style={ styles.topTitle }>SIGN TRANSACTION</Text>
        <Text style={ styles.title }>FROM ACCOUNT</Text>
        <AccountCard
            title={this.props.senderName || 'no name'}
            address={this.props.sender}
            onPress={() => {}}
          />
        <Text style={ styles.title }>TRANSACTION DETAILS</Text>
        <TxDetailsCard value={ this.props.value } recipient={ this.props.action } />
        <Button
          buttonStyles={ { backgroundColor: colors.bg_positive, marginTop: 20, height: 60 } } title="Sign Transaction"
          onPress={ () => this.props.onNext() } />
      </ScrollView>
    )
  }
}

const styles = StyleSheet.create({
  body: {
    flex: 1,
    flexDirection: 'column',
    padding: 20,
    backgroundColor: colors.bg
  },
  bodyContent: {
    paddingBottom: 40,
  },
  transactionDetails: {
    flex: 1,
    backgroundColor: colors.card_bg,
  },
  topTitle: {
    textAlign: 'center',
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20
  },
  title: {
    textAlign: 'center',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 20
  },
  wrapper: {
    borderRadius: 5
  },
  address: {
    flex: 1
  },
  deleteText: {
    textAlign: 'right'
  },
  changePinText: {
    textAlign: 'left',
    color: 'green'
  },
  actionsContainer: {
    flex: 1,
    flexDirection: 'row'
  },
  actionButtonContainer: {
    flex: 1
  }
})
