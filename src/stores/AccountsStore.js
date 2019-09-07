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
import { loadAccounts, saveAccount, deleteAccount as deleteDbAccount} from '../util/db';
import {parseSURI} from '../util/suri'
import { decryptData, encryptData } from '../util/native';

export type Account = {
  address: string,
  createdAt: number,
  derivationPassword: string,
  derivationPath: string, // doesn't contain the ///password
  encryptedSeed: string,
  name: string,
  networkKey: string,
  seed: string, //this is the SURI (seedPhrase + /soft//hard///password derivation)
  seedPhrase: string, //contains only the BIP39 words, no derivation path
  updatedAt: number,
  validBip39Seed: boolean
};

type AccountsState = {
  accounts: Map<string, Account>,
  newAccount: Account,
  selected: Account
};


export default class AccountsStore extends Container {
  state = {
    accounts: new Map(),
    newAccount: empty(),
    selected: undefined
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

  updateNew(accountUpdate) {
    this.setState({ newAccount : {...this.state.newAccount, ...accountUpdate} })
  }

  getNew() {
    return this.state.newAccount;
  }

  async submitNew(pin) {
    const account = this.state.newAccount;

    // only save a new account if the seed isn't empty
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

  async save(account, pin = null) {
    try {
      // for account creation
      if (pin && account.seed) {
        account.encryptedSeed = await encryptData(account.seed, pin);
      }

      const accountToSave = this.deleteSensitiveData(account);

      accountToSave.updatedAt = new Date().getTime();
      await saveAccount(accountToSave);
    } catch (e) {
      console.error(e);
    }
  }


  async deleteAccount(account) {
    const { accounts } = this.state;

    accounts.delete(accountId(account));
    this.setState({ accounts });
    await deleteDbAccount(account);
  }

  async unlockAccount(account, pin) {
    if (!account || !account.encryptedSeed) {
      return false;
    }

    try {
      account.seed = await decryptData(account.encryptedSeed, pin);

      const {phrase, derivePath, password} = parseSURI(account.seed)

      account.seedPhrase = phrase || '';
      account.derivationPath = derivePath || '';
      account.derivationPassword = password || '';
      this.setState({
        accounts: this.state.accounts.set(accountId(account), account)
      });
    } catch (e) {
      return false;
    }
    return true;
  }

  deleteSensitiveData (account){
    delete account.seed;
    delete account.seedPhrase;
    delete account.derivationPassword;
    delete account.derivationPath;

    return account
  }

  lockAccount(account) {
    const {accounts} = this.state

    if (accounts.get(accountId(account))) {
      const lockedAccount = this.deleteSensitiveData(account)
      accounts.set(accountId(account), lockedAccount);
      this.setState({ accounts });
    }
  }

  async checkPinForSelected(pin) {
    const account = this.getSelected();

    if (account && account.encryptedSeed) {
      return await decryptData(account.encryptedSeed, pin);
    } else {
      return false;
    }
  }

  getById(account) {
    return this.state.accounts.get(accountId(account)) || empty(account.address, account.networkKey);
  }

  getByAddress(address) {
    return this.getAccounts().find(
      a => a.address.toLowerCase() === address.toLowerCase()
    );
  }

  getSelected() {
    return this.state.accounts.get(this.state.selected);
  }

  getAccounts() {
    return Array.from(this.state.accounts.values())
      .filter(a => !!a.networkKey)
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
