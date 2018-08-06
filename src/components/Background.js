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

import React from 'react';
import { View, StyleSheet } from 'react-native';
import colors from '../colors';

export default class Background extends React.PureComponent {
  render() {
    // const lines = new Array(100)
    //   .fill(0)
    //   .map((_, i) => <View key={i} style={styles.line} />);
    return (
      <View style={styles.bg}>
        {/* <View style={styles.lines}>{lines}</View> */}
      </View>
    );
  }
}

const styles = StyleSheet.create({
  bg: {
    position: 'absolute',
    backgroundColor: colors.bg,
    flex: 1
  },
  lines: {
    position: 'absolute',
    zIndex: -1000,
    transform: [
      { rotate: '-30deg' },
      { translateX: -300 },
      { translateY: -3100 },
      { scale: 0.2 }
    ]
  },
  line: {
    zIndex: -1000,
    height: 60,
    width: 4000,
    borderBottomWidth: 2,
    borderBottomColor: '#3d3d3d',
    backgroundColor: colors.bg
  }
});
