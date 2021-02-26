import { ETHEREUM_NETWORK_LIST, NetworkProtocols } from 'constants/networkSpecs';
import { createContext, default as React, useCallback, useContext, useEffect, useReducer, useState } from 'react';
import { Account, AccountsStoreState, Identity, isUnlockedAccount, LegacyAccount, LockedAccount, UnlockedAccount } from 'types/identityTypes';
import { SubstrateNetworkParams } from 'types/networkTypes';
import { emptyAccount, generateAccountId } from 'utils/account';
import { deleteAccount as deleteDbAccount, loadAccounts, saveAccount as saveDbAccount, saveIdentities } from 'utils/db';
import { accountExistedError, addressGenerateError, duplicatedIdentityError, emptyIdentityError, identityUpdateError } from 'utils/errors';
import { deepCopyIdentities, deepCopyIdentity, emptyIdentity, extractAddressFromAccountId, getAddressKeyByPath } from 'utils/identitiesUtils';
import { brainWalletAddressWithRef, decryptData, encryptData } from 'utils/native';
import { CreateSeedRefWithNewSeed, TryBrainWalletAddress, TrySubstrateAddress } from 'utils/seedRefHooks';
import { constructSuriSuffix, parseSURI } from 'utils/suri';

import { NetworksContext, NetworksContextType } from './NetworksContext';

export interface AccountsContextType {
	accounts: LegacyAccount[];
	accountsLoaded: boolean;
	newAccount: LegacyAccount;
	clearIdentity: () => void;
	deleteAccount: (accountAddress: string) => Promise<void>;
	deleteCurrentIdentity: () => Promise<void>;
	deletePath: (path: string, networkContext: NetworksContextType) => void;
	deriveNewPath: (newPath: string, createSubstrateAddress: TrySubstrateAddress, networkParams: SubstrateNetworkParams, name: string, password: string) => Promise<void>;
	deriveEthereumAccount: (createBrainWalletAddress: TryBrainWalletAddress, networkKey: string) => Promise<void>;
	getAccountByAddress: (address: string) => LegacyAccount | undefined;
	getIdentityByAccountId: (accountId: string) => Identity | undefined;
	getSelectedAccount: () => LegacyAccount | undefined;
	lockAccount: (accountKey: string) => void;
	resetCurrentIdentity: () => void;
	saveAccount: (account: Account, pin?: string) => Promise<void>;
	saveNewIdentity: (seedPhrase: string, pin: string, generateSeedRef: CreateSeedRefWithNewSeed) => Promise<void>;
	selectAccount: (accountAddress: string) => void;
	state: AccountsStoreState;
	submitNew: (pin: string) => Promise<void>;
	selectIdentity: (identity: Identity) => void;
	unlockAccount: (accountKey: string, pin: string) => Promise<boolean>;
	updateIdentityName: (name: string) => void;
	updateNew: (accountUpdate: Partial<LegacyAccount>) => void;
	updateNewIdentity: (identityUpdate: Partial<Identity>) => void;
	updatePathName: (path: string, name: string) => void;
	updateAccountName: (updatedAccount: Partial<LockedAccount>) => void;
};

const defaultAccountState = {
	currentIdentity: null,
	identities: [],
	newIdentity: emptyIdentity()
};

interface AccountsContextProviderProps {
	children?: React.ReactElement;
}

export const AccountsContext = createContext({} as AccountsContextType);

