import * as React from 'react';
import { useState } from 'react';

export type SetAlert = (
	title: string,
	message: string,
	actions?: Action[]
) => void;
export type Action = {
	text: string;
	onPress?: () => any;
};
export type AlertState = {
	actions: Action[];
	index: number;
	title: string;
	message: string;
	setAlert: SetAlert;
};

export const defaultAlertState: AlertState = {
	actions: [],
	index: 0,
	message: '',
	setAlert: (): any => 0,
	title: ''
};

export function useAlertContext(): AlertState {
	const [alertState, setAlertState] = useState<{
		actions: Action[];
		index: number;
		title: string;
		message: string;
	}>({
		actions: [],
		index: 0,
		message: '',
		title: ''
	});

	return {
		...alertState,
		setAlert: (title, message, actions = []): void =>
			setAlertState({
				actions,
				index: alertState.index + 1,
				message,
				title
			})
	};
}

export const AlertStateContext = React.createContext(defaultAlertState);
