import { useState } from 'react';

import { SeedRef } from 'utils/native';

type IsValidFunc = () => boolean;
type TryCreateFunc = (encryptedSeed: string, password: string) => Promise<void>;
type TryDestroyFunc = () => Promise<void>;
type TrySignFunc = (message: string) => Promise<string>;

type SeedRefHooks = [IsValidFunc, TryCreateFunc, TryDestroyFunc, TrySignFunc];

export function useSeedRef(): SeedRefHooks {
	const [seedRef, setSeedRef] = useState<SeedRef>(new SeedRef());

	const isValid: IsValidFunc = function () {
		return seedRef.isValid();
	};

	// Decrypt a seed and store the reference. Must be called before signing.
	const create: TryCreateFunc = function (
		encryptedSeed: string,
		password: string
	) {
		return seedRef.tryCreate(encryptedSeed, password).then(createdRef => {
			setSeedRef(createdRef);
		});
	};

	// Destroy the decrypted seed. Must be called before this leaves scope or
	// memory will leak.
	const destroy: TryDestroyFunc = function () {
		return seedRef.tryDestroy().then(destroyedRef => {
			setSeedRef(destroyedRef);
		});
	};

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const brainWalletSign: TrySignFunc = function (message: string) {
		return seedRef.tryBrainWalletSign(message);
	};

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const substrateSign: TrySignFunc = function (message: string) {
		return seedRef.trySubstrateSign(message);
	};

	return [isValid, create, destroy, brainWalletSign, substrateSign];
}
