import * as React from 'react';

export type AlertState = {
	index: number;
	title: string;
	message: string;
	setAlert: (title: string, message: string) => void;
};

export const defaultAlertState: AlertState = {
	index: 0,
	title: '',
	message: '',
	setAlert: (): void => {}
};

export const AlertStateContext = React.createContext(defaultAlertState);
