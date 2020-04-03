import { useState } from 'react';

import {
	decryptDataRef,
	destroyDataRef,
	brainWalletSignWithRef,
	substrateSignWithRef
} from 'utils/native';

type TryCreateFunc = (encryptedSeed: string, password: string) => Promise<void>;
type TryDestroyFunc = () => Promise<void>;
type TrySignFunc = (message: string) => Promise<string>;

type SeedRefHooks = [
	number,
	boolean,
	TryCreateFunc,
	TryDestroyFunc,
	TrySignFunc
];

export function useSeedRef(): SeedRefHooks {
	const [dataRef, setDataRef] = useState<number>(0);
	const [valid, setValid] = useState<boolean>(false);

	// Decrypt a seed and store the reference. Must be called before signing.
	const create: TryCreateFunc = function (encryptedSeed, password) {
		// Seed reference was already created.
		if (valid) {
			throw new Error('cannot create a seed reference when one already exists');
		}
		return decryptDataRef(encryptedSeed, password).then((ref: number) => {
			setDataRef(ref);
			setValid(true);
		});
	};

	// Destroy the decrypted seed. Must be called before this leaves scope or
	// memory will leak.
	const destroy: TryDestroyFunc = function () {
		if (!valid) {
			// Seed reference was never created or was already destroyed.
			throw new Error('cannot destroy an invalid seed reference');
		}
		return destroyDataRef(dataRef).then(() => {
			setValid(false);
		});
	};

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const brainWalletSign: TrySignFunc = function () {
		if (!valid) {
			// Seed reference was never created or was already destroyed.
			throw new Error('cannot sign with an invalid seed reference');
		}
		return brainWalletSignWithRef(dataRef, message);
	};

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const substrateSign: TrySignFunc = function () {
		if (!valid) {
			// Seed reference was never created or was already destroyed.
			throw new Error('cannot sign with an invalid seed reference');
		}
		return substrateSignWithRef(dataRef, message);
	};

	return [dataRef, valid, create, destroy, brainWalletSign, substrateSign];
}
