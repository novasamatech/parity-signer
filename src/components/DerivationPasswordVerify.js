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
          style={{diplay:'flex'}}
        >
          <View
            style={{justifyContent:'center'}}
          >
            <Text style={styles.passwordText}>
              <Icon name={'info'} size={20} color={colors.bg_text_sec} />
              {' '}This account countains a derivation password.{' '}
              <Text style={styles.link}  onPress={toggleVerifyField} >Verify it here </Text>
              <Icon 
                name={verifyField ? 'arrow-drop-up' : 'arrow-drop-down'}
                size={20}
              />
            </Text>
          </View>
        </TouchableOpacity>
        {verifyField && 
          <>
            <TextInput
              onChangeText={setEnteredPassword}
              placeholder="derivation password"
              style={isMatching ? styles.validInput: styles.invalidInput}
            />
          </>
        }
      </>
    )
  }

  const styles = StyleSheet.create({
    link: {
      textDecorationLine: 'underline',
    },
    passwordText: {
      color: colors.bg_text_sec,
      fontFamily: fonts.regular,
      fontSize: 18,
      marginBottom: 10,
      marginTop: 20,
      paddingBottom: 0
    },
    invalidInput: {
      backgroundColor: '#fee3e3'
    },
    validInput: {
      backgroundColor: '#e4fee4'
    }
  });
