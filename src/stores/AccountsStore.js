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

import { accountId, emptyAccount } from '../util/account';
import {
	loadAccounts,
	saveAccount,
	deleteAccount as deleteDbAccount
} from '../util/db';
import { parseSURI } from '../util/suri';
import { decryptData, encryptData } from '../util/native';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import type { AccountsStoreState } from './types';
import { emptyIdentity } from '../util/identity';

export default class AccountsStore extends Container<AccountsStoreState> {
	state = {
		accounts: new Map(),
		identities: [],
		newAccount: emptyAccount(),
		newIdentity: emptyIdentity(),
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
		this.setState({
			newAccount: { ...this.state.newAccount, ...accountUpdate }
		});
	}

	updateNewIdentity(identityUpdate) {
		this.setState({
			newIdentity: { ...this.state.newIdentity, ...identityUpdate }
		});
	}

	getNew() {
		return this.state.newAccount;
	}

	getNewIdentity() {
		return this.state.newIdentity;
	}

	async submitNew(pin) {
		const account = this.state.newAccount;
		if (!account.seed) return;

		const { protocol } = NETWORK_LIST[account.networkKey];

		if (protocol === NetworkProtocols.SUBSTRATE) {
			// TODO unlock save into identities;
			// const lockedAccount = this.encryptSeedPhraseAndLockAccount(account, pin);
		} else {
			await this.save(accountId(account), account, pin);
		}
		// only save a new account if the seed isn't empty

		this.setState({
			accounts: this.state.accounts.set(accountId(account), account),
			newAccount: emptyAccount()
		});
	}

	updateAccount(accountKey, updatedAccount) {
		const accounts = this.state.accounts;
		const account = accounts.get(accountKey);

		if (account && updatedAccount) {
			this.setState({
				accounts: accounts.set(accountKey, { ...account, ...updatedAccount })
			});
		}
	}

	updateSelectedAccount(updatedAccount) {
		this.updateAccount(this.state.selectedKey, updatedAccount);
	}

	async refreshList() {
		loadAccounts().then(accounts => {
			this.setState({ accounts });
		});
	}

	async encryptSeedPhraseAndLockAccount(account, pin = null) {
		try {
			// for account creation
			if (pin && account.seedPhrase) {
				account.encryptedSeedPhrase = await encryptData(
					account.seedPhrase,
					pin
				);
			}
			const encryptedAccount = this.deleteSensitiveData(account);

			encryptedAccount.updatedAt = new Date().getTime();
			return encryptedAccount;
		} catch (e) {
			console.error(e);
		}
	}

	async save(accountKey, account, pin = null) {
		try {
			// for account creation
			if (pin && account.seed) {
				account.encryptedSeed = await encryptData(account.seed, pin);
			}

			const accountToSave = this.deleteSensitiveData(account);

			accountToSave.updatedAt = new Date().getTime();
			await saveAccount(accountKey, accountToSave);
		} catch (e) {
			console.error(e);
		}
	}

	async deleteAccount(accountKey) {
		const { accounts } = this.state;

		accounts.delete(accountKey);
		this.setState({ accounts, selectedKey: '' });
		await deleteDbAccount(accountKey);
	}

	async unlockAccount(accountKey, pin) {
		const { accounts } = this.state;
		const account = accounts.get(accountKey);

		if (!accountKey || !account || !account.encryptedSeed) {
			return false;
		}

		try {
			account.seed = await decryptData(account.encryptedSeed, pin);
			const { phrase, derivePath, password } = parseSURI(account.seed);

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

	deleteSensitiveData(account) {
		delete account.seed;
		delete account.seedPhrase;
		delete account.derivationPassword;
		delete account.derivationPath;

		return account;
	}

	lockAccount(accountKey) {
		const { accounts } = this.state;
		const account = accounts.get(accountKey);

		if (account) {
			const lockedAccount = this.deleteSensitiveData(account);
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
		return (
			this.state.accounts.get(accountId(account)) ||
			emptyAccount(account.address, account.networkKey)
		);
	}

	getByAddress(address) {
		if (!address) {
			return false;
		}

		for (let v of this.state.accounts.values()) {
			if (v.address.toLowerCase() === address.toLowerCase()) {
				return v;
			}
		}

		return false;
	}

	getSelected() {
		return this.state.accounts.get(this.state.selectedKey);
	}

	getSelectedKey() {
		return this.state.selectedKey;
	}

	getAccounts() {
		return this.state.accounts;
	}

	getLegacySubstrateAccounts() {
		const result = [];
		const accounts = this.state.accounts[Symbol.iterator]();
		for (let [key, value] of accounts) {
			if (key.split(':')[0] === NetworkProtocols.SUBSTRATE) {
				result.push([key, value]);
			}
		}
		return result;
	}
}
