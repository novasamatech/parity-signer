import { useEffect, useState } from 'react';
import * as React from 'react';
import { loadTaCAndPPConfirmation } from 'utils/db';

export interface TacHookType {
	dataLoaded: boolean;
	ppAndTaCAccepted: boolean;
	setPpAndTaCAccepted: (setValue: boolean) => void;
};

export const defaultGlobalState: TacHookType = {
	dataLoaded: false,
	ppAndTaCAccepted: false,
	setPpAndTaCAccepted: (): void => {}
};

export const TacContext = React.createContext(defaultGlobalState);

export function useTac(): TacHookType {
	const [ppAndTaCAccepted, setPpAndTaCAccepted] = useState(false);
	const [dataLoaded, setDataLoaded] = useState(false);

	useEffect(() => {
		loadTaCAndPPConfirmation()
			.then((tacPP) => {
				setPpAndTaCAccepted(tacPP);
				setDataLoaded(true);
			}).catch((e) => {
				console.error('Error 1 in useTac', e)
			});
	}, []);

	return { dataLoaded, ppAndTaCAccepted, setPpAndTaCAccepted }
}
