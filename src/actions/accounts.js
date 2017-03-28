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
