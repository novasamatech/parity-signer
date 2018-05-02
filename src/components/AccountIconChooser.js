// @flow

import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { View, Text, StyleSheet, ListView, TouchableOpacity} from 'react-native'
import { brainWalletAddress, words } from '../util/native'
import colors from '../colors'
import Card from './Card'
import AccountIcon from './AccountIcon'
import AppStyles from '../styles'

export default class AccountIconChooser extends Component<{
  selectedAddress: string,
  onSelect: () => any,
  }> {
  static propTypes = {
    selectedAddress : PropTypes.string.isRequired,
    onSelect: PropTypes.func,
  };

  constructor(props) {
    super(props)
    this.icons = []
    const iconsDS = new ListView.DataSource({rowHasChanged: (r1, r2) => true})
    this.state = { iconsDS };
  }

  refreshIcons = async () => {
    try {
      this.icons = [
        ...this.icons,
        ...await Promise.all(Array(10).join(' ').split(' ').map(async () => {
          const seed = await words();
          return {
            seed,
            address: await brainWalletAddress(seed)
          }
        }))
      ]
      this.setState({ iconsDS: this.state.iconsDS.cloneWithRows(this.icons)})
    } catch (e) {
      console.error(e)
    }
  }

  componentDidMount () {
    this.refreshIcons()
  }

  render() {
    const {
      selectedAddress,
      onSelect
    } = this.props;

    return (
      <View style={styles.body}>
        <ListView
          style={styles.icons}
          dataSource={this.state.iconsDS}
          horizontal={true}
          renderRow={({ address, seed }, sectionID: number, rowID: number, highlightRow) => {
            const selected = address === selectedAddress
            const style = [styles.icon]
            return (
              <TouchableOpacity style={ [styles.iconBorder, address === selectedAddress ? styles.selected : {}] } onPress={ () => this.props.onSelect(address) }>
                  <AccountIcon style={style} seed={'0x' + address} />
              </TouchableOpacity>
            )
          }}>
        </ListView>
        <Text style={styles.addressText}>{selectedAddress ? `0x${selectedAddress}` : `Select an identicon`}</Text>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    flex: 1,
    flexDirection: 'column',
    marginBottom: 20,
    padding: 20,
    backgroundColor: colors.card_bg,
  },
  icons: {
    backgroundColor: colors.card_bg,
  },
  icon: {
    width: 50,
    height: 50,
    margin: 6,
    padding: 5
  },
  iconBorder: {
    borderWidth: 6,
    borderColor: colors.card_bg,
  },
  selected: {
    borderColor: colors.bg,
  },
  addressText: {
    paddingTop: 20,
    color: colors.bg,
    fontWeight: '700',
    fontSize: 11
  },
});
