import React from 'react';
import { Text, View, StyleSheet, Image } from 'react-native';
import colors from '../colors';

export default class HeaderLeftHome extends React.PureComponent {
  render() {
    return (
      <View
        style={{ flexDirection: 'row', alignItems: 'center' }}
        accessibilityComponentType="button"
        accessibilityTraits="button"
        testID="header-back"
        delayPressIn={0}
        onPress={() => this.props.onPress && this.props.onPress()}
      >
        <Image source={require('../../icon.png')} style={styles.logo} />
        <Text style={styles.headerTextLeft}>parity</Text>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  headerStyle: {
    backgroundColor: colors.bg,
    height: 60,
    flexDirection: 'row',
    alignItems: 'center',
    padding: 14,
    borderBottomWidth: 0.5,
    borderBottomColor: colors.bg_text_sec
  },
  logo: {
    width: 42,
    height: 42
  },
  headerTextLeft: {
    flex: 1,
    paddingLeft: 10,
    fontSize: 25,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    color: colors.bg_text
  }
});
