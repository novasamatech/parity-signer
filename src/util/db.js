'use strict'

import SecureStorage from 'react-native-secure-storage'

const accountsStore = {
  keychainService: 'accounts',
  sharedPreferencesName: 'accounts',
}

export const deleteAccount = (account) => SecureStorage.deleteItem(account.address, accountsStore)

export const saveAccount = (account) =>
  SecureStorage.setItem(account.address, JSON.stringify(account, null, 0), accountsStore)

export const saveAccounts = (accounts) => accounts.forEach(saveAccount)

export const loadAccounts = () => SecureStorage.getAllItems(accountsStore).then(
  accounts => Object.values(accounts).map(account => JSON.parse(account))
)
