// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import { StackNavigationProp } from '@react-navigation/stack';

import { NETWORK_LIST } from 'constants/networkSpecs';
import strings from 'modules/sign/strings';
import AccountsStore from 'stores/AccountsStore';
import ScannerStore from 'stores/ScannerStore';
import { FoundIdentityAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { RootStackParamList } from 'types/routes';
import { TxRequestData } from 'types/scannerTypes';
import { isAddressString, isJsonString, rawDataToU8A } from 'utils/decoders';
import { getIdentityFromSender } from 'utils/identitiesUtils';
import { SeedRefClass } from 'utils/native';
import {
	navigateToSignedMessage,
	navigateToSignedTx,
	unlockSeedPhrase,
	unlockSeedPhraseWithPassword
} from 'utils/navigationHelpers';
import { constructSuriSuffix } from 'utils/suri';

function getSeedRef(
	encryptedSeed: string,
	seedRefs: Map<string, SeedRefClass>
): SeedRefClass | undefined {
	if (seedRefs.has(encryptedSeed)) {
		return seedRefs.get(encryptedSeed);
	}
}

export async function processBarCode(
	showErrorMessage: (title: string, message: string) => void,
	txRequestData: TxRequestData,
	navigation: StackNavigationProp<RootStackParamList, 'QrScanner'>,
	accounts: AccountsStore,
	scannerStore: ScannerStore,
	seedRefs: Map<string, SeedRefClass>
): Promise<void> {
	async function parseQrData(): Promise<void> {
		if (isAddressString(txRequestData.data)) {
			throw new Error(strings.ERROR_ADDRESS_MESSAGE);
		} else if (isJsonString(txRequestData.data)) {
			// Ethereum Legacy
			await scannerStore.setUnsigned(txRequestData.data);
		} else if (!scannerStore.isMultipartComplete()) {
			const strippedData = rawDataToU8A(txRequestData.rawData);
			if (strippedData === null) throw new Error(strings.ERROR_NO_RAW_DATA);
			await scannerStore.setParsedData(strippedData, false);
		}
	}

	async function unlockSeedAndSign(
		sender: FoundIdentityAccount
	): Promise<void> {
		const senderNetworkParams = NETWORK_LIST[sender.networkKey];
		const isEthereum = isEthereumNetworkParams(senderNetworkParams);

		// 1. check if sender existed
		const senderIdentity = getIdentityFromSender(
			sender,
			accounts.state.identities
		);
		if (!senderIdentity) throw new Error(strings.ERROR_NO_SENDER_IDENTITY);

		let seedRef = getSeedRef(sender.encryptedSeed, seedRefs);
		let password = '';
		// 2. unlock and get Seed reference
		if (seedRef === undefined || !seedRef.isValid()) {
			if (sender.hasPassword) {
				//need unlock with password
				password = await unlockSeedPhraseWithPassword(
					navigation,
					false,
					senderIdentity
				);
			} else {
				await unlockSeedPhrase(navigation, false, senderIdentity);
			}
			seedRef = getSeedRef(sender.encryptedSeed, seedRefs)!;
		} else {
			if (sender.hasPassword) {
				password = await unlockSeedPhraseWithPassword(
					navigation,
					true,
					senderIdentity
				);
			}
		}
		// 3. sign data
		if (isEthereum) {
			await scannerStore.signEthereumData(
				seedRef.tryBrainWalletSign.bind(seedRef)
			);
		} else {
			const suriSuffix = constructSuriSuffix({
				derivePath: sender.path,
				password
			});
			await scannerStore.signSubstrateData(
				seedRef.trySubstrateSign.bind(seedRef),
				suriSuffix
			);
		}
	}

	try {
		await parseQrData();
		if (scannerStore.getUnsigned() === null) return;
		await scannerStore.setData(accounts);
		scannerStore.clearMultipartProgress();
		const sender = scannerStore.getSender();
		if (!sender)
			return showErrorMessage(
				strings.ERROR_TITLE,
				strings.ERROR_NO_SENDER_FOUND
			);
		if (sender.isLegacy) {
			if (scannerStore.getType() === 'transaction') {
				return navigation.navigate('AccountUnlockAndSign', {
					next: 'SignedTx'
				});
			} else {
				return navigation.navigate('AccountUnlockAndSign', {
					next: 'SignedMessage'
				});
			}
		}
		await unlockSeedAndSign(sender);
		if (scannerStore.getType() === 'transaction') {
			navigateToSignedTx(navigation);
		} else {
			navigateToSignedMessage(navigation);
		}
	} catch (e) {
		return showErrorMessage(strings.ERROR_TITLE, e.message);
	}
}
