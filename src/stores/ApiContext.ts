import React, { useReducer } from 'react';
import { AppState, AppStateStatus } from 'react-native';
import { ApiPromise } from '@polkadot/api/promise';
import { WsProvider } from '@polkadot/rpc-provider';

import { NetworksContextState } from './NetworkContext';
import { RegistriesStoreState } from './RegistriesContext';

export type ApiStoreState = {
	api: ApiPromise | null;
	apiError: string | null;
	apiNetworkKey: string;
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
	disconnect: (api: ApiPromise | null) => void;
};

const defaultApiState = {
	api: null,
	apiError: null,
	apiNetworkKey: '',
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
	const onConnected = (): void => setState({ isApiConnected: true });
	const onDisconnected = (): void => setState({ isApiConnected: false });
	const onError = (error: Error): void => setState({ apiError: error.message });
	const onReady = (): void => {
		setState({ isApiReady: true });
		console.log('API READY');
	};

	async function selectNetwork(
		networkKey: string,
		networkContextState: NetworksContextState,
		registriesState: RegistriesStoreState
	): Promise<void> {
		setState({ apiNetworkKey: networkKey });
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

		api.on('connected', onConnected);
		api.on('disconnected', onDisconnected);
		api.on('error', onError);
		api.on('ready', onReady);

		setState({ isApiInitialized: true });
	}

	// TODO: ensure this cleanup works as expected
	async function disconnectAsync(api: ApiPromise | null): Promise<void> {
		if (api && api.isConnected) {
			console.log('DISCONNECTING API');
			setState({
				api: null,
				apiError: null,
				isApiConnected: false,
				isApiInitialized: false,
				isApiReady: false
			});
			api.off('connected', onConnected);
			api.off('disconnected', onDisconnected);
			api.off('error', onError);
			api.off('ready', onReady);
			return api.disconnect();
		}
	}

	function disconnect(api: ApiPromise | null): void {
		disconnectAsync(api);
	}

	const [appState, setAppState] = React.useState<AppStateStatus>(
		AppState.currentState
	);

	// manage entering/leaving the app
	React.useEffect(() => {
		const _handleAppStateChange = async (
			nextAppState: AppStateStatus
		): Promise<void> => {
			console.log(`state change triggered: ${appState} -> ${nextAppState}`);
			if (nextAppState.match(/inactive|background/) && appState === 'active') {
				// disconnect on inactive
				// TODO: save state if needed
				await disconnectAsync(state.api);
			} else if (
				nextAppState === 'active' &&
				(appState === 'inactive' || appState === 'background')
			) {
				// TODO: reconnect on active if not connected
			}
			setAppState(nextAppState);
		};
		AppState.addEventListener('change', _handleAppStateChange);

		return (): void => {
			AppState.removeEventListener('change', _handleAppStateChange);
		};
	}, [appState]);

	return {
		disconnect,
		selectNetwork,
		state
	};
}

export const ApiContext = React.createContext({} as ApiContextState);
