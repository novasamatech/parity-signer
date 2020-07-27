import { default as React, useEffect, useReducer } from 'react';

import {
	ETHEREUM_NETWORK_LIST,
	NetworkProtocols,
	SUBSTRATE_NETWORK_LIST,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import { defaultGlobalState } from 'stores/globalStateContext';
import {
	Account,
	AccountsStoreState,
	FoundAccount,
	FoundIdentityAccount,
	Identity,
	isUnlockedAccount,
	LockedAccount,
	UnlockedAccount
} from 'types/identityTypes';
import { emptyAccount, generateAccountId } from 'utils/account';
import {
	deleteAccount as deleteDbAccount,
	loadAccounts,
	loadIdentities,
	saveAccount,
	saveIdentities
} from 'utils/db';
import {
	accountExistedError,
	addressGenerateError,
	duplicatedIdentityError,
	emptyIdentityError,
	identityUpdateError
} from 'utils/errors';
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
	brainWalletAddressWithRef,
	decryptData,
	encryptData
} from 'utils/native';
import {
	CreateSeedRefWithNewSeed,
	TryBrainWalletAddress,
	TrySubstrateAddress
} from 'utils/seedRefHooks';
import { constructSuriSuffix, parseSURI } from 'utils/suri';

export type AccountsContextState = {
	clearIdentity: any;
	state: AccountsStoreState;
	select: any;
	updateNew: any;
	getNew: any;
	submitNew: any;
	deriveEthereumAccount: any;
	updateSelectedAccount: any;
	refreshList: any;
	save: any;
	deleteAccount: any;
	unlockAccount: any;
	deleteSensitiveData: any;
	lockAccount: any;
	getById: any;
	getAccountByAddress: any;
	getSelected: any;
	getSelectedKey: any;
	getAccounts: () => Map<string, UnlockedAccount | LockedAccount>;
	getIdentityByAccountId: any;
	resetCurrentIdentity: any;
	saveNewIdentity: any;
	selectIdentity: any;
	updateNewIdentity: any;
	updateIdentityName: any;
	updatePathName: any;
	deriveNewPath: any;
	deletePath: any;
	deleteCurrentIdentity: any;
};

const defaultAccountState = {
	accounts: new Map(),
	currentIdentity: null,
	identities: [],
	loaded: false,
	newAccount: emptyAccount(),
	newIdentity: emptyIdentity(),
	selectedKey: ''
};

