// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import {
	NetworkProtocols,
	NETWORK_LIST,
	SubstrateNetworkKeys
} from 'constants/networkSpecs';
import { UnlockedAccount } from 'types/identityTypes';
import {
	EthereumNetworkParams,
	isSubstrateNetworkParams
} from 'types/networkSpecsTypes';
import { ValidSeed } from 'types/utilTypes';

export function generateAccountId({
	address,
	networkKey
}: {
	address: string;
	networkKey: string;
}): string {
	if (
		typeof address !== 'string' ||
		address.length === 0 ||
		!networkKey ||
		!NETWORK_LIST[networkKey]
	) {
		throw new Error(
			"Couldn't create an accountId. Address or networkKey missing, or network key was invalid."
		);
	}

	const networkParams = NETWORK_LIST[networkKey];
	const { protocol } = networkParams;

	if (isSubstrateNetworkParams(networkParams)) {
		const { genesisHash } = networkParams;
		return `${protocol}:${address}:${genesisHash ?? ''}`;
	} else if (protocol === NetworkProtocols.UNKNOWN) {
		return `substrate:${address}`;
	} else {
		const { ethereumChainId } = networkParams as EthereumNetworkParams;
		return `${protocol}:0x${address}@${ethereumChainId}`;
	}
}

export function emptyAccount(
	address = '',
	networkKey: string = SubstrateNetworkKeys.KUSAMA
): UnlockedAccount {
	return {
		address: address,
		createdAt: new Date().getTime(),
		derivationPassword: '',
		derivationPath: '',
		encryptedSeed: undefined,
		isLegacy: true,
		name: '',
		networkKey: networkKey,
		seed: '',
		seedPhrase: '',
		updatedAt: new Date().getTime(),
		validBip39Seed: false
	};
}

export function validateSeed(seed: string, validBip39Seed: boolean): ValidSeed {
	if (!seed || seed.length === 0) {
		return {
			accountRecoveryAllowed: false,
			bip39: false,
			reason: 'A seed phrase is required.',
			valid: false
		};
	}

	const words = validBip39Seed ? seed.trimEnd().split(' ') : seed.split(' ');

	for (const word of words) {
		if (word === '') {
			return {
				accountRecoveryAllowed: true,
				bip39: false,
				reason: 'Extra whitespace found.',
				valid: false
			};
		}
	}

	if (!validBip39Seed) {
		return {
			accountRecoveryAllowed: true,
			bip39: false,
			reason:
				'This recovery phrase is not a valid BIP39 seed, will be treated as a legacy Parity brain wallet.',
			valid: false
		};
	}

	return {
		bip39: true,
		reason: null,
		valid: true
	};
}
