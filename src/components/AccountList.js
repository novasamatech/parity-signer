'use strict'

import React, { Component, PropTypes } from 'react'
import { Button, View, Text, ListView, RecyclerViewBackedScrollView, StatusBar, StyleSheet } from 'react-native'
import AccountListRow from './AccountListRow'
import AppStyles from '../styles'

export default class AccountList extends Component {
  static propTypes = {
    accounts: PropTypes.arrayOf(PropTypes.shape({
      address: PropTypes.string.isRequired
    })).isRequired,
    onNewAccount: PropTypes.func.isRequired,
    onAccountSelected: PropTypes.func.isRequired
  }

  constructor (props) {
    super(props)
    const ds = new ListView.DataSource({rowHasChanged: (r1, r2) => r1 !== r2})
    this.state = {
      dataSource: ds.cloneWithRows(props.accounts)
    }
  }

  componentWillReceiveProps (nextProps) {
    this.setState({
      dataSource: this.state.dataSource.cloneWithRows(nextProps.accounts)
    })
  }

  render () {
    if (!this.props.accounts.length) {
      return (
        <View style={AppStyles.view}>
          <View style={styles.introContainer}>
            <Text style={styles.introText}>
              To sign transactions you need at least one account.
            </Text>
            <View style={AppStyles.buttonContainer}>
              <Button
                style={styles.introButton}
                onPress={this.props.onNewAccount}
                color='green'
                title='Create Account'
                accessibilityLabel='Create new account.'
              />
            </View>
          </View>
        </View>
      )
    }
    return (
      <ListView
        style={AppStyles.listView}
        dataSource={this.state.dataSource}
        renderRow={(rowData, sectionID: number, rowID: number, highlightRow) => {
          return (
            <AccountListRow
              upperText={rowData.name ? rowData.name : 'no name'}
              lowerText={'0x' + rowData.address}
              onPress={() => {
                highlightRow(sectionID, rowID)
                this.props.onAccountSelected(this.props.accounts[rowID])
              }}
            />
          )
        }}
        enableEmptySections
        renderScrollComponent={props => <RecyclerViewBackedScrollView {...props} />}
      >
        <StatusBar barStyle='light-content' />
      </ListView>
    )
  }
}

const styles = StyleSheet.create({
  introContainer: {
    padding: 30,
    flex: 1,
    flexDirection: 'column',
    justifyContent: 'center'
  },
  introText: {
    textAlign: 'center',
    fontSize: 16,
    marginBottom: 20
  }
})
