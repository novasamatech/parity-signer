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

// @flow

import { Container } from 'unstated';
import { loadAccounts, saveAccount, deleteAccount } from '../util/db';
import { encryptData, decryptData } from '../util/native';

export type Account = {
  name: string,
  address: string,
  seed: string,
  encryptedSeed: string,
  createdAt: number,
  updatedAt: number,
  archived: boolean
};

type AccountsState = {
  accounts: Map<string, Account>,
  newAccount: Account,
  selected: string
};

function empty(address = '') {
  return {
    name: '',
    address,
    seed: '',
    createdAt: new Date().getTime(),
    updatedAt: new Date().getTime(),
    archived: false,
    encryptedSeed: null
  };
}

export default class AccountsStore extends Container<AccountsState> {
  state = {
    accounts: new Map(),
    newAccount: empty(),
    selected: ''
  };

  constructor(props) {
    super(props);
    this.refreshList();
  }

  select(address) {
    this.setState({ selected: address.toLowerCase() });
  }

  updateNew(accountUpdate: Object) {
    Object.assign(this.state.newAccount, accountUpdate);
    this.setState({});
  }

  getNew(): Account {
    return this.state.newAccount;
  }

  submitNew() {
    this.setState({
      accounts: this.state.accounts.set(this.state.newAccount.address.toLowerCase(), this.state.newAccount),
      newAccount: empty()
    });
  }

  update(accountUpdate: { address: string }) {
    accountUpdate.address = accountUpdate.address.toLowerCase();
    let account = this.state.accounts.get(accountUpdate.address);
    if (!account) {
      this.state.accounts.set(accountUpdate.address.toLowerCase(), accountUpdate);
      account = this.state.accounts.get(accountUpdate.address);
    }
    Object.assign(account, accountUpdate);
    this.setState({});
  }

  updateSelected(accountUpdate) {
    this.update(Object.assign(this.getSelected(), accountUpdate));
  }

  async refreshList() {
    loadAccounts().then(res => {
      const accounts = new Map(res.map(a => [a.address.toLowerCase(), a]));
      this.setState({ accounts });
    });
  }

  async save(account, pin = null) {
    try {
      if (pin && account.seed) {
        let encryptedSeed = await encryptData(account.seed, pin);
        delete account.seed;
        account.encryptedSeed = encryptedSeed;
      }
      account.updatedAt = new Date().getTime();
      saveAccount(account);
    } catch (e) {
      console.error(e);
    }
  }

  async deleteAccount(account) {
    // deleteAccount(account)
    account.archived = true;
    this.state.accounts.set(account.address.toLowerCase(), account);
    this.setState({
      accounts: this.state.accounts
    });
    await this.save(account);
  }

  async saveSelected(pin) {
    await this.save(this.getSelected(), pin);
  }

  async unlockAccount(address, pin) {
    address = address.toLowerCase();
    const account = this.getByAddress(address);
    if (!account || !account.encryptedSeed) {
      return false;
    }
    try {
      account.seed = await decryptData(account.encryptedSeed, pin);
      this.setState({ accounts: this.state.accounts.set(address, account) });
    } catch (e) {
      return false;
    }
    return true;
  }

  async checkPinForSelected(pin) {
    const account = this.getSelected();
    if (account && account.encryptedSeed) {
      return await decryptData(account.encryptedSeed, pin);
    } else {
      return false;
    }
  }

  getByAddress(address: string): ?Account {
    return this.state.accounts.get(address.toLowerCase()) || empty(address);
  }

  getSelected(): ?Account {
    return this.state.accounts.get(this.state.selected);
  }

  getAccounts(): Array<Account> {
    return Array.from(this.state.accounts.values()).filter(a => !a.archived);
  }
}