export function useAccountContext(): AccountsContextState {
	const initialState: AccountsStoreState = defaultAccountState;

	const reducer = (
		state: AccountsStoreState,
		delta: Partial<AccountsStoreState>
	): AccountsStoreState => ({
		...state,
		...delta
	});
	const [state, setState] = useReducer(reducer, initialState);
	useEffect(() => {
		const refreshList = async (): Promise<void> => {
			const accounts = await loadAccounts();
			const identities = await loadIdentities();
			let { currentIdentity } = state;
			if (identities.length > 0) currentIdentity = identities[0];
			setState({
				accounts,
				currentIdentity,
				identities,
				loaded: true
			});
		};
		refreshList();
	}, []);

	function select(accountKey: string): void {
		setState({ selectedKey: accountKey });
	}

	function updateNew(accountUpdate: Partial<UnlockedAccount>): void {
		setState({
			newAccount: { ...state.newAccount, ...accountUpdate }
		});
	}

	function getNew(): UnlockedAccount {
		return state.newAccount;
	}

	function deleteSensitiveData(account: UnlockedAccount): LockedAccount {
		delete account.seed;
		delete account.seedPhrase;
		delete account.derivationPassword;
		delete account.derivationPath;

		return account;
	}

	async function save(
		accountKey: string,
		account: Account,
		pin: string | null = null
	): Promise<void> {
		try {
			// for account creation
			let accountToSave = account;
			if (pin && isUnlockedAccount(account)) {
				account.encryptedSeed = await encryptData(account.seed, pin);
				accountToSave = deleteSensitiveData(account);
			}

			await saveAccount(accountKey, accountToSave);
		} catch (e) {
			console.error(e);
		}
	}

	async function submitNew(pin: string): Promise<void> {
		const account = state.newAccount;
		if (!account.seed) return;

		const accountKey = generateAccountId(account);
		await save(accountKey, account, pin);

		setState({
			accounts: state.accounts.set(accountKey, account),
			newAccount: emptyAccount()
		});
	}

	function _updateIdentitiesWithCurrentIdentity(
		updatedCurrentIdentity: Identity
	): void {
		setState({
			currentIdentity: updatedCurrentIdentity
		});
		const newIdentities = deepCopyIdentities(state.identities);
		if (state.currentIdentity === null) return;
		const identityIndex = newIdentities.findIndex(
			(identity: Identity) =>
				identity.encryptedSeed === state.currentIdentity!.encryptedSeed
		);
		newIdentities.splice(identityIndex, 1, updatedCurrentIdentity);
		setState({ identities: newIdentities });
		saveIdentities(newIdentities);
	}

	function _updateCurrentIdentity(updatedIdentity: Identity): void {
		try {
			_updateIdentitiesWithCurrentIdentity(updatedIdentity);
		} catch (error) {
			throw new Error(identityUpdateError);
		}
	}

	function updateIdentityName(name: string): void {
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity!);
		updatedCurrentIdentity.name = name;
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	async function deriveEthereumAccount(
		createBrainWalletAddress: TryBrainWalletAddress,
		networkKey: string
	): Promise<void> {
		const networkParams = ETHEREUM_NETWORK_LIST[networkKey];
		const ethereumAddress = await brainWalletAddressWithRef(
			createBrainWalletAddress
		);
		if (ethereumAddress.address === '') throw new Error(addressGenerateError);
		const { ethereumChainId } = networkParams;
		const accountId = generateAccountId({
			address: ethereumAddress.address,
			networkKey
		});
		if (state.currentIdentity === null) throw new Error(emptyIdentityError);
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity);
		if (updatedCurrentIdentity.meta.has(ethereumChainId))
			throw new Error(accountExistedError);
		updatedCurrentIdentity.meta.set(ethereumChainId, {
			address: ethereumAddress.address,
			createdAt: new Date().getTime(),
			name: '',
			updatedAt: new Date().getTime()
		});
		updatedCurrentIdentity.addresses.set(accountId, ethereumChainId);
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	function _updateAccount(
		accountKey: string,
		updatedAccount: Partial<LockedAccount>
	): void {
		const accounts = state.accounts;
		const account = accounts.get(accountKey);

		if (account && updatedAccount) {
			setState({
				accounts: accounts.set(accountKey, { ...account, ...updatedAccount })
			});
		}
	}

	function updateSelectedAccount(updatedAccount: Partial<LockedAccount>): void {
		_updateAccount(state.selectedKey, updatedAccount);
	}

	async function refreshList(): Promise<void> {
		const accounts = await loadAccounts();
		const identities = await loadIdentities();
		let { currentIdentity } = state;
		if (identities.length > 0) currentIdentity = identities[0];
		setState({
			accounts,
			currentIdentity,
			identities,
			loaded: true
		});
	}

	async function deleteAccount(accountKey: string): Promise<void> {
		const { accounts } = state;

		accounts.delete(accountKey);
		setState({ accounts, selectedKey: '' });
		await deleteDbAccount(accountKey);
	}

	async function unlockAccount(
		accountKey: string,
		pin: string
	): Promise<boolean> {
		const { accounts } = state;
		const account = accounts.get(accountKey);

		if (!accountKey || !account || !account.encryptedSeed) {
			return false;
		}

		try {
			const decryptedSeed = await decryptData(account.encryptedSeed, pin);
			const { phrase, derivePath, password } = parseSURI(decryptedSeed);
			setState({
				accounts: state.accounts.set(accountKey, {
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

	function lockAccount(accountKey: string): void {
		const { accounts } = state;
		const account = accounts.get(accountKey);

		if (account && isUnlockedAccount(account)) {
			const lockedAccount = deleteSensitiveData(account);
			setState({
				accounts: state.accounts.set(accountKey, lockedAccount)
			});
		}
	}

	function _getAccountWithoutCaseSensitive(accountId: string): Account | null {
		let findLegacyAccount = null;
		for (const [key, value] of state.accounts) {
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

	function _getAccountFromIdentity(
		accountIdOrAddress: string
	): false | FoundIdentityAccount {
		const isAccountId = accountIdOrAddress.split(':').length > 1;
		let targetAccountId = null;
		let targetIdentity = null;
		let targetNetworkKey = null;
		let targetPath = null;
		for (const identity of state.identities) {
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
		setState({ currentIdentity: targetIdentity });

		const metaData = targetIdentity.meta.get(targetPath);
		if (metaData === undefined) return false;
		return {
			accountId: targetAccountId,
			encryptedSeed: targetIdentity.encryptedSeed,
			hasPassword: !!metaData.hasPassword,
			isLegacy: false,
			networkKey: targetNetworkKey!,
			path: targetPath,
			validBip39Seed: true,
			...metaData
		};
	}

	function getById({
		address,
		networkKey
	}: {
		address: string;
		networkKey: string;
	}): null | FoundAccount {
		const accountId = generateAccountId({ address, networkKey });
		const legacyAccount = _getAccountWithoutCaseSensitive(accountId);
		if (legacyAccount) return parseFoundLegacyAccount(legacyAccount, accountId);
		let derivedAccount;
		//assume it is an accountId
		if (networkKey !== UnknownNetworkKeys.UNKNOWN) {
			derivedAccount = _getAccountFromIdentity(accountId);
		}
		//TODO backward support for user who has create account in known network for an unknown network. removed after offline network update
		derivedAccount = derivedAccount || _getAccountFromIdentity(address);

		if (derivedAccount instanceof Object)
			return { ...derivedAccount, isLegacy: false };
		return null;
	}

	function getAccountByAddress(address: string): false | FoundAccount {
		if (!address) {
			return false;
		}

		for (const [k, v] of state.accounts.entries()) {
			if (v.address.toLowerCase() === address.toLowerCase()) {
				return { ...v, accountId: k, isLegacy: true };
			}
		}
		return _getAccountFromIdentity(address);
	}

	function getSelected(): Account | undefined {
		return state.accounts.get(state.selectedKey);
	}

	function getSelectedKey(): string {
		return state.selectedKey;
	}

	function getAccounts(): Map<string, Account> {
		return state.accounts;
	}

	function getIdentityByAccountId(accountId: string): Identity | undefined {
		const networkProtocol = accountId.split(':')[0];
		const searchAddress =
			networkProtocol === NetworkProtocols.SUBSTRATE
				? extractAddressFromAccountId(accountId)
				: accountId;
		return state.identities.find(identity =>
			identity.addresses.has(searchAddress)
		);
	}

	function resetCurrentIdentity(): void {
		setState({ currentIdentity: null });
	}

	async function _addPathToIdentity(
		newPath: string,
		createSubstrateAddress: TrySubstrateAddress,
		updatedIdentity: Identity,
		name: string,
		networkKey: string,
		password: string
	): Promise<Identity> {
		const { prefix, pathId } = SUBSTRATE_NETWORK_LIST[networkKey];
		const suriSuffix = constructSuriSuffix({
			derivePath: newPath,
			password
		});
		if (updatedIdentity.meta.has(newPath)) throw new Error(accountExistedError);
		let address = '';
		try {
			address = await createSubstrateAddress(suriSuffix, prefix);
		} catch (e) {
			throw new Error(addressGenerateError);
		}
		if (address === '') throw new Error(addressGenerateError);
		const pathMeta = {
			address,
			createdAt: new Date().getTime(),
			hasPassword: password !== '',
			name,
			networkPathId: pathId,
			updatedAt: new Date().getTime()
		};
		updatedIdentity.meta.set(newPath, pathMeta);
		updatedIdentity.addresses.set(address, newPath);
		return updatedIdentity;
	}

	async function saveNewIdentity(
		seedPhrase: string,
		pin: string,
		generateSeedRef: CreateSeedRefWithNewSeed
	): Promise<void> {
		const updatedIdentity = deepCopyIdentity(state.newIdentity);
		const suri = seedPhrase;

		updatedIdentity.encryptedSeed = await encryptData(suri, pin);
		//prevent duplication
		if (
			state.identities.find(
				i => i.encryptedSeed === updatedIdentity.encryptedSeed
			)
		)
			throw new Error(duplicatedIdentityError);
		await generateSeedRef(updatedIdentity.encryptedSeed, pin);
		const newIdentities = state.identities.concat(updatedIdentity);
		setState({
			currentIdentity: updatedIdentity,
			identities: newIdentities,
			newIdentity: emptyIdentity()
		});
		await saveIdentities(newIdentities);
	}

	function selectIdentity(identity: Identity): void {
		setState({ currentIdentity: identity });
	}

	function clearIdentity() {
		setState({ newIdentity: emptyIdentity() });
	}

	function updateNewIdentity(identityUpdate: Partial<Identity>): void {
		setState({
			newIdentity: { ...state.newIdentity, ...identityUpdate }
		});
	}

	function updatePathName(path: string, name: string): void {
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity!);
		const updatedPathMeta = Object.assign(
			{},
			updatedCurrentIdentity.meta.get(path),
			{ name }
		);
		updatedCurrentIdentity.meta.set(path, updatedPathMeta);
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	async function deriveNewPath(
		newPath: string,
		createSubstrateAddress: TrySubstrateAddress,
		networkKey: string,
		name: string,
		password: string
	): Promise<void> {
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity!);
		await _addPathToIdentity(
			newPath,
			createSubstrateAddress,
			updatedCurrentIdentity,
			name,
			networkKey,
			password
		);
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	function deletePath(path: string): void {
		if (state.currentIdentity === null) throw new Error(emptyIdentityError);
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity);
		const pathMeta = updatedCurrentIdentity.meta.get(path)!;
		updatedCurrentIdentity.meta.delete(path);
		updatedCurrentIdentity.addresses.delete(
			getAddressKeyByPath(path, pathMeta)
		);
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	async function deleteCurrentIdentity(): Promise<void> {
		const newIdentities = deepCopyIdentities(state.identities);
		const identityIndex = newIdentities.findIndex(
			(identity: Identity) =>
				identity.encryptedSeed === state.currentIdentity!.encryptedSeed
		);
		newIdentities.splice(identityIndex, 1);
		setState({
			currentIdentity: newIdentities.length >= 1 ? newIdentities[0] : null,
			identities: newIdentities
		});
		await saveIdentities(newIdentities);
	}

	return {
		clearIdentity,
		deleteAccount,
		deleteCurrentIdentity,
		deletePath,
		deleteSensitiveData,
		deriveEthereumAccount,
		deriveNewPath,
		getAccountByAddress,
		getAccounts,
		getById,
		getIdentityByAccountId,
		getNew,
		getSelected,
		getSelectedKey,
		lockAccount,
		refreshList,
		resetCurrentIdentity,
		save,
		saveNewIdentity,
		select,
		selectIdentity,
		state,
		submitNew,
		unlockAccount,
		updateIdentityName,
		updateNew,
		updateNewIdentity,
		updatePathName,
		updateSelectedAccount
	};
}

export const AccountsContext = React.createContext({} as AccountsContextState);
