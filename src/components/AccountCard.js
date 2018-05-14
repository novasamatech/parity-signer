// @flow

import React from 'react'
import PropTypes from 'prop-types'
import { View, Text, Platform, StyleSheet,
  TouchableNativeFeedback, TouchableOpacity} from 'react-native'
import colors from '../colors'
import Card from './Card'
import AccountIcon from './AccountIcon'

export default class AccountCard extends React.Component<{
  title: string,
  address: string,
  networkId: number,
  onPress: () => any,
}> {
  static propTypes = {
    title: PropTypes.string.isRequired,
    address: PropTypes.string.isRequired,
    networkId: PropTypes.number,
    onPress: PropTypes.func,
  };

  render() {
    const {
      title,
      address,
      networkId,
      onPress
    } = this.props;

    const Touchable = Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;

    return (
      <Touchable
        accessibilityComponentType="button"
        disabled={false}
        onPress={onPress}>
        <View style={styles.body}>
          <View style={styles.content}>
            <AccountIcon style={styles.icon} seed={"0x" + address} />
            <View style={styles.desc}>
              <Text style={styles.titleText}>{title}</Text>
              <Text style={styles.secondaryText}>0x{address}</Text>
            </View>
          </View>
          <View style={styles.footer}>
            <Text style={styles.footerText}>Ethereum</Text>
          </View>
        </View>
      </Touchable>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    paddingBottom: 20,
  },
  content: {
    flexDirection: 'row',
    backgroundColor: colors.card_bg,
    padding: 10,
  },
  icon: {
    width: 47,
    height: 47
  },
  desc: {
    flexDirection: 'column',
    justifyContent: 'space-between',
    paddingLeft: 10,
    flex: 1
  },
  footer: {
    backgroundColor: '#977CF6',
    flexDirection: 'row-reverse',
    padding: 5
  },
  titleText: {
    fontSize: 20
  },
  secondaryText: {
    color: colors.bg_text_sec,
    fontWeight: '500',
    fontSize: 10
  },
  footerText: {
    color: colors.card_bg,
    fontWeight: 'bold'
  }
});
