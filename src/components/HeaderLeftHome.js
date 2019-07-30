import React from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import colors from '../colors';

export default class HeaderLeftHome extends React.PureComponent {
  render() {
    return (
      <View
        style={{ flexDirection: 'row', alignItems: 'center', paddingLeft: 14 }}
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
    borderBottomWidth: 0.5,
    borderBottomColor: colors.bg_text_sec
  },
  logo: {
    width: 42,
    height: 42
  },
  headerTextLeft: {
    flex: 1,
    paddingLeft: 4,
    fontSize: 25,
    fontFamily: 'Manifold CF',
    color: colors.bg_text
  }
});
