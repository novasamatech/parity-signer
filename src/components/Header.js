import React from 'react'
import { Text, View, StyleSheet, Image } from 'react-native'
import colors from '../colors'

export default function () {
  return (
    <View style={styles.row}>
      <Image source={require('../../icon.png')} style={styles.logo} />
      <Text style={styles.headerTextLeft}>parity</Text>
      <Text style={styles.headerTextRight}>Secured</Text>
    </View>)
}


const styles = StyleSheet.create({
  row: {
    backgroundColor: colors.bg,
    height: 60,
    flexDirection: 'row',
    alignItems: 'center',
    padding: 14,
    borderBottomWidth: 0.5,
    borderBottomColor: colors.bg_text_sec,

  },
  logo: {
    width: 50,
    height: 50
  },
  headerTextLeft: {
    flex: 1,
    paddingLeft: 10,
    fontSize: 25,
    fontWeight: 'bold',
    color: colors.bg_text
  },
  headerTextRight: {
    marginLeft: 0,
    fontSize: 17,
    fontWeight: 'bold',
    color: colors.bg_text_positive
  },
});
