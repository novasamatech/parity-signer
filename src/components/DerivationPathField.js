// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

'use strict';

import React, { useState } from 'react';
import {
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import styles from '../styles';
import {parseDerivationPath} from '../util/suri'
import TextInput from './TextInput';

export default function DerivationPathField(props) {
  const { onChange, styles } = props;
  const [showAdvancedField, setShowAdvancedField] =  useState(false)
  const [isValidPath, setIsValidPath] = useState(true);

  const toggleShowAdvancedField = () => {
    setShowAdvancedField(!showAdvancedField)
  }

  return (
    <View style={styles.b_marginBottom}>
      <TouchableOpacity
        onPress={toggleShowAdvancedField}
        style={{diplay:'flex'}}
      >
        <Text style={[styles.t_text, styles.b_row, styles.b_marginV_xs]}>
          Advanced
          <Icon 
            name={showAdvancedField ? 'arrow-drop-up' : 'arrow-drop-down'}
          />
        </Text>

      </TouchableOpacity>
      {showAdvancedField && 
        <TextInput
          onChangeText={(text) => {
            try {
              const derivationPath = parseDerivationPath(text);

              onChange({
                derivationPassword: derivationPath.password || '',
                derivationPath: derivationPath.derivePath || '',
                isDerivationPathValid: true
              });
              setIsValidPath(true);
            } catch (e) {
              // wrong derivationPath
              onChange({
                derivationPassword: '',
                derivationPath: '',
                isDerivationPathValid: false
              });
              setIsValidPath(false);
            }
          }}
          placeholder="optional derivation path"
          style={[styles.seedText, styles.t_parityS, styles.pinInput, {minHeight: 30, marginBottom: 0}, isValidPath ? {}: styles.seedText_invalid]}
        />
      }
    </View>
  )
}
