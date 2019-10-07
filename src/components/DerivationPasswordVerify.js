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

import colors from '../colors';
import fonts from "../fonts";
import styles from "../styles";
import TextInput from './TextInput';

  export default function DerivationPasswordVerify(props) {
    const { password } = props;
    const [enteredPassword, setEnteredPassword] =  useState('')
    const [verifyField, setVerifyField] = useState(false);
    const isMatching = enteredPassword === password;

    const toggleVerifyField = () => {
      setVerifyField(!verifyField)
    }
  
    return (
      <>
        <TouchableOpacity
          onPress={toggleVerifyField}
        >
          <Text style={[styles.t_hintText, {marginTop:16}]}>
            <Icon name={'info'} size={15} color={colors.bg_text_sec} />
            {' '}This account countains a derivation password.{' '}
            <Text style={styles.t_underline}  onPress={toggleVerifyField} >Verify it here </Text>
            <Icon 
              name={verifyField ? 'arrow-drop-up' : 'arrow-drop-down'}
              size={20}
            />
          </Text>
        </TouchableOpacity>
        {verifyField && 
          <>
            <TextInput
              onChangeText={setEnteredPassword}
              placeholder="derivation password"
              style={[styles.seedText, styles.t_parityS, styles.pinInput, {minHeight: 30, marginBottom: 0}, isMatching ? {}: styles.seedText_invalid]}
            />
          </>
        }
      </>
    )
  }
