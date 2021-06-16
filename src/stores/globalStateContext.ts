import { useEffect, useState } from 'react';
import * as React from 'react';

import { checkUserAgreement, dbInit } from 'utils/native';

/* eslint-disable @typescript-eslint/no-empty-function */
export type GlobalState = {
	dataLoaded: boolean;
	policyConfirmed: boolean;
	setDataLoaded: (setValue: boolean) => void;
	setPolicyConfirmed: (setValue: boolean) => void;
};

export const defaultGlobalState: GlobalState = {
	dataLoaded: false,
	policyConfirmed: false,
	setDataLoaded: (): void => {},
	setPolicyConfirmed: (): void => {}
};

export function useGlobalStateContext(): GlobalState {
	const [policyConfirmed, setPolicyConfirmed] = useState<boolean>(false);
	const [dataLoaded, setDataLoaded] = useState<boolean>(false);

	useEffect(() => {
		const loadPolicyConfirmation = async (): Promise<void> => {
			const tocPP = await checkUserAgreement();
			await dbInit();
			console.log("loadPolicyConfirmation");
			setPolicyConfirmed(tocPP);
			setDataLoaded(true);
		};
		loadPolicyConfirmation();
	}, []);

	return {
		dataLoaded,
		policyConfirmed,
		setDataLoaded,
		setPolicyConfirmed
	};
}

export const GlobalStateContext = React.createContext(defaultGlobalState);
