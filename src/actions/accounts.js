import { ADD_ACCOUNT, SELECT_ACCOUNT } from '../constants/AccountActions'

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
