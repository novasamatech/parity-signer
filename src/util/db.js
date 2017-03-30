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

import SecureStorage from 'react-native-secure-storage'

const accountsStore = {
  keychainService: 'accounts',
  sharedPreferencesName: 'accounts'
}

export const deleteAccount = (account) => SecureStorage.deleteItem(account.address, accountsStore)

export const saveAccount = (account) =>
  SecureStorage.setItem(account.address, JSON.stringify(account, null, 0), accountsStore)

export const saveAccounts = (accounts) => accounts.forEach(saveAccount)

export const loadAccounts = () => {
  if (!SecureStorage) {
    return Promise.resolve([])
  }

  return SecureStorage.getAllItems(accountsStore).then(
    accounts => Object.values(accounts).map(account => JSON.parse(account))
  )
}
