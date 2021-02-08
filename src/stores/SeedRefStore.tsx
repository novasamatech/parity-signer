import React, { Dispatch, SetStateAction, useState } from 'react';
import { AppState, AppStateStatus } from 'react-native';
import { SeedRefClass } from 'utils/native';

export type SeedRefsState = [
	Map<string, SeedRefClass>,
	Dispatch<SetStateAction<Map<string, SeedRefClass>>>
];

export const SeedRefsContext = React.createContext(([] as unknown) as SeedRefsState);

export function useSeedRefStore(): SeedRefsState {
	const [seedRefs, setSeedRefs] = useState(new Map());

	const [appState, setAppState] = React.useState<AppStateStatus>(AppState.currentState);

	React.useEffect(() => {
		const _handleAppStateChange = async (nextAppState: AppStateStatus): Promise<void> => {
			if (nextAppState.match(/inactive|background/) && appState === 'active') {
				const promises: Promise<SeedRefClass>[] = Array.from(seedRefs.entries()).map(([, seedRef]) => {
					if (seedRef.isValid()) {
						return seedRef.tryDestroy();
					}

					return Promise.resolve();
				});

				await Promise.all(promises);
				setSeedRefs(new Map());
			}

			setAppState(nextAppState);
		};

		AppState.addEventListener('change', _handleAppStateChange);

		return (): void => {
			AppState.removeEventListener('change', _handleAppStateChange);
		};
	}, [appState, seedRefs]);

	return [seedRefs, setSeedRefs];
}
