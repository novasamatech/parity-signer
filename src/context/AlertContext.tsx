import { createContext, default as React, useCallback, useState } from 'react';

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

export interface AlertContextType extends AlertBase {
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

export const defaultAlertContext: AlertContextType = {
	...defaultAlertBase,
	setAlert: (): any => 0
};

interface AlertContextProviderProps {
	children?: React.ReactElement;
}

export const AlertContext = createContext(defaultAlertContext);

export function AlertContextProvider({ children }: AlertContextProviderProps): React.ReactElement  {
	const [alertState, setAlertState] = useState<AlertBase>(defaultAlertBase);

	const setAlert = useCallback((title, message, actions = []): void => {
		setAlertState({
			actions,
			alertIndex: alertState.alertIndex + 1,
			message,
			title
		})
	}, [alertState.alertIndex])

	return (
		<AlertContext.Provider value={{ ...alertState, setAlert }}>
			{children}
		</AlertContext.Provider>
	);

}
