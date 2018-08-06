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
import { Text, View, StyleSheet, Image, SafeAreaView } from 'react-native';
import colors from '../colors';

export default function() {
  return (
    <SafeAreaView style={{ backgroundColor: colors.bg }}>
      <View style={styles.row}>
        <Image source={require('../../icon.png')} style={styles.logo} />
        <Text style={styles.headerTextLeft}>parity</Text>
        <Text style={styles.headerTextRight}>Secure</Text>
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  row: {
    backgroundColor: colors.bg,
    height: 60,
    flexDirection: 'row',
    alignItems: 'center',
    padding: 14,
    borderBottomWidth: 0.5,
    borderBottomColor: colors.bg_text_sec
  },
  logo: {
    width: 50,
    height: 50
  },
  headerTextLeft: {
    flex: 1,
    paddingLeft: 10,
    fontSize: 25,
    fontFamily: 'Manifold CF',
    fontWeight: '900',
    color: colors.bg_text
  },
  headerTextRight: {
    marginLeft: 0,
    fontFamily: 'Roboto',
    fontSize: 17,
    fontWeight: 'bold',
    color: colors.bg_text_positive
  }
});
