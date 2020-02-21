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

import { Container } from 'unstated';

import {
	ETHEREUM_NETWORK_LIST,
	NetworkProtocols,
	SUBSTRATE_NETWORK_LIST,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import { emptyAccount, generateAccountId } from 'utils/account';
import {
	loadAccounts,
	saveAccount,
	deleteAccount as deleteDbAccount,
	saveIdentities,
	loadIdentities
} from 'utils/db';
import { constructSURI, parseSURI } from 'utils/suri';
import {
	brainWalletAddress,
	decryptData,
	encryptData,
	substrateAddress
} from 'utils/native';
import {
	deepCopyIdentities,
	deepCopyIdentity,
	emptyIdentity,
	extractAddressFromAccountId,
	getAddressKeyByPath,
	getNetworkKey,
	isEthereumAccountId,
	parseFoundLegacyAccount
} from 'utils/identitiesUtils';
import {
	AccountsStoreState,
	Account,
	LockedAccount,
	UnlockedAccount,
	FoundAccount,
	Identity,
	FoundIdentityAccount,
	isUnlockedAccount
} from 'types/identityTypes';

export default class AccountsStore extends Container<AccountsStoreState> {
	state: AccountsStoreState = {
		accounts: new Map(),
		currentIdentity: null,
		identities: [],
		loaded: false,
		newAccount: emptyAccount(),
		newIdentity: emptyIdentity(),
		selectedKey: ''
	};

	constructor() {
		super();
		this.refreshList();
	}

	async select(accountKey: string): Promise<void> {
		await this.setState({ selectedKey: accountKey });
	}

	updateNew(accountUpdate: Partial<UnlockedAccount>): void {
		this.setState({
			newAccount: { ...this.state.newAccount, ...accountUpdate }
		});
	}

	getNew(): UnlockedAccount {
		return this.state.newAccount;
	}

	async submitNew(pin: string): Promise<void> {
		const account = this.state.newAccount;
		if (!account.seed) return;

		const accountKey = generateAccountId(account);
		await this.save(accountKey, account, pin);

		await this.setState({
			accounts: this.state.accounts.set(accountKey, account),
			newAccount: emptyAccount()
		});
	}

	async deriveEthereumAccount(
		seedPhrase: string,
		networkKey: string
	): Promise<boolean> {
		const networkParams = ETHEREUM_NETWORK_LIST[networkKey];
		const ethereumAddress = await brainWalletAddress(seedPhrase);
		if (ethereumAddress.address === '') return false;
		const { ethereumChainId } = networkParams;
		const accountId = generateAccountId({
			address: ethereumAddress.address,
			networkKey
		});
		if (this.state.currentIdentity === null) return false;
		const updatedCurrentIdentity = deepCopyIdentity(this.state.currentIdentity);
		if (updatedCurrentIdentity.meta.has(ethereumChainId)) return false;
		updatedCurrentIdentity.meta.set(ethereumChainId, {
			address: ethereumAddress.address,
			createdAt: new Date().getTime(),
			name: '',
			updatedAt: new Date().getTime()
		});
		updatedCurrentIdentity.addresses.set(accountId, ethereumChainId);
		return await this.updateCurrentIdentity(updatedCurrentIdentity);
	}

	async updateAccount(
		accountKey: string,
		updatedAccount: Partial<LockedAccount>
	): Promise<void> {
		const accounts = this.state.accounts;
		const account = accounts.get(accountKey);

		if (account && updatedAccount) {
			await this.setState({
				accounts: accounts.set(accountKey, { ...account, ...updatedAccount })
			});
		}
	}

	async updateSelectedAccount(
		updatedAccount: Partial<LockedAccount>
	): Promise<void> {
		await this.updateAccount(this.state.selectedKey, updatedAccount);
	}

	async refreshList(): Promise<void> {
		const accounts = await loadAccounts();
		const identities = await loadIdentities();
		let { currentIdentity } = this.state;
		if (identities.length > 0) currentIdentity = identities[0];
		await this.setState({
			accounts,
			currentIdentity,
			identities,
			loaded: true
		});
	}

	async save(
		accountKey: string,
		account: Account,
		pin: string | null = null
	): Promise<void> {
		try {
			// for account creation
			let accountToSave = account;
			if (pin && isUnlockedAccount(account)) {
				account.encryptedSeed = await encryptData(account.seed, pin);
				accountToSave = this.deleteSensitiveData(account);
			}

			await saveAccount(accountKey, accountToSave);
		} catch (e) {
			console.error(e);
		}
	}

	async deleteAccount(accountKey: string): Promise<void> {
		const { accounts } = this.state;

		accounts.delete(accountKey);
		await this.setState({ accounts, selectedKey: '' });
		await deleteDbAccount(accountKey);
	}

	async unlockAccount(accountKey: string, pin: string): Promise<boolean> {
		const { accounts } = this.state;
		const account = accounts.get(accountKey);

		if (!accountKey || !account || !account.encryptedSeed) {
			return false;
		}

		try {
			const decryptedSeed = await decryptData(account.encryptedSeed, pin);
			const { phrase, derivePath, password } = parseSURI(decryptedSeed);
			await this.setState({
				accounts: this.state.accounts.set(accountKey, {
					derivationPassword: password || '',
					derivationPath: derivePath || '',
					seed: decryptedSeed,
					seedPhrase: phrase || '',
					...account
				})
			});
		} catch (e) {
			return false;
		}
		return true;
	}

	deleteSensitiveData(account: UnlockedAccount): LockedAccount {
		delete account.seed;
		delete account.seedPhrase;
		delete account.derivationPassword;
		delete account.derivationPath;

		return account;
	}

	lockAccount(accountKey: string): void {
		const { accounts } = this.state;
		const account = accounts.get(accountKey);

		if (account && isUnlockedAccount(account)) {
			const lockedAccount = this.deleteSensitiveData(account);
			this.setState({
				accounts: this.state.accounts.set(accountKey, lockedAccount)
			});
		}
	}

	getAccountWithoutCaseSensitive(accountId: string): Account | null {
		let findLegacyAccount = null;
		for (const [key, value] of this.state.accounts) {
			if (isEthereumAccountId(accountId)) {
				if (key.toLowerCase() === accountId.toLowerCase()) {
					findLegacyAccount = value;
					break;
				}
			} else if (key === accountId) {
				findLegacyAccount = value;
				break;
			} else if (
				//backward compatible with hard spoon substrate key pairs
				extractAddressFromAccountId(key) ===
				extractAddressFromAccountId(accountId)
			) {
				findLegacyAccount = value;
				break;
			}
		}
		return findLegacyAccount;
	}

	getById({
		address,
		networkKey
	}: {
		address: string;
		networkKey: string;
	}): null | FoundAccount {
		const accountId = generateAccountId({ address, networkKey });
		const legacyAccount = this.getAccountWithoutCaseSensitive(accountId);
		if (legacyAccount) return parseFoundLegacyAccount(legacyAccount, accountId);
		let derivedAccount;
		//assume it is an accountId
		if (networkKey !== UnknownNetworkKeys.UNKNOWN) {
			derivedAccount = this.getAccountFromIdentity(accountId);
		} else {
			derivedAccount = this.getAccountFromIdentity(address);
		}

		if (derivedAccount instanceof Object)
			return { ...derivedAccount, isLegacy: false };
		return null;
	}

	getAccountFromIdentity(
		accountIdOrAddress: string
	): false | FoundIdentityAccount {
		const isAccountId = accountIdOrAddress.split(':').length > 1;
		let targetAccountId = null;
		let targetIdentity = null;
		let targetNetworkKey = null;
		let targetPath = null;
		for (const identity of this.state.identities) {
			const searchList = Array.from(identity.addresses.entries());
			for (const [addressKey, path] of searchList) {
				const networkKey = getNetworkKey(path, identity);
				let accountId, address;
				if (isEthereumAccountId(addressKey)) {
					accountId = addressKey;
					address = extractAddressFromAccountId(addressKey);
				} else {
					accountId = generateAccountId({ address: addressKey, networkKey });
					address = addressKey;
				}
				const searchAccountIdOrAddress = isAccountId ? accountId : address;
				const found = isEthereumAccountId(accountId)
					? searchAccountIdOrAddress.toLowerCase() ===
					  accountIdOrAddress.toLowerCase()
					: searchAccountIdOrAddress === accountIdOrAddress;
				if (found) {
					targetPath = path;
					targetIdentity = identity;
					targetAccountId = accountId;
					targetNetworkKey = networkKey;
					break;
				}
			}
		}

		if (
			targetPath === null ||
			targetIdentity === null ||
			targetAccountId === null
		)
			return false;
		this.setState({ currentIdentity: targetIdentity });

		const metaData = targetIdentity.meta.get(targetPath);
		if (metaData === undefined) return false;
		return {
			accountId: targetAccountId,
			encryptedSeed: targetIdentity.encryptedSeed,
			isLegacy: false,
			networkKey: targetNetworkKey!,
			path: targetPath,
			validBip39Seed: true,
			...metaData
		};
	}

	getAccountByAddress(address: string): false | FoundAccount {
		if (!address) {
			return false;
		}

		for (const [k, v] of this.state.accounts.entries()) {
			if (v.address.toLowerCase() === address.toLowerCase()) {
				return { ...v, accountId: k, isLegacy: true };
			}
		}
		return this.getAccountFromIdentity(address);
	}

	getSelected(): Account | undefined {
		return this.state.accounts.get(this.state.selectedKey);
	}

	getSelectedKey(): string {
		return this.state.selectedKey;
	}

	getAccounts(): Map<string, Account> {
		return this.state.accounts;
	}

	getIdentityByAccountId(accountId: string): Identity | undefined {
		const networkProtocol = accountId.split(':')[0];
		const searchAddress =
			networkProtocol === NetworkProtocols.SUBSTRATE
				? extractAddressFromAccountId(accountId)
				: accountId;
		return this.state.identities.find(identity =>
			identity.addresses.has(searchAddress)
		);
	}

	getNewIdentity(): Identity {
		return this.state.newIdentity;
	}

	async resetCurrentIdentity(): Promise<void> {
		await this.setState({ currentIdentity: null });
	}

	async _addPathToIdentity(
		newPath: string,
		seedPhrase: string,
		updatedIdentity: Identity,
		name: string,
		networkKey: string
	): Promise<boolean> {
		const { prefix, pathId } = SUBSTRATE_NETWORK_LIST[networkKey];
		const suri = constructSURI({
			derivePath: newPath,
			password: '',
			phrase: seedPhrase
		});
		let address = '';
		try {
			address = await substrateAddress(suri, prefix);
		} catch (e) {
			return false;
		}
		if (address === '') return false;
		if (updatedIdentity.meta.has(newPath)) return false;
		const pathMeta = {
			address,
			createdAt: new Date().getTime(),
			name,
			networkPathId: pathId,
			updatedAt: new Date().getTime()
		};
		updatedIdentity.meta.set(newPath, pathMeta);
		updatedIdentity.addresses.set(address, newPath);
		return true;
	}

	async saveNewIdentity(seedPhrase: string, pin: string): Promise<void> {
		const updatedIdentity = deepCopyIdentity(this.state.newIdentity);
		//TODO encrypt seedPhrase with password in the future version,
		// current encryption with only seedPhrase is compatible.
		updatedIdentity.encryptedSeed = await encryptData(seedPhrase, pin);
		const newIdentities = this.state.identities.concat(updatedIdentity);
		await this.setState({
			currentIdentity: updatedIdentity,
			identities: newIdentities,
			newIdentity: emptyIdentity()
		});
		await saveIdentities(newIdentities);
	}

	async selectIdentity(identity: Identity): Promise<void> {
		await this.setState({ currentIdentity: identity });
	}

	updateNewIdentity(identityUpdate: Partial<Identity>): void {
		this.setState({
			newIdentity: { ...this.state.newIdentity, ...identityUpdate }
		});
	}

	async updateCurrentIdentity(updatedIdentity: Identity): Promise<boolean> {
		try {
			await this.setState({
				currentIdentity: updatedIdentity
			});
			await this._updateIdentitiesWithCurrentIdentity();
		} catch (e) {
			console.warn('derive new Path error', e);
			return false;
		}
		return true;
	}

	async _updateIdentitiesWithCurrentIdentity(): Promise<void> {
		const newIdentities = deepCopyIdentities(this.state.identities);
		if (this.state.currentIdentity === null) return;
		const identityIndex = newIdentities.findIndex(
			(identity: Identity) =>
				identity.encryptedSeed === this.state.currentIdentity!.encryptedSeed
		);
		newIdentities.splice(identityIndex, 1, this.state.currentIdentity);
		await this.setState({ identities: newIdentities });
		await saveIdentities(newIdentities);
	}

	async updateIdentityName(name: string): Promise<void> {
		const updatedCurrentIdentity = deepCopyIdentity(
			this.state.currentIdentity!
		);
		updatedCurrentIdentity.name = name;
		try {
			await this.setState({ currentIdentity: updatedCurrentIdentity });
			await this._updateIdentitiesWithCurrentIdentity();
		} catch (e) {
			console.warn('update identity name error', e);
		}
	}

	async updatePathName(path: string, name: string): Promise<void> {
		const updatedCurrentIdentity = deepCopyIdentity(
			this.state.currentIdentity!
		);
		const updatedPathMeta = Object.assign(
			{},
			updatedCurrentIdentity.meta.get(path),
			{ name }
		);
		updatedCurrentIdentity.meta.set(path, updatedPathMeta);
		try {
			await this.setState({ currentIdentity: updatedCurrentIdentity });
			await this._updateIdentitiesWithCurrentIdentity();
		} catch (e) {
			console.warn('update path name error', e);
		}
	}

	async deriveNewPath(
		newPath: string,
		seedPhrase: string,
		networkKey: string,
		name: string
	): Promise<boolean> {
		const updatedCurrentIdentity = deepCopyIdentity(
			this.state.currentIdentity!
		);
		const deriveSucceed = await this._addPathToIdentity(
			newPath,
			seedPhrase,
			updatedCurrentIdentity,
			name,
			networkKey
		);
		if (!deriveSucceed) return false;
		return await this.updateCurrentIdentity(updatedCurrentIdentity);
	}

	async deletePath(path: string): Promise<boolean> {
		if (this.state.currentIdentity === null) return false;
		const updatedCurrentIdentity = deepCopyIdentity(this.state.currentIdentity);
		const { address } = updatedCurrentIdentity.meta.get(path)!;
		updatedCurrentIdentity.meta.delete(path);
		updatedCurrentIdentity.addresses.delete(getAddressKeyByPath(address, path));

		try {
			await this.setState({
				currentIdentity: updatedCurrentIdentity
			});
			await this._updateIdentitiesWithCurrentIdentity();
		} catch (e) {
			console.warn('derive new Path error', e);
			return false;
		}
		return true;
	}

	async deleteCurrentIdentity(): Promise<boolean> {
		try {
			const newIdentities = deepCopyIdentities(this.state.identities);
			const identityIndex = newIdentities.findIndex(
				(identity: Identity) =>
					identity.encryptedSeed === this.state.currentIdentity!.encryptedSeed
			);
			newIdentities.splice(identityIndex, 1);
			await this.setState({
				currentIdentity: newIdentities.length >= 1 ? newIdentities[0] : null,
				identities: newIdentities
			});
			await saveIdentities(newIdentities);
			return true;
		} catch (e) {
			return false;
		}
	}
}