export function AccountsContextProvider({ children }: AccountsContextProviderProps): React.ReactElement {
	const initialState: AccountsStoreState = defaultAccountState;
	const [selectedAccountAddress, setSelectedAccountAddress] = useState('');
	const [accounts, setAccounts] = useState<LegacyAccount[]>([]);
	const [newAccount, setNewAccount] = useState<LegacyAccount>(emptyAccount());
	const [accountsLoaded, setAccountLoaded] = useState(false);

	const reducer = (state: AccountsStoreState,
		delta: Partial<AccountsStoreState>): AccountsStoreState => ({
		...state,
		...delta
	});
	const [state, setState] = useReducer(reducer, initialState)
	const { allNetworks, getNetwork } = useContext(NetworksContext);

	// console.log('accounts', accounts)
	// console.log('account new', newAccount)

	const loadAccountsFromDb = async (): Promise<void> => {
		const loadedAccounts = await loadAccounts();
		// const identities = await loadIdentities();
		// const currentIdentity = identities.length > 0 ? identities[0] : null;

		setAccounts(loadedAccounts);
		setAccountLoaded(true);
	};

	useEffect(() => {
		loadAccountsFromDb();
	}, []);

	const updateNew = useCallback((accountUpdate: Partial<LegacyAccount>): void => {
		// const newConstrustedAccount = { ...newAccount, ...accountUpdate }

		setNewAccount((previous: LegacyAccount | undefined) => ({
			...previous,
			...accountUpdate
		} as LegacyAccount))
	}, [])

	function _deleteSensitiveData({ address, createdAt, encryptedSeed, isLegacy, name, networkKey, recovered, updatedAt, validBip39Seed }: UnlockedAccount): LockedAccount {
		return {
			address,
			createdAt,
			encryptedSeed,
			isLegacy,
			name,
			networkKey,
			recovered,
			updatedAt,
			validBip39Seed
		} as LockedAccount;
	}

	async function saveAccount(account: LegacyAccount, pin?: string): Promise<void> {
		try {
			// for account creation
			let accountToSave = account;

			if (pin && isUnlockedAccount(account)) {
				account.encryptedSeed = await encryptData(account.seed, pin);
				accountToSave = _deleteSensitiveData(account);
			}

			const isEthereum = getNetwork(account.networkKey)?.protocol === NetworkProtocols.ETHEREUM;

			await saveDbAccount(accountToSave, isEthereum);
			loadAccountsFromDb();
		} catch (e) {
			console.error(e);
		}
	}

	async function submitNew(pin: string): Promise<void> {
		try{
			if (!newAccount.seed) {
				console.error('Account seed is empty')

				return
			}

			await saveAccount(newAccount, pin);

			// setAccounts((prev) => [...prev, newAccount]);
			setNewAccount(emptyAccount());
			loadAccountsFromDb();
		} catch (e) {
			console.error('error when saving new account', e)
		}
	}

	function _updateIdentitiesWithCurrentIdentity(updatedCurrentIdentity: Identity): void {
		setState({ currentIdentity: updatedCurrentIdentity });
		const newIdentities = deepCopyIdentities(state.identities);

		if (state.currentIdentity === null) return;
		const identityIndex = newIdentities.findIndex((identity: Identity) =>
			identity.encryptedSeed === state.currentIdentity!.encryptedSeed);

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

	async function deriveEthereumAccount(createBrainWalletAddress: TryBrainWalletAddress, networkKey: string): Promise<void> {
		const networkParams = ETHEREUM_NETWORK_LIST[networkKey];
		const ethereumAddress = await brainWalletAddressWithRef(createBrainWalletAddress);

		if (ethereumAddress.address === '') throw new Error(addressGenerateError);
		const { ethereumChainId, pathId } = networkParams;
		const accountId = generateAccountId(ethereumAddress.address, networkKey, allNetworks);

		if (state.currentIdentity === null) {
			throw new Error(emptyIdentityError);
		}

		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity);

		if (updatedCurrentIdentity.meta.has(ethereumChainId)){
			throw new Error(accountExistedError);
		}

		updatedCurrentIdentity.meta.set(ethereumChainId, {
			address: ethereumAddress.address,
			createdAt: new Date().getTime(),
			name: '',
			networkPathId: pathId,
			updatedAt: new Date().getTime()
		});

		updatedCurrentIdentity.addresses.set(accountId, ethereumChainId);
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	function _updateAccount(updatedAccount: Partial<LockedAccount>): void {
		const account = accounts.find((stored) => stored.address === updatedAccount.address);

		if (!account){
			console.error('No account found to update', accounts, account)

			return;
		}

		setAccounts([...accounts, { ...account, ...updatedAccount }]);
	}

	function updateAccountName(updatedAccount: Partial<LockedAccount>): void {
		_updateAccount({ ... updatedAccount });
	}

	async function deleteAccount(address: string): Promise<void> {
		const newAccounts = accounts.filter((account) => {
			// TODO make sure this works with substrate addresses and that we store ss58 substrate encoding
			return account.address !== address
		})

		await deleteDbAccount(address);
		setAccounts(newAccounts)
		setSelectedAccountAddress('');

	}

	const accountStateWithoutAccount = (address: string) => {

		return accounts.filter((account) => account.address !== address)
	}

	const getAccountByAddress = (address: string): LegacyAccount | undefined => {

		return accounts.find((account) => account.address.toLowerCase() === address.toLowerCase())
	}

	async function unlockAccount(address: string, pin: string): Promise<boolean> {
		const account = getAccountByAddress(address) ;

		if (!account || !account.encryptedSeed) {
			console.error('No account found for the address', address)

			return false;
		}

		try {
			const decryptedSeed = await decryptData(account.encryptedSeed, pin);
			const { derivePath, password, phrase } = parseSURI(decryptedSeed);
			const unlockedAccount = {
				derivationPassword: password || '',
				derivationPath: derivePath || '',
				seed: decryptedSeed,
				seedPhrase: phrase || '',
				...account
			}

			setAccounts([
				...accountStateWithoutAccount(address),
				unlockedAccount
			]);
		} catch (e) {
			return false;
		}

		return true;
	}

	function lockAccount(address: string): void {
		const account = getAccountByAddress(address);

		if (account && isUnlockedAccount(account)) {
			const lockedAccount = _deleteSensitiveData(account);

			setAccounts([
				...accountStateWithoutAccount(address),
				lockedAccount
			]);
		}
	}

	// function _getAccountWithoutCaseSensitive(accountId: string): Account | null {
	// 	let findLegacyAccount = null;

	// 	for (const [key, value] of state.accounts) {
	// 		if (isEthereumAccountId(accountId)) {
	// 			if (key.toLowerCase() === accountId.toLowerCase()) {
	// 				findLegacyAccount = value;
	// 				break;
	// 			}
	// 		} else if (key === accountId) {
	// 			findLegacyAccount = value;
	// 			break;
	// 		} else if (
	// 			//backward compatible with hard spoon substrate key pairs
	// 			extractAddressFromAccountId(key) ===
	// 			extractAddressFromAccountId(accountId)
	// 		) {
	// 			findLegacyAccount = value;
	// 			break;
	// 		}
	// 	}

	// 	return findLegacyAccount;
	// }

	// function _getAccountFromIdentity(accountIdOrAddress: string, networkContext: NetworksContextState): false | FoundIdentityAccount {
	// 	const { allNetworks } = networkContext;
	// 	const isAccountId = accountIdOrAddress.split(':').length > 1;
	// 	let targetAccountId = null;
	// 	let targetIdentity = null;
	// 	let targetNetworkKey = null;
	// 	let targetPath = null;

	// 	for (const identity of state.identities) {
	// 		const searchList = Array.from(identity.addresses.entries());

	// 		for (const [addressKey, path] of searchList) {
	// 			const networkKey = getNetworkKey(path, identity, networkContext);
	// 			let accountId, address;

	// 			if (isEthereumAccountId(addressKey)) {
	// 				accountId = addressKey;
	// 				address = extractAddressFromAccountId(addressKey);
	// 			} else {
	// 				accountId = generateAccountId(addressKey, networkKey, allNetworks);
	// 				address = addressKey;
	// 			}

	// 			const searchAccountIdOrAddress = isAccountId ? accountId : address;
	// 			const found = isEthereumAccountId(accountId)
	// 				? searchAccountIdOrAddress.toLowerCase() ===
	// 				  accountIdOrAddress.toLowerCase()
	// 				: searchAccountIdOrAddress === accountIdOrAddress;

	// 			if (found) {
	// 				targetPath = path;
	// 				targetIdentity = identity;
	// 				targetAccountId = accountId;
	// 				targetNetworkKey = networkKey;
	// 				break;
	// 			}
	// 		}
	// 	}

	// 	if (
	// 		targetPath === null ||
	// 		targetIdentity === null ||
	// 		targetAccountId === null
	// 	)
	// 		return false;
	// 	setState({ currentIdentity: targetIdentity });

	// 	const metaData = targetIdentity.meta.get(targetPath);

	// 	if (metaData === undefined) return false;

	// 	return {
	// 		accountId: targetAccountId,
	// 		encryptedSeed: targetIdentity.encryptedSeed,
	// 		hasPassword: !!metaData.hasPassword,
	// 		isLegacy: false,
	// 		networkKey: targetNetworkKey!,
	// 		path: targetPath,
	// 		validBip39Seed: true,
	// 		...metaData
	// 	};
	// }

	// function getById(address: string, networkKey: string, networkContext: NetworksContextState): null | FoundAccount {
	// 	const { allNetworks } = networkContext;
	// 	const accountId = generateAccountId(address, networkKey, allNetworks);
	// 	const legacyAccount = _getAccountWithoutCaseSensitive(accountId);

	// 	if (legacyAccount) return parseFoundLegacyAccount(legacyAccount, accountId);
	// 	let derivedAccount;

	// 	//assume it is an accountId
	// 	if (networkKey !== UnknownNetworkKeys.UNKNOWN) {
	// 		derivedAccount = _getAccountFromIdentity(accountId, networkContext);
	// 	}

	// 	//TODO backward support for user who has create account in known network for an unknown network. removed after offline network update
	// 	derivedAccount =
	// 		derivedAccount || _getAccountFromIdentity(address, networkContext);

	// 	if (derivedAccount instanceof Object)
	// 		return { ...derivedAccount, isLegacy: false };

	// 	return null;
	// }

	// function getAccountByAddress(address: string, networkContext: NetworksContextState): false | FoundAccount {
	// 	if (!address) {
	// 		return false;
	// 	}

	// 	for (const [k, v] of state.accounts.entries()) {
	// 		if (v.address.toLowerCase() === address.toLowerCase()) {
	// 			return { ...v, accountId: k, isLegacy: true };
	// 		}
	// 	}

	// 	return _getAccountFromIdentity(address, networkContext);
	// }

	function selectAccount(address: string): void {

		setSelectedAccountAddress(address);
	}

	const getSelectedAccount = () => getAccountByAddress(selectedAccountAddress);

	function getIdentityByAccountId(accountId: string): Identity | undefined {
		const networkProtocol = accountId.split(':')[0];
		const searchAddress =
			networkProtocol === NetworkProtocols.SUBSTRATE
				? extractAddressFromAccountId(accountId)
				: accountId;

		return state.identities.find(identity =>
			identity.addresses.has(searchAddress));
	}

	function resetCurrentIdentity(): void {
		setState({ currentIdentity: null });
	}

	async function _addPathToIdentity(newPath: string,
		createSubstrateAddress: TrySubstrateAddress,
		updatedIdentity: Identity,
		name: string,
		networkParams: SubstrateNetworkParams,
		password: string): Promise<Identity> {
		const { pathId, prefix } = networkParams;
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

	async function saveNewIdentity(seedPhrase: string,
		pin: string,
		generateSeedRef: CreateSeedRefWithNewSeed): Promise<void> {
		const updatedIdentity = deepCopyIdentity(state.newIdentity);
		const suri = seedPhrase;

		updatedIdentity.encryptedSeed = await encryptData(suri, pin);
		//prevent duplication
		if (
			state.identities.find(i => i.encryptedSeed === updatedIdentity.encryptedSeed)
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

	function clearIdentity(): void {
		setState({ newIdentity: emptyIdentity() });
	}

	function updateNewIdentity(identityUpdate: Partial<Identity>): void {
		setState({ newIdentity: { ...state.newIdentity, ...identityUpdate } });
	}

	function updatePathName(path: string, name: string): void {
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity!);
		const updatedPathMeta = Object.assign({},
			updatedCurrentIdentity.meta.get(path),
			{ name });

		updatedCurrentIdentity.meta.set(path, updatedPathMeta);
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	async function deriveNewPath(newPath: string,
		createSubstrateAddress: TrySubstrateAddress,
		networkParams: SubstrateNetworkParams,
		name: string,
		password: string): Promise<void> {
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity!);

		await _addPathToIdentity(newPath,
			createSubstrateAddress,
			updatedCurrentIdentity,
			name,
			networkParams,
			password);
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	function deletePath(path: string, networkContext: NetworksContextType): void {
		if (state.currentIdentity === null) throw new Error(emptyIdentityError);
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity);
		const pathMeta = updatedCurrentIdentity.meta.get(path)!;

		updatedCurrentIdentity.meta.delete(path);
		updatedCurrentIdentity.addresses.delete(getAddressKeyByPath(path, pathMeta, networkContext));
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	async function deleteCurrentIdentity(): Promise<void> {
		const newIdentities = deepCopyIdentities(state.identities);
		const identityIndex = newIdentities.findIndex((identity: Identity) =>
			identity.encryptedSeed === state.currentIdentity!.encryptedSeed);

		newIdentities.splice(identityIndex, 1);
		setState({
			currentIdentity: newIdentities.length >= 1 ? newIdentities[0] : null,
			identities: newIdentities
		});
		await saveIdentities(newIdentities);
	}

	return (
		<AccountsContext.Provider value={{
			accounts,
			accountsLoaded,
			clearIdentity,
			deleteAccount,
			deleteCurrentIdentity,
			deletePath,
			deriveEthereumAccount,
			deriveNewPath,
			getAccountByAddress,
			getIdentityByAccountId,
			getSelectedAccount,
			lockAccount,
			newAccount,
			resetCurrentIdentity,
			saveAccount,
			saveNewIdentity,
			selectAccount,
			selectIdentity,
			state,
			submitNew,
			unlockAccount,
			updateAccountName,
			updateIdentityName,
			updateNew,
			updateNewIdentity,
			updatePathName
		}}>
			{children}
		</AccountsContext.Provider>
	);
}
