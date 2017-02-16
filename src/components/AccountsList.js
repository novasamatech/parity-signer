import React, { Component, PropTypes } from 'react'
import { Text, StyleSheet, ListView, RecyclerViewBackedScrollView } from 'react-native'
import AccountsListRow from './AccountsListRow'

export default class AccountsList extends Component {
  static propTypes = {
    accounts: PropTypes.arrayOf(PropTypes.shape({
      address: PropTypes.string.isRequired,
    })).isRequired,
    onAccountSelected: PropTypes.func.isRequired,
  }

  constructor(props) {
    super(props)
    const ds = new ListView.DataSource({rowHasChanged: (r1, r2) => r1 !== r2})
    this.state = {
      dataSource: ds.cloneWithRows(props.accounts)
    }
  }

  componentWillReceiveProps(nextProps) {
    this.setState({
      dataSource: this.state.dataSource.cloneWithRows(nextProps.accounts)
    })
  }

  render() {
    return (
      <ListView
        style={styles.view}
        dataSource={this.state.dataSource}
        renderRow={(rowData, sectionID: number, rowID: number, highlightRow) => {
          return (
            <AccountsListRow text={rowData.name ? rowData.name : '0x' + rowData.address} onPress={() => {
              highlightRow(sectionID, rowID)
              this.props.onAccountSelected(this.props.accounts[rowID])
            }}/>
          )
        }}
        enableEmptySections={true}
        renderScrollComponent={props => <RecyclerViewBackedScrollView {...props} />}
      />
    )
  }
}

const styles = StyleSheet.create({
  view: {
    flex: 1,
    marginTop: 60,
    marginBottom: 50,
  },
})

