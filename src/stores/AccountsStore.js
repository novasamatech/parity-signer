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

import {
	accountId,
	emptyAccount,
	extractAddressFromAccountId
} from '../util/account';
import {
	loadAccounts,
	saveAccount,
	deleteAccount as deleteDbAccount,
	saveIdentities,
	loadIdentities
} from '../util/db';
import { constructSURI, parseSURI } from '../util/suri';
import {
	brainWalletAddress,
	decryptData,
	encryptData,
	substrateAddress
} from '../util/native';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import type { AccountsStoreState } from './types';
import {
	deepCopyIdentities,
	deepCopyIdentity,
	emptyIdentity,
	getNetworkKeyByPath
} from '../util/identitiesUtils';

export default class AccountsStore extends Container<AccountsStoreState> {
	state = {
		accounts: new Map(),
		currentIdentity: null,
		identities: [],
		newAccount: emptyAccount(),
		newIdentity: emptyIdentity(),
		selectedKey: ''
	};

	constructor(props) {
		super(props);
		this.refreshList();
	}

	select(accountKey) {
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

	async deriveEthereumAccount(seed, networkKey) {
		const networkParams = NETWORK_LIST[networkKey];
		const ethereumAddress = await brainWalletAddress(seed);
		if (ethereumAddress === '') return false;
		const { ethereumChainId } = networkParams;
		const accountAddress = accountId({
			address: ethereumAddress.address,
			networkKey
		});
		const updatedCurrentIdentity = deepCopyIdentity(this.state.currentIdentity);
		if (updatedCurrentIdentity.meta.has(ethereumChainId)) return false;
		updatedCurrentIdentity.meta.set(ethereumChainId, {
			address: accountAddress,
			createdAt: new Date().getTime(),
			name: '',
			updatedAt: new Date().getTime()
		});
		updatedCurrentIdentity.addresses.set(accountAddress, ethereumChainId);
		return await this.updateCurrentIdentity(updatedCurrentIdentity);
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
		const accounts = await loadAccounts();
		const identities = await loadIdentities();
		this.setState({ accounts, identities });
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

	async getById({ address, networkKey }) {
		const generateAccountId = accountId({ address, networkKey });
		const legacyAccount = this.state.accounts.get(generateAccountId);
		if (legacyAccount) return;
		return (
			this.getAccountFromIdentity(generateAccountId) ||
			emptyAccount(address, networkKey)
		);
	}

	async getAccountFromIdentity(address) {
		const isAccountId = address.split(':').length > 1;
		const mapFunction = isAccountId ? i => i : extractAddressFromAccountId;
		let targetPath = null;
		let targetIdentity = null;
		for (const identity of this.state.identities) {
			const addressIndex = Array.from(identity.addresses.keys())
				.map(mapFunction)
				.indexOf(address);
			if (addressIndex !== -1) {
				await this.setState({ currentIdentity: identity });
				targetPath = Array.from(identity.addresses.values())[addressIndex];
				targetIdentity = identity;
				break;
			}
		}
		if (!targetPath || !targetIdentity) return false;

		const metaData = targetIdentity.meta.get(targetPath);
		const networkKey = getNetworkKeyByPath(targetPath);
		return {
			...metaData,
			encryptedSeed: targetIdentity.encryptedSeed,
			isBip39: true,
			isLegacy: false,
			networkKey
		};
	}

	async getAccountByAddress(address) {
		if (!address) {
			return false;
		}

		for (let v of this.state.accounts.values()) {
			if (v.address.toLowerCase() === address.toLowerCase()) {
				return { ...v, isLegacy: true };
			}
		}
		return await this.getAccountFromIdentity(address);
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

	async unlockIdentitySeed(pin) {
		const { encryptedSeed } = this.state.currentIdentity;
		const seed = await decryptData(encryptedSeed, pin);
		const { phrase } = parseSURI(seed);
		return phrase;
	}

	getNewIdentity() {
		return this.state.newIdentity;
	}

	async saveNewIdentity(seedPhrase, pin) {
		const updatedIdentity = deepCopyIdentity(this.state.newIdentity);
		updatedIdentity.encryptedSeed = await encryptData(seedPhrase, pin);
		const newIdentities = this.state.identities.concat(updatedIdentity);
		this.setState({
			currentIdentity: updatedIdentity,
			identities: newIdentities,
			newIdentity: emptyIdentity()
		});
		await saveIdentities(newIdentities);
	}

	async selectIdentity(identity) {
		await this.setState({ currentIdentity: identity });
	}

	updateNewIdentity(identityUpdate) {
		this.setState({
			newIdentity: { ...this.state.newIdentity, ...identityUpdate }
		});
	}

	async updateCurrentIdentity(updatedIdentity) {
		try {
			await this.setState({
				currentIdentity: updatedIdentity
			});
			await this.updateIdentitiesWithCurrentIdentity();
		} catch (e) {
			console.warn('derive new Path error', e);
			return false;
		}
		return true;
	}

	async updateIdentitiesWithCurrentIdentity() {
		const newIdentities = deepCopyIdentities(this.state.identities);
		const identityIndex = newIdentities.findIndex(
			identity =>
				identity.encryptedSeed === this.state.currentIdentity.encryptedSeed
		);
		newIdentities.splice(identityIndex, 1, this.state.currentIdentity);
		this.setState({ identities: newIdentities });
		await saveIdentities(newIdentities);
	}

	async updateIdentityName(name) {
		const updatedCurrentIdentity = deepCopyIdentity(this.state.currentIdentity);
		updatedCurrentIdentity.name = name;
		try {
			await this.setState({ currentIdentity: updatedCurrentIdentity });
			await this.updateIdentitiesWithCurrentIdentity();
		} catch (e) {
			console.warn('update identity name error', e);
		}
	}

	async updatePathName(path, name) {
		const updatedCurrentIdentity = deepCopyIdentity(this.state.currentIdentity);
		const updatedPathMeta = Object.assign(
			{},
			updatedCurrentIdentity.meta.get(path),
			{ name }
		);
		updatedCurrentIdentity.meta.set(path, updatedPathMeta);
		try {
			await this.setState({ currentIdentity: updatedCurrentIdentity });
			await this.updateIdentitiesWithCurrentIdentity();
		} catch (e) {
			console.warn('update path name error', e);
		}
	}

	async deriveNewPath(newPath, seed, prefix, networkKey) {
		const updatedCurrentIdentity = deepCopyIdentity(this.state.currentIdentity);
		const suri = constructSURI({
			derivePath: newPath,
			password: '',
			phrase: seed
		});
		const address = await substrateAddress(suri, prefix);
		if (address === '') return false;
		if (updatedCurrentIdentity.meta.has(newPath)) return false;
		const accountAddress = accountId({ address, networkKey });
		updatedCurrentIdentity.meta.set(newPath, {
			address: accountAddress,
			createdAt: new Date().getTime(),
			name: '',
			updatedAt: new Date().getTime()
		});
		updatedCurrentIdentity.addresses.set(accountAddress, newPath);
		return await this.updateCurrentIdentity(updatedCurrentIdentity);
	}

	async deletePath(path) {
		const updatedCurrentIdentity = deepCopyIdentity(this.state.currentIdentity);
		const { address } = updatedCurrentIdentity.meta.get(path);
		updatedCurrentIdentity.meta.delete(path);
		updatedCurrentIdentity.addresses.delete(address);
		try {
			await this.setState({
				currentIdentity: updatedCurrentIdentity
			});
			await this.updateIdentitiesWithCurrentIdentity();
		} catch (e) {
			console.warn('derive new Path error', e);
			return false;
		}
		return true;
	}
}
