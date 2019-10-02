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
  dbKey: string, // this is the key that is used in the db to save / delete.
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
  selectedKey: string
};


export default class AccountsStore extends Container {
  state = {
    accounts: new Map(),
    newAccount: empty(),
    selectedKey: ''
  };

  constructor(props) {
    super(props);
    this.refreshList();
  }

  async select(accountKey) {
    this.setState({ selectedKey: accountKey });
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
      account.dbKey = accountId(account);
      this.setState({
        accounts: this.state.accounts.set(accountId(account), account),
        newAccount: empty()
      });
    }
  }
  
  updateAccount(accountKey, updatedAccount) {
    const accounts = this.state.accounts;
    const account = accounts.get(accountKey);

    if (account && updatedAccount) {
      this.setState({ accounts: accounts.set(accountKey, {...account, ...updatedAccount}) });
    }
    // debugger;
    // Object.assign(account, accountUpdate);
    // this.setState({});
  }

  updateSelectedAccount(updatedAccount) {
    this.updateAccount(this.state.selectedKey, updatedAccount)
  }

  async refreshList() {
    // loadAccounts().then(accounts => {
        // .then(accounts =>
        // Object.values(accounts).map(account => JSON.parse(account))
        // );

        // let accounts = new Map();
        // for (let [key, value] of Object.entries(res)) {
        //   const account = JSON.parse(value)
        //   accounts.set(key, {...account, dbKey: key})
        // }

      // const accounts = Object.entries(res).map(() => {
      //   new Map(res.map(a => [accountId(a), a]));
      // }) 
    loadAccounts().then(accounts => {
      // const accounts = new Map(res.map(a => [accountId(a), a]));
      // console.log('accounts refreshList',accounts)
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

    accounts.delete(account.dbKey);
    this.setState({ accounts });
    await deleteDbAccount(account);
  }

  async unlockAccount(accountKey, pin) {
    const {accounts} = this.state;
    const account = accounts.get(accountKey);

    if (!accountKey || !account || !account.encryptedSeed) {
      return false;
    }

    try {
      account.seed = await decryptData(account.encryptedSeed, pin);
      const {phrase, derivePath, password} = parseSURI(account.seed)

      account.seedPhrase = phrase || '';
      account.derivationPath = derivePath || '';
      account.derivationPassword = password || '';
      this.setState({
        accounts: this.state.accounts.set(accountKey, account)
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

  lockAccount(accountKey) {
    const {accounts} = this.state
    const account = accounts.get(accountKey);

    if (account) {
      const lockedAccount = this.deleteSensitiveData(account)
      this.setState({
        accounts: this.state.accounts.set(accountKey, lockedAccount)
      });
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
    //const acc = this.state.accounts[account.dbKey] || empty(account.address, account.networkKey)
    return this.state.accounts.get(accountId(account)) || empty(account.address, account.networkKey);
  }

  getByAddress(address) {
  //   const account = this.getAccounts().find(
  //     a => a.address.toLowerCase() === address.toLowerCase()
  //  );

   for (let v of this.state.accounts.values()) {
      if (v.address.toLowerCase() === address.toLowerCase()){
        return v;
      } 
    }

    throw new Error(`no account found for the address: ${address}`);
}

  getSelected() {
    return this.state.accounts.get(this.state.selectedKey);
  }

  getSelectedKey() {
    return this.state.selectedKey
  }

  getAccounts() {
    // console.log('this.state.accounts',this.state.accounts)
    return this.state.accounts
    // return Array.from(this.state.accounts.values())
    //   .filter(a => !!a.networkKey)
    //   .sort((a, b) => {
    //     if (a.name < b.name) {
    //       return -1;
    //     }
    //     if (a.name > b.name) {
    //       return 1;
    //     }
    //     return 0;
    //   });
  }
}
