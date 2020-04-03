import { useState } from 'react';

import { SeedRef } from 'utils/native';

type IsValidFunc = () => boolean;
type TryCreateFunc = (encryptedSeed: string, password: string) => Promise<void>;
type TryDestroyFunc = () => Promise<void>;
type TrySignFunc = (message: string) => Promise<string>;

type SeedRefHooks = [IsValidFunc, TryCreateFunc, TryDestroyFunc, TrySignFunc];

export function useSeedRef(): SeedRefHooks {
	const [seedRef] = useState<SeedRef>(new SeedRef());

	const isValid: IsValidFunc = function () {
		return seedRef.isValid();
	};

	// Decrypt a seed and store the reference. Must be called before signing.
	const create: TryCreateFunc = function (encryptedSeed, password) {
		return seedRef.tryCreate(encryptedSeed, password);
	};

	// Destroy the decrypted seed. Must be called before this leaves scope or
	// memory will leak.
	const destroy: TryDestroyFunc = function () {
		return seedRef.tryDestroy();
	};

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const brainWalletSign: TrySignFunc = function () {
		return seedRef.tryBrainWalletSign(message);
	};

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const substrateSign: TrySignFunc = function () {
		return seedRef.trySubstrateSign(message);
	};

	return [isValid, create, destroy, brainWalletSign, substrateSign];
}
