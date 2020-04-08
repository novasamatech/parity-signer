import { useState } from 'react';

import { SeedRef, SeedRefClass } from 'utils/native';

type IsValidFunc = () => boolean;
type TryCreateFunc = (encryptedSeed: string, password: string) => Promise<void>;
type TryDestroyFunc = () => Promise<void>;
type TrySignFunc = (message: string) => Promise<string>;

type SeedRefHooks = {
	isSeedRefValid: IsValidFunc;
	createSeedRef: TryCreateFunc;
	destroySeedRef: TryDestroyFunc;
	brainWalletSign: TrySignFunc;
	substrateSign: TrySignFunc;
};

export function useSeedRef(): SeedRefHooks {
	const [seedRef, setSeedRef] = useState<SeedRefClass>(SeedRef);

	const isSeedRefValid: IsValidFunc = function () {
		return seedRef.isValid();
	};

	// Decrypt a seed and store the reference. Must be called before signing.
	const createSeedRef: TryCreateFunc = function (
		encryptedSeed: string,
		password: string
	) {
		return seedRef.tryCreate(encryptedSeed, password).then(createdRef => {
			setSeedRef(createdRef);
		});
	};

	// Destroy the decrypted seed. Must be called before this leaves scope or
	// memory will leak.
	const destroySeedRef: TryDestroyFunc = function () {
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

	return {
		brainWalletSign,
		createSeedRef,
		destroySeedRef,
		isSeedRefValid,
		substrateSign
	};
}
