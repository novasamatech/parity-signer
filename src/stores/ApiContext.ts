import React from 'react';
import { ApiPromise } from '@polkadot/api/promise';

export interface ApiProps {
	api: ApiPromise;
	apiError: string | null;
	isApiConnected: boolean;
	isApiInitialized: boolean;
	isApiReady: boolean;
}

const ApiContext: React.Context<ApiProps> = React.createContext(
	({} as unknown) as ApiProps
);
const ApiConsumer: React.Consumer<ApiProps> = ApiContext.Consumer;
const ApiProvider: React.Provider<ApiProps> = ApiContext.Provider;

export default ApiContext;

export { ApiConsumer, ApiProvider };
