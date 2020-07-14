import {useState} from 'react';
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

export function useAlertContext (){
	const [alertState, setAlertState] = useState<{
		index: number;
		title: string;
		message: string;
	}>({
		index: 0,
		title: '',
		message: ''
	});

	const alertContext: AlertState = {
		...alertState,
		setAlert: (title, message) =>
			setAlertState({
				index: alertState.index + 1,
				title,
				message
			})
	};

	return alertContext;
}

export const AlertStateContext = React.createContext(defaultAlertState);
