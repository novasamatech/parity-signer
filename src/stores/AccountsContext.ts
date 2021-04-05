import { default as React, useEffect, useReducer } from 'react';

import {
	ETHEREUM_NETWORK_LIST,
	NetworkProtocols,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import { NetworksContextState } from 'stores/NetworkContext';
import {
	AccountsStoreState,
	FoundAccount,
	Identity
} from 'types/identityTypes';
import { NetworkParams, SubstrateNetworkParams } from 'types/networkTypes';
import { generateAccountId } from 'utils/account';
import { loadIdentities, saveIdentities } from 'utils/db';
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
	isEthereumAccountId
} from 'utils/identitiesUtils';
import { brainWalletAddressWithRef, encryptData } from 'utils/native';
import {
	CreateSeedRefWithNewSeed,
	TryBrainWalletAddress,
	TrySubstrateAddress
} from 'utils/seedRefHooks';
import { PIN } from 'constants/pin';

export type AccountsContextState = {
	clearIdentity: () => void;
	state: AccountsStoreState;
	deriveEthereumAccount: (
		createBrainWalletAddress: TryBrainWalletAddress,
		networkKey: string,
		allNetworks: Map<string, NetworkParams>
	) => Promise<void>;
	getById: (
		address: string,
		networkKey: string,
		networkContext: NetworksContextState
	) => null | FoundAccount;
	getAccountByAddress: (
		address: string,
		networkContext: NetworksContextState
	) => false | FoundAccount;
	getIdentityByAccountId: (accountId: string) => Identity | undefined;
	resetCurrentIdentity: () => void;
	saveNewIdentity: (
		seedPhrase: string,
		generateSeedRef: CreateSeedRefWithNewSeed
	) => Promise<void>;
	selectIdentity: (identity: Identity) => void;
	updateNewIdentity: (identityUpdate: Partial<Identity>) => void;
	updateIdentityName: (name: string) => void;
	updatePathName: (path: string, name: string) => void;
	deriveNewPath: (
		newPath: string,
		createSubstrateAddress: TrySubstrateAddress,
		networkParams: SubstrateNetworkParams,
		name: string
	) => Promise<void>;
	deleteEthereumAddress: (networkParams) => void;
	deleteSubstratePath: (
		path: string,
		networkContext: NetworksContextState
	) => void;
	deleteWallet: (identity: Identity) => Promise<void>;
};

