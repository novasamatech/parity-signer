/* @flow */

import React from 'react';
import {
  Text,
  Animated,
  TouchableWithoutFeedback,
  StyleSheet,
  View,
  Platform,
  SafeAreaView
} from 'react-native';
import Icon from 'react-native-vector-icons/FontAwesome';
import colors from '../colors'
import withDimensions from './utils/withDimensions';

type Props = TabBarOptions & {
  navigation: any,
};

const majorVersion = parseInt(Platform.Version, 10);
const isIos = Platform.OS === 'ios';
const isIOS11 = majorVersion >= 11 && isIos;

class TabBarBottom extends React.Component<Props> {
  static defaultProps = {
    activeTintColor: '#3478f6', // Default active tint color in iOS 10
    activeBackgroundColor: colors.bg,
    inactiveTintColor: '#929292', // Default inactive tint color in iOS 10
    inactiveBackgroundColor: colors.bg,
    showLabel: true,
    showIcon: true,
    allowFontScaling: true,
    adaptive: isIOS11,
  };

  render() {
    const {
      navigation,
      activeBackgroundColor,
      inactiveBackgroundColor,
      onTabPress,
      jumpTo,
      style,
      tabStyle,
    } = this.props;

    const { routes } = navigation.state;

    const tabBarStyle = styles.tabBar;

    return (
      <SafeAreaView
        style={tabBarStyle}
        forceInset={{ bottom: 'always', top: 'never' }}
      >
        {routes.map((route, index) => {
          const focused = index === navigation.state.index;
          const scene = { route, focused };

          const backgroundColor = focused
            ? colors.card_bg
            : colors.bg;

          const color = focused
            ? colors.bg
            : colors.card_bg;

          return (
            <TouchableWithoutFeedback
              key={route.key}
              onPress={() => {
                navigation.navigate(route.key);
              }}
            >
              <View
                style={[
                  styles.tab,
                  { backgroundColor }
                ]}
              >
                <Icon style={[ styles.labelIcon, { color }]} name={ route.key === 'Scanner' ? 'qrcode' : 'briefcase' } />
                <Text style={[styles.labelText, { color }]}>{ route.key === 'Scanner' ? 'Scanner' : 'Accounts' }</Text>
              </View>
            </TouchableWithoutFeedback>
          );
        })}
      </SafeAreaView>
    );
  }
}

const DEFAULT_HEIGHT = 65;

const styles = StyleSheet.create({
  tabBar: {
    height: DEFAULT_HEIGHT,
    backgroundColor: colors.bg,
    borderTopWidth: StyleSheet.hairlineWidth,
    borderTopColor: colors.bg_text_sec,
    flexDirection: 'row',
  },
  tabBarRegular: {
    height: DEFAULT_HEIGHT,
  },
  tab: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center'
  },
  labelText: {
    fontSize: 22,
    fontFamily: 'Roboto',
    fontWeight: '400',
    color: colors.card_bg,
    color:'white',
  },
  labelIcon: {
    fontSize: 30,
    paddingRight: 10
  }
});

export default withDimensions(TabBarBottom);
