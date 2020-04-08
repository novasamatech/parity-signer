import { useState } from 'react';

import { SeedRef, SeedRefClass } from 'utils/native';

type IsValidFunc = () => boolean;
type TryCreateFunc = (encryptedSeed: string, password: string) => Promise<void>;
type TryDestroyFunc = () => Promise<void>;
type TrySignFunc = (suriSuffix: string, message: string) => Promise<string>;
type TrySubstrateAddress = (
	suriSuffix: string,
	prefix: number
) => Promise<string>;
type TryBrainWalletAddress = () => Promise<string>;

export type SeedRefHooks = {
	isSeedRefValid: IsValidFunc;
	createSeedRef: TryCreateFunc;
	destroySeedRef: TryDestroyFunc;
	brainWalletSign: TrySignFunc;
	substrateSign: TrySignFunc;
	substrateAddress: TrySubstrateAddress;
	brainWalletAddress: TryBrainWalletAddress;
};

export function useSeedRef(): SeedRefHooks {
	const [seedRef, setSeedRef] = useState<SeedRefClass>(SeedRef);

	const isSeedRefValid: IsValidFunc = seedRef.isValid;

	// Decrypt a seed and store the reference. Must be called before signing.
	const createSeedRef: TryCreateFunc = function (encryptedSeed, password) {
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
	const brainWalletSign: TrySignFunc = seedRef.tryBrainWalletSign;

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const substrateSign: TrySignFunc = seedRef.trySubstrateSign;

	const substrateAddress: TrySubstrateAddress = seedRef.trySubstrateAddress;

	const brainWalletAddress: TryBrainWalletAddress =
		seedRef.tryBrainWalletAddress;

	return {
		brainWalletAddress,
		brainWalletSign,
		createSeedRef,
		destroySeedRef,
		isSeedRefValid,
		substrateAddress,
		substrateSign
	};
}
