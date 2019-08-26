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
    <>
      <TouchableOpacity
        onPress={toggleShowAdvancedField}
        style={{diplay:'flex'}}
      >
        <View
          style={{justifyContent:'center'}}
        >
          <Text style={[styles.title, ownStyles.advancedText]}>
            ADVANCED
            <Icon 
              name={showAdvancedField ? 'arrow-drop-up' : 'arrow-drop-down'}
              size={20}
            />
          </Text>
        </View>
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
          style={isValidPath ? ownStyles.validInput: ownStyles.invalidInput}
        />
      }
    </>
  )
}



const ownStyles = StyleSheet.create({
  advancedText: {
    paddingBottom: 0,
    paddingTop:20
  },
  invalidInput: {
    backgroundColor: '#fee3e3'
  },
  validInput: {
    backgroundColor: '#e4fee4'
  }
});