const defaultAccountState = {
	currentIdentity: null,
	identities: [],
	loaded: false,
	newIdentity: emptyIdentity()
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
		const loadInitialContext = async (): Promise<void> => {
			const identities = await loadIdentities();
			const currentIdentity = identities.length > 0 ? identities[0] : null;
			setState({
				currentIdentity,
				identities,
				loaded: true
			});
		};
		loadInitialContext();
	}, []);

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
		networkKey: string,
		allNetworks: Map<string, NetworkParams>
	): Promise<void> {
		const networkParams = ETHEREUM_NETWORK_LIST[networkKey];
		const ethereumAddress = await brainWalletAddressWithRef(
			createBrainWalletAddress
		);
		if (ethereumAddress.address === '') throw new Error(addressGenerateError);
		const { ethereumChainId } = networkParams;
		const accountId = generateAccountId(
			ethereumAddress.address,
			networkKey,
			allNetworks
		);
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

	function _getAccountFromIdentity(
		accountIdOrAddress: string,
		networkContext: NetworksContextState
	): false | FoundAccount {
		const { allNetworks } = networkContext;
		const isAccountId = accountIdOrAddress.split(':').length > 1;
		let targetAccountId = null;
		let targetIdentity = null;
		let targetNetworkKey = null;
		let targetPath = null;
		for (const identity of state.identities) {
			const searchList = Array.from(identity.addresses.entries());
			for (const [addressKey, path] of searchList) {
				const networkKey = getNetworkKey(path, identity, networkContext);
				let accountId, address;
				if (isEthereumAccountId(addressKey)) {
					accountId = addressKey;
					address = extractAddressFromAccountId(addressKey);
				} else {
					accountId = generateAccountId(addressKey, networkKey, allNetworks);
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
			networkKey: targetNetworkKey!,
			path: targetPath,
			validBip39Seed: true,
			...metaData
		};
	}

	function getById(
		address: string,
		networkKey: string,
		networkContext: NetworksContextState
	): null | FoundAccount {
		const { allNetworks } = networkContext;
		const accountId = generateAccountId(address, networkKey, allNetworks);
		let derivedAccount;
		//assume it is an accountId
		if (networkKey !== UnknownNetworkKeys.UNKNOWN) {
			derivedAccount = _getAccountFromIdentity(accountId, networkContext);
		}
		//TODO backward support for user who has create account in known network for an unknown network. removed after offline network update
		derivedAccount =
			derivedAccount || _getAccountFromIdentity(address, networkContext);

		if (derivedAccount instanceof Object) return { ...derivedAccount };
		return null;
	}

	function getAccountByAddress(
		address: string,
		networkContext: NetworksContextState
	): false | FoundAccount {
		if (!address) {
			return false;
		}

		return _getAccountFromIdentity(address, networkContext);
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
		networkParams: SubstrateNetworkParams
	): Promise<Identity> {
		const { prefix, pathId } = networkParams;
		if (updatedIdentity.meta.has(newPath)) throw new Error(accountExistedError);
		let address = '';
		try {
			address = await createSubstrateAddress('', prefix);
		} catch (e) {
			throw new Error(addressGenerateError);
		}
		if (address === '') throw new Error(addressGenerateError);
		const pathMeta = {
			address,
			createdAt: new Date().getTime(),
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
		generateSeedRef: CreateSeedRefWithNewSeed
	): Promise<void> {
		const updatedIdentity = deepCopyIdentity(state.newIdentity);
		const suri = seedPhrase;

		updatedIdentity.encryptedSeed = await encryptData(suri, PIN);
		//prevent duplication
		if (
			state.identities.find(
				i => i.encryptedSeed === updatedIdentity.encryptedSeed
			)
		)
			throw new Error(duplicatedIdentityError);
		await generateSeedRef(updatedIdentity.encryptedSeed, PIN);
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
		networkParams: SubstrateNetworkParams,
		name: string
	): Promise<void> {
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity!);
		await _addPathToIdentity(
			newPath,
			createSubstrateAddress,
			updatedCurrentIdentity,
			name,
			networkParams
		);
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	function deleteEthereumAddress(networkKey): void {
		if (state.currentIdentity === null) throw new Error(emptyIdentityError);
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity);

		updatedCurrentIdentity.meta.delete(networkKey);

		let key;
		updatedCurrentIdentity.addresses.forEach((k, v) => {
			if (k === networkKey) key = v;
		});
		if (key) {
			updatedCurrentIdentity.addresses.delete(key);
		}

		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	function deleteSubstratePath(
		path: string,
		networkContext: NetworksContextState
	): void {
		if (state.currentIdentity === null) throw new Error(emptyIdentityError);
		const updatedCurrentIdentity = deepCopyIdentity(state.currentIdentity);
		const pathMeta = updatedCurrentIdentity.meta.get(path);
		if (pathMeta) {
			updatedCurrentIdentity.meta.delete(path);
			updatedCurrentIdentity.addresses.delete(
				getAddressKeyByPath(path, pathMeta, networkContext)
			);
		} else {
			updatedCurrentIdentity.addresses.delete(path);
		}
		_updateCurrentIdentity(updatedCurrentIdentity);
	}

	function deleteWallet(identity: Identity): Promise<void> {
		const newIdentities = deepCopyIdentities(state.identities);
		const identityIndex = newIdentities.findIndex(
			(i: Identity) => identity.encryptedSeed === i.encryptedSeed
		);
		newIdentities.splice(identityIndex, 1);
		setState({
			currentIdentity: newIdentities.length >= 1 ? newIdentities[0] : null,
			identities: newIdentities
		});
		saveIdentities(newIdentities);
	}

	return {
		clearIdentity,
		deleteWallet,
		deleteSubstratePath,
		deleteEthereumAddress,
		deriveEthereumAccount,
		deriveNewPath,
		getAccountByAddress,
		getById,
		getIdentityByAccountId,
		resetCurrentIdentity,
		saveNewIdentity,
		selectIdentity,
		state,
		updateIdentityName,
		updateNewIdentity,
		updatePathName
	};
}

export const AccountsContext = React.createContext({} as AccountsContextState);
