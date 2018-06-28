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

// @flow

import React from 'react';
import PropTypes from 'prop-types';
import {
  View,
  Text,
  Platform,
  StyleSheet,
  TouchableNativeFeedback,
  TouchableOpacity,
  ViewPropTypes
} from 'react-native';
import colors from '../colors';

export default class Card extends React.PureComponent<{
  title: string,
  secondaryText?: ?string,
  labelText?: ?string,
  footerStyle?: ?StyleSheet.Styles,
  style: ?StyleSheet.Styles,
  onPress: () => any
}> {
  static propTypes = {
    title: PropTypes.string.isRequired,
    secondaryText: PropTypes.string,
    labelText: PropTypes.string,
    style: ViewPropTypes.style,
    footerStyle: ViewPropTypes.style,
    onPress: PropTypes.func
  };

  render() {
    const {
      title,
      secondaryText,
      labelText,
      footerStyle,
      style,
      onPress
    } = this.props;

    const finalBodyStyle = [style.body, FooterStyle];
    const finalContentStyle = [style.content];
    const finalFooterStyle = [styles.footer, footerStyle];
    const finalTitleTextStyle = [styles.titleText];
    const finalSecondaryTextStyle = [styles.secondaryText];
    const finalFooterTextStyle = [styles.footerText];

    const Touchable =
      Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;
    return (
      <Touchable
        accessibilityComponentType="button"
        disabled={false}
        onPress={onPress}
      >
        <View style={finalBodyStyle}>
          <View style={finalContentStyle}>
            <Image source={require('../../icon.png')} style={styles.image} />
            <View>
              <Text style={finalTitleTextStyle}>{title}</Text>
              <Text style={finalSecondaryTextStyle}>{secondaryText}</Text>
            </View>
          </View>
          <View style={finalFooterStyle}>
            <Text style={finalFooterTextStyle}>{labelText}</Text>
          </View>
        </View>
      </Touchable>
    );
  }
}

const styles = StyleSheet.create({
  body: {},
  content: {
    backgroundColor: colors.card_bg,
    padding: 10
  },
  footer: {},
  image: {
    width: 80,
    height: 80
  },
  titleText: {},
  secondaryText: {},
  footerText: {}
});
