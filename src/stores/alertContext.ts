import * as React from 'react';
import { useState } from 'react';

export type AlertState = {
	index: number;
	title: string;
	message: string;
	setAlert: (title: string, message: string) => void;
};

export const defaultAlertState: AlertState = {
	index: 0,
	message: '',
	setAlert: (): any => 0,
	title: ''
};

export function useAlertContext(): AlertState {
	const [alertState, setAlertState] = useState<{
		index: number;
		title: string;
		message: string;
	}>({
		index: 0,
		message: '',
		title: ''
	});

	return {
		...alertState,
		setAlert: (title, message): void =>
			setAlertState({
				index: alertState.index + 1,
				message,
				title
			})
	};
}

export const AlertStateContext = React.createContext(defaultAlertState);
