import React, { useReducer } from 'react';
import { ApiPromise } from '@polkadot/api/promise';
import { WsProvider } from '@polkadot/rpc-provider';

import { NetworksContextState } from './NetworkContext';
import { RegistriesStoreState } from './RegistriesContext';

export type ApiStoreState = {
	api: ApiPromise | null;
	apiError: string | null;
	isApiConnected: boolean;
	isApiInitialized: boolean;
	isApiReady: boolean;
};

export type ApiContextState = {
	state: ApiStoreState;
	selectNetwork: (
		networkKey: string,
		networkContextState: NetworksContextState,
		registriesState: RegistriesStoreState
	) => Promise<void>;
	disconnect: () => Promise<void>;
};

const defaultApiState = {
	api: null,
	apiError: null,
	isApiConnected: false,
	isApiInitialized: false,
	isApiReady: false
};

export function useApiContext(): ApiContextState {
	const initialState: ApiStoreState = defaultApiState;
	const reducer = (
		state: ApiStoreState,
		delta: Partial<ApiStoreState>
	): ApiStoreState => ({
		...state,
		...delta
	});
	const [state, setState] = useReducer(reducer, initialState);

	// TODO: load an initial context

	async function selectNetwork(
		networkKey: string,
		networkContextState: NetworksContextState,
		registriesState: RegistriesStoreState
	): Promise<void> {
		const networkParams = networkContextState.getSubstrateNetwork(networkKey);
		if (!networkParams.url) return;

		const [registry, metadata] = registriesState.getTypeRegistry(
			networkContextState.networks,
			networkKey
		)!;
		// TODO: load metadata at startup
		// TODO: handle errors
		// TODO: make this stateful so we don't have to reload every time we come here
		console.log(`CREATING API: ${networkParams.url}`);
		const provider = new WsProvider(networkParams.url);
		const api = new ApiPromise({
			metadata,
			provider,
			registry
		});
		setState({ api });

		api.on('connected', () => setState({ isApiConnected: true }));
		api.on('disconnected', () => setState({ isApiConnected: false }));
		api.on('error', (error: Error) => setState({ apiError: error.message }));
		api.on('ready', (): void => setState({ isApiReady: true }));
		setState({ isApiInitialized: true });
	}

	// TODO: ensure this cleanup works as expected
	async function disconnect(): Promise<void> {
		if (state.api && state.api.isConnected) {
			console.log('DISCONNECTING API');
			state.api.disconnect();
		}
		setState({
			api: null,
			apiError: null,
			isApiConnected: false,
			isApiInitialized: false,
			isApiReady: false
		});
	}

	return {
		disconnect,
		selectNetwork,
		state
	};
}

export const ApiContext = React.createContext({} as ApiContextState);
