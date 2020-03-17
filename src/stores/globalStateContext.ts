import * as React from 'react';

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

export const GlobalStateContext = React.createContext(defaultGlobalState);
