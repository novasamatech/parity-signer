import { useEffect, useState } from 'react';
import * as React from 'react';
import { loadToCAndPPConfirmation } from 'utils/db';
import { migrateAccounts, migrateIdentity } from 'utils/migrationUtils';

export interface TacHookType {
	dataLoaded: boolean;
	policyConfirmed: boolean;
	setDataLoaded: (setValue: boolean) => void;
	setPolicyConfirmed: (setValue: boolean) => void;
};

export const defaultGlobalState: TacHookType = {
	dataLoaded: false,
	policyConfirmed: false,
	setDataLoaded: (): void => {},
	setPolicyConfirmed: (): void => {}
};

export const TacContext = React.createContext(defaultGlobalState);

export function useTac(): TacHookType {
	const [policyConfirmed, setPolicyConfirmed] = useState<boolean>(false);
	const [dataLoaded, setDataLoaded] = useState<boolean>(false);

	useEffect(() => {
		const loadPolicyConfirmationAndMigrateData = async (): Promise<void> => {
			const tocPP = await loadToCAndPPConfirmation();

			setPolicyConfirmed(tocPP);

			if (!tocPP) {
				await migrateAccounts();
				await migrateIdentity();
			}
		};

		setDataLoaded(true);
		loadPolicyConfirmationAndMigrateData();
	}, []);

	return { dataLoaded, policyConfirmed, setDataLoaded, setPolicyConfirmed }
}
