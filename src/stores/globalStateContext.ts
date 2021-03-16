import { useEffect, useState } from 'react';
import * as React from 'react';

import { migrateAccounts, migrateIdentity } from 'utils/migrationUtils';

/* eslint-disable @typescript-eslint/no-empty-function */
export type GlobalState = {
	dataLoaded: boolean;
	setDataLoaded: (setValue: boolean) => void;
};

export const defaultGlobalState: GlobalState = {
	dataLoaded: false,
	setDataLoaded: (): void => {},
};

export function useGlobalStateContext(): GlobalState {
	const [dataLoaded, setDataLoaded] = useState<boolean>(false);

	useEffect(() => {
		const loadPolicyConfirmationAndMigrateData = async (): Promise<void> => {
			await migrateAccounts();
			await migrateIdentity();
		};
		setDataLoaded(true);
		loadPolicyConfirmationAndMigrateData();
	}, []);

	return {
		dataLoaded,
		setDataLoaded,
	};
}

export const GlobalStateContext = React.createContext(defaultGlobalState);
