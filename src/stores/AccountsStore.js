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
import {
	loadAccounts,
	saveAccount,
	deleteAccount as deleteDbAccount
} from '../util/db';
import { parseSURI } from '../util/suri';
import {
	decryptData,
	encryptData,
	secureGet,
	securePut,
	secureContains,
	secureDelete
} from '../util/native';
import { v4 } from 'react-native-uuid';
import { APP_ID } from '../constants.js';

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
	validBip39Seed: boolean,
	pinKey: string,
	biometricEnabled: boolean
};

type AccountsState = {
	accounts: Map<string, Account>,
	newAccount: Account,
	selectedKey: string
};

export default class AccountsStore extends Container<AccountsState> {
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
		this.setState({
			newAccount: { ...this.state.newAccount, ...accountUpdate }
		});
	}

	getNew() {
		return this.state.newAccount;
	}

	async submitNew(pin) {
		const account = this.state.newAccount;

		// only save a new account if the seed isn't empty
		if (account.seed) {
			const accountKey = accountId(account);

			await this.save(accountKey, account, pin);
			this.setState({
				accounts: this.state.accounts.set(accountKey, account),
				newAccount: empty()
			});
		}
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

	async save(accountKey, account, pin = null) {
		account = account ? account : this.state.accounts.get(accountKey);
		try {
			if (pin) {
				if (account.biometricEnabled) {
					await this.updateBiometric(accountKey, account, pin);
				}

				// for account creation
				if (account.seed) {
					account.encryptedSeed = await encryptData(account.seed, pin);
				}
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
		const account = accounts.get(accountKey);

		accounts.delete(accountKey);
		this.setState({ accounts, selectedKey: '' });
		await deleteDbAccount(accountKey);
		if (await secureContains(APP_ID, account.pinKey)) {
			await secureDelete(APP_ID, account.pinKey);
		}
	}

	async unlockAccountWithBiometric(accountKey) {
		const { accounts } = this.state;
		const account = accounts.get(accountKey);

		if (!account || !account.encryptedSeed || !account.biometricEnabled) {
			return false;
		}

		try {
			const keyExists = await secureContains(APP_ID, account.pinKey);
			if (!keyExists) {
				throw {
					message:
						'Invalid state: biometry is enabled but no corresponding key found in storage.'
				};
			} else {
				const pin = await secureGet(APP_ID, account.pinKey);
				account.seed = await decryptData(account.encryptedSeed, pin);

				const { phrase, derivePath, password } = parseSURI(account.seed);
				account.seedPhrase = phrase || '';
				account.derivationPath = derivePath || '';
				account.derivationPassword = password || '';
				this.setState({
					accounts: this.state.accounts.set(accountId(account), account)
				});
			}
		} catch (e) {
			throw e;
		}
		return true;
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
			empty(account.address, account.networkKey)
		);
	}

	getByAddress(address) {
		for (let v of this.state.accounts.values()) {
			if (v.address.toLowerCase() === address.toLowerCase()) {
				return v;
			}
		}

		throw new Error(`no account found for the address: ${address}`);
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

	async enableBiometric(accountKey, pin) {
		const account = this.state.accounts.get(accountKey);
		return securePut(APP_ID, account.pinKey, pin).then(() => {
			this.updateAccount(accountKey, { biometricEnabled: true });
			return this.save(accountKey);
		});
	}

	async disableBiometric(accountKey) {
		const account = this.state.accounts.get(accountKey);
		return secureDelete(APP_ID, account.pinKey)
			.catch(e => console.log(e))
			.finally(() => {
				// incase delete failed, overwrite the pinKey and forget the encrypted pin
				this.updateAccount(accountKey, {
					pinKey: v4(),
					biometricEnabled: false
				});
				return this.save(accountKey);
			});
	}

	async updateBiometric(accountKey, account, pin) {
		return secureDelete(APP_ID, account.pinKey)
			.catch(e => {
				console.log(e);
				account.pinKey = v4();
			})
			.finally(() => securePut(APP_ID, account.pinKey, pin));
	}
}
