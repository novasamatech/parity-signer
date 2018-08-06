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
import colors from '../colors';
import withDimensions from './utils/withDimensions';

type Props = TabBarOptions & {
  navigation: any
};

const majorVersion = parseInt(Platform.Version, 10);
const isIos = Platform.OS === 'ios';
const isIOS11 = majorVersion >= 11 && isIos;

class TabBarBottom extends React.PureComponent<Props> {
  static defaultProps = {
    activeTintColor: '#3478f6', // Default active tint color in iOS 10
    activeBackgroundColor: colors.bg,
    inactiveTintColor: '#929292', // Default inactive tint color in iOS 10
    inactiveBackgroundColor: colors.bg,
    showLabel: true,
    showIcon: true,
    allowFontScaling: true,
    adaptive: isIOS11
  };

  render() {
    const {
      navigation,
      activeBackgroundColor,
      inactiveBackgroundColor,
      onTabPress,
      jumpTo,
      style,
      tabStyle
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
          const backgroundColor = focused ? colors.card_bg : colors.bg;
          const color = focused ? colors.bg : colors.card_bg;
          return (
            <TouchableWithoutFeedback
              key={route.key}
              onPress={() => {
                if (focused) {
                  if (route.routes.length > 1) {
                    // In case of we are not in the "home" route and route is "focused" we want to go home
                    navigation.popToTop();
                  } else {
                    // In case we are on the "home" route we want to go to the first item in the list
                    if (route.key === 'Accounts') {
                      navigation.navigate('AccountList', { index: 0 });
                    }
                  }
                } else {
                  navigation.navigate(route.key);
                }
              }}
            >
              <View style={[styles.tab, { backgroundColor }]}>
                <Icon
                  style={[styles.labelIcon, { color }]}
                  name={route.key === 'Scanner' ? 'qrcode' : 'briefcase'}
                />
                <Text style={[styles.labelText, { color }]}>
                  {route.key === 'Scanner' ? 'Scanner' : 'Accounts'}
                </Text>
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
    flexDirection: 'row'
  },
  tabBarRegular: {
    height: DEFAULT_HEIGHT
  },
  tab: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center'
  },
  labelText: {
    fontSize: 22,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    color: colors.card_bg,
    color: 'white'
  },
  labelIcon: {
    fontSize: 30,
    paddingRight: 10
  }
});

export default withDimensions(TabBarBottom);
