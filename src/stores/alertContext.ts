import { createContext, useCallback, useState } from 'react';

export type SetAlert = (
	title: string,
	message: string,
	actions?: Action[]
) => void;

export interface Action {
	text: string;
	testID?: string;
	onPress?: () => any;
};
export interface AlertState extends AlertBase {
	setAlert: SetAlert;
};

interface AlertBase {
	actions: Action[];
	alertIndex: number;
	title: string;
	message: string;
}

const defaultAlertBase: AlertBase = {
	actions: [],
	alertIndex: 0,
	message: '',
	title: ''
}

export const defaultAlertState: AlertState = {
	...defaultAlertBase,
	setAlert: (): any => 0
};

export const AlertStateContext = createContext(defaultAlertState);

export function useAlertContext(): AlertState {
	const [alertState, setAlertState] = useState<AlertBase>(defaultAlertBase);

	const setAlert = useCallback((title, message, actions = []): void => {
		setAlertState({
			actions,
			alertIndex: alertState.alertIndex + 1,
			message,
			title
		})
	}, [alertState.alertIndex])

	return {
		...alertState,
		setAlert
	}
}
