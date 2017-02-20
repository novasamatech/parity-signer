'use strict'

import { ADD_ACCOUNT, SELECT_ACCOUNT, DELETE_ACCOUNT, SET_PIN, CONFIRM_PIN } from '../constants/AccountActions'

export function addAccount(account) {
  return {
    type: ADD_ACCOUNT,
    account,
  }
}

export function selectAccount(account) {
  return {
    type: SELECT_ACCOUNT,
    account,
  }
}

export function deleteAccount(account) {
  return {
    type: DELETE_ACCOUNT,
    account,
  }
}

export function setPin(pin) {
  return {
    type: SET_PIN,
    pin,
  }
}
