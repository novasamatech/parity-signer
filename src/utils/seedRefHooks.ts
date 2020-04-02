import { useState } from 'react';

import { decryptDataRef, destroyDataRef } from 'utils/native';

type TryCreateFunc = (encryptedSeed: string, password: string) => Promise<void>;
type TryDestroyFunc = () => Promise<void>;

type SeedRefHooks = [number, boolean, TryCreateFunc, TryDestroyFunc];

export function useSeedRef(): SeedRefHooks {
	const [dataRef, setDataRef] = useState<number>(0);
	const [valid, setValid] = useState<boolean>(false);

	// Decrypt a seed and store the reference. Must be called before signing.
	const create: TryCreateFunc = function (encryptedSeed, password) {
		return decryptDataRef(encryptedSeed, password).then((ref: number) => {
			setDataRef(ref);
			setValid(true);
		});
	};

	// Destroy the decrypted seed. Must be called before this leaves scope or
	// memory will leak.
	const destroy: TryDestroyFunc = function () {
		return destroyDataRef(dataRef).then(() => {
			setValid(false);
		});
	};

	return [dataRef, valid, create, destroy];
}
