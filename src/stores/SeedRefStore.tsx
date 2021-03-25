import React, { useState } from 'react';
import { AppState, AppStateStatus } from 'react-native';

import { SeedRefClass } from 'utils/native';

export type SeedRefsState = {
	seedRefs: Map<string, SeedRefClass>;
	setSeedRefs: (newSeedRefs: Map<string, SeedRefClass>) => void;
};

export const SeedRefsContext = React.createContext({} as SeedRefsState);

export function useSeedRefStore(): SeedRefsState {
	const [seedRefs, setSeedRefsInternal] = useState(new Map());

	const [appState, setAppState] = React.useState<AppStateStatus>(
		AppState.currentState
	);

	function setSeedRefs(newSeedRefs: Map<string, SeedRefClass>): void {
		setSeedRefsInternal(newSeedRefs);
	}

	React.useEffect(() => {
		const _handleAppStateChange = async (
			nextAppState: AppStateStatus
		): Promise<void> => {
			if (nextAppState.match(/inactive|background/) && appState === 'active') {
				const promises: Promise<SeedRefClass>[] = Array.from(
					seedRefs.entries()
				).map(([, seedRef]) => {
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

	return { seedRefs, setSeedRefs };
}
