import { useContext, useMemo } from 'react';

import { SeedRefsContext, SeedRefsState } from 'stores/SeedRefStore';
import { SeedRefClass } from 'utils/native';
import { deepCopyMap } from 'stores/utils';

export type TryCreateFunc = (password: string) => Promise<void>;
export type TryDestroyFunc = () => Promise<void>;
export type TrySignFunc = (
	suriSuffix: string,
	message: string
) => Promise<string>;
export type TryBrainWalletSignFunc = (message: string) => Promise<string>;
export type TrySubstrateAddress = (
	suriSuffix: string,
	prefix: number
) => Promise<string>;
export type TryBrainWalletAddress = () => Promise<string>;
export type TrySubstrateSecret = (suriSuffix: string) => Promise<string>;

export type SeedRefHooks = {
	isSeedRefValid: boolean;
	createSeedRef: TryCreateFunc;
	destroySeedRef: TryDestroyFunc;
	brainWalletSign: TryBrainWalletSignFunc;
	substrateSign: TrySignFunc;
	substrateAddress: TrySubstrateAddress;
	brainWalletAddress: TryBrainWalletAddress;
	substrateSecret: TrySubstrateSecret;
};

export type CreateSeedRefWithNewSeed = (
	encryptedSeed: string,
	password: string
) => Promise<void>;

export function useNewSeedRef(): CreateSeedRefWithNewSeed {
	const { seedRefs, setSeedRefs } = useContext<SeedRefsState>(SeedRefsContext);
	return async (encryptedSeed, password): Promise<void> => {
		if (!seedRefs.has(encryptedSeed)) {
			const seedRef = new SeedRefClass();
			await seedRef.tryCreate(encryptedSeed, password);
			const newSeedRefs = deepCopyMap(seedRefs);
			newSeedRefs.set(encryptedSeed, seedRef);
			setSeedRefs(newSeedRefs);
		}
	};
}

export function useSeedRef(encryptedSeed: string): SeedRefHooks {
	const { seedRefs, setSeedRefs } = useContext<SeedRefsState>(SeedRefsContext);
	const seedRef = useMemo(() => {
		if (seedRefs.has(encryptedSeed)) {
			return seedRefs.get(encryptedSeed)!;
		}
		const newSeedRef = new SeedRefClass();
		const newSeedRefs = deepCopyMap(seedRefs);
		newSeedRefs.set(encryptedSeed, newSeedRef);
		setSeedRefs(newSeedRefs);
		return newSeedRef;
	}, [seedRefs, setSeedRefs, encryptedSeed]);

	const isSeedRefValid: boolean = seedRef.isValid();

	// Decrypt a seed and store the reference. Must be called before signing.
	const createSeedRef: TryCreateFunc = async function (password) {
		await seedRef.tryCreate(encryptedSeed, password);
		const newSeedRefs = deepCopyMap(seedRefs);
		newSeedRefs.set(encryptedSeed, seedRef);
		setSeedRefs(newSeedRefs);
	};

	// Destroy the decrypted seed. Must be called before this leaves scope or
	// memory will leak.
	const destroySeedRef: TryDestroyFunc = function () {
		return seedRef.tryDestroy().then(() => {
			const newSeedRefs = new Map();
			for (const [key, value] of seedRefs.entries())
				if (key !== encryptedSeed) newSeedRefs.set(key, value);
			setSeedRefs(newSeedRefs);
		});
	};

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const brainWalletSign: TryBrainWalletSignFunc = seedRef.tryBrainWalletSign.bind(
		seedRef
	);

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	const substrateSign: TrySignFunc = seedRef.trySubstrateSign.bind(seedRef);

	const substrateAddress: TrySubstrateAddress = seedRef.trySubstrateAddress.bind(
		seedRef
	);

	const brainWalletAddress: TryBrainWalletAddress = seedRef.tryBrainWalletAddress.bind(
		seedRef
	);

	const substrateSecret: TrySubstrateSecret = seedRef.trySubstrateSecret.bind(
		seedRef
	);

	return {
		brainWalletAddress,
		brainWalletSign,
		createSeedRef,
		destroySeedRef,
		isSeedRefValid,
		substrateAddress,
		substrateSecret,
		substrateSign
	};
}
