import { ADD_ACCOUNT } from '../constants/AccountActions'

export function addAccount(account) {
  return {
    type: ADD_ACCOUNT,
    address: account.address,
  }
}
