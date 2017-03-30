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

'use strict'

import {
  ADD_ACCOUNT, SELECT_ACCOUNT, DELETE_ACCOUNT, MODIFY_ACCOUNT, SET_NEW_PIN, SET_ACCOUNTS
} from '../constants/AccountActions'

export function addAccount (account) {
  return {
    type: ADD_ACCOUNT,
    account
  }
}

export function selectAccount (account) {
  return {
    type: SELECT_ACCOUNT,
    account
  }
}

export function deleteAccount (account) {
  return {
    type: DELETE_ACCOUNT,
    account
  }
}

export function modifyAccount (account, modifications) {
  return {
    type: MODIFY_ACCOUNT,
    account,
    modifications
  }
}

export function setNewPin (pin) {
  return {
    type: SET_NEW_PIN,
    pin
  }
}

export function setAccounts (accounts) {
  return {
    type: SET_ACCOUNTS,
    accounts
  }
}
