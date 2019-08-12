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

// @flow

import { Container } from 'unstated';
import { accountId, empty } from '../util/account';
import { loadAccounts, saveAccount } from '../util/db';
import { decryptData, encryptData } from '../util/native';

export type Account = {
  address: string,
  archived: boolean,
  createdAt: number,
  encryptedSeed: string,
  name: string,
  networkKey: string,
  protocol: string,
  seed: string,
  updatedAt: number,
  validBip39Seed: boolean
};

type AccountsState = {
  accounts: Map<string, Account>,
  newAccount: Account,
  selected: string,
  accountTxs: [Object]
};

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

  async select(account) {
    return new Promise((res, rej) => {
      this.setState(
        state => ({ selected: accountId(account) }),
        state => {
          res(state);
        }
      );
    });
  }

  updateNew(accountUpdate: Object) {
    this.setState({ newAccount : {...this.state.newAccount, ...accountUpdate} })
  }

  getNew(): Account {
    return this.state.newAccount;
  }

  async submitNew(pin) {
    const account = this.state.newAccount;

    // only save the account if the seed isn't empty
    if (account.seed) {
      await this.save(account, pin);
      this.setState({
        accounts: this.state.accounts.set(accountId(account), account),
        newAccount: empty()
      });
    }
  }

  update(accountUpdate) {
    let account = this.state.accounts.get(accountId(accountUpdate));
    if (!account) {
      this.state.accounts.set(accountId(accountUpdate), accountUpdate);
      account = this.state.accounts.get(accountId(accountUpdate));
    }
    Object.assign(account, accountUpdate);
    this.setState({});
  }

  updateSelected(accountUpdate) {
    this.update(Object.assign(this.getSelected(), accountUpdate));
  }

  async refreshList() {
    loadAccounts().then(res => {
      const accounts = new Map(res.map(a => [accountId(a), a]));
      this.setState({ accounts });
    });
  }

  async loadAccountTxs() { }

  async save(account, pin = null) {
    try {
      //only save an account if the seed isn't empty
      if (account.seed === ''){
        return;
      }

      if (pin && account.seed) {
        let encryptedSeed = await encryptData(account.seed, pin);
        delete account.seed;
        account.encryptedSeed = encryptedSeed;
      }

      account.updatedAt = new Date().getTime();
      await saveAccount(account);
    } catch (e) {
      console.error(e);
    }
  }

  async deleteAccount(account) {
    account.archived = true;
    this.state.accounts.set(accountId(account), account);
    this.setState({
      accounts: this.state.accounts
    });
    await this.save(account);
  }

  async saveSelected(pin) {
    await this.save(this.getSelected(), pin);
  }

  async unlockAccount(account, pin) {
    if (!account || !account.encryptedSeed) {
      return false;
    }
    try {
      account.seed = await decryptData(account.encryptedSeed, pin);
      this.setState({
        accounts: this.state.accounts.set(accountId(account), account)
      });
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

  getById(account): ?Account {
    return this.state.accounts.get(accountId(account)) || empty(account);
  }

  getByAddress(address): ?Account {
    const accounts = this.getAccounts();
    return accounts.find(
      a => a.address.toLowerCase() === address.toLowerCase()
    );
  }

  getSelected(): ?Account {
    return this.state.accounts.get(this.state.selected);
  }

  getAccounts(): Array<Account> {
    return Array.from(this.state.accounts.values())
      .filter(a => !a.archived && a.networkKey)
      .sort((a, b) => {
        if (a.name < b.name) {
          return -1;
        }
        if (a.name > b.name) {
          return 1;
        }
        return 0;
      });
  }
}
