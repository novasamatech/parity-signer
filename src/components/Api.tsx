import React, { useContext, useEffect, useMemo, useState } from 'react';
import { ApiPromise } from '@polkadot/api/promise';
import { WsProvider } from '@polkadot/rpc-provider';

import ApiContext, { ApiProps } from 'stores/ApiContext';
import { isSubstrateNetworkParams, NetworkParams } from 'types/networkTypes';
import { NetworksContext } from 'stores/NetworkContext';
import { RegistriesContext } from 'stores/RegistriesContext';

interface Props {
	children: React.ReactNode;
	networkKey: string;
}

let api: ApiPromise;

export { api };

function Api({
	children,
	networkKey
}: Props): React.ReactElement<Props> | null {
	const networkContextState = useContext(NetworksContext);
	const networkParams = networkContextState.getSubstrateNetwork(networkKey);
	const { getTypeRegistry } = useContext(RegistriesContext);
	const [isApiConnected, setIsApiConnected] = useState(false);
	const [isApiInitialized, setIsApiInitialized] = useState(false);
	const [isApiReady, setIsApiReady] = useState(false);
	const [apiError, setApiError] = useState<null | string>(null);

	const value = useMemo<ApiProps>(
		() => ({
			api,
			apiError,
			isApiConnected,
			isApiInitialized,
			isApiReady
		}),
		[apiError, isApiConnected, isApiInitialized, isApiReady]
	);

	// initial initialization when given a network key
	useEffect((): void => {
		if (!isSubstrateNetworkParams(networkParams)) return;
		const [registry, metadata] = getTypeRegistry(
			networkContextState.networks,
			networkKey
		)!;
		// TODO: load metadata at startup
		// TODO: handle errors
		// TODO: make this stateful so we don't have to reload every time we come here
		console.log(`CREATING API: ${networkParams.url}`);
		const provider = new WsProvider(networkParams.url);
		api = new ApiPromise({
			metadata,
			provider,
			registry
		});

		api.on('connected', () => setIsApiConnected(true));
		api.on('disconnected', () => setIsApiConnected(false));
		api.on('error', (error: Error) => setApiError(error.message));
		api.on('ready', (): void => {
			setIsApiReady(true);
		});

		setIsApiInitialized(true);
	}, [networkKey]);

	return <ApiContext.Provider value={value}>{children}</ApiContext.Provider>;
}

export default React.memo(Api);
