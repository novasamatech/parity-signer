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
import { AccountsContextState } from 'stores/AccountsContext';
import { QrInfo, ScannerContextState } from 'stores/ScannerContext';
import { FoundIdentityAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { RootStackParamList } from 'types/routes';
import { isMultipartData, ParsedData, TxRequestData } from 'types/scannerTypes';
import { assertNever } from 'types/utilTypes';
import {
	constructDataFromBytes,
	isAddressString,
	isJsonString,
	rawDataToU8A
} from 'utils/decoders';
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
	accounts: AccountsContextState,
	scannerStore: ScannerContextState,
	seedRefs: Map<string, SeedRefClass>
): Promise<void> {
	async function parseQrData(): Promise<ParsedData> {
		if (isAddressString(txRequestData.data)) {
			throw new Error(strings.ERROR_ADDRESS_MESSAGE);
		} else if (isJsonString(txRequestData.data)) {
			// Ethereum Legacy
			return JSON.parse(txRequestData.data);
		} else if (!scannerStore.state.multipartComplete) {
			const strippedData = rawDataToU8A(txRequestData.rawData);
			if (strippedData === null) throw new Error(strings.ERROR_NO_RAW_DATA);
			const parsedData = await constructDataFromBytes(strippedData, false);
			return parsedData;
		} else {
			throw new Error(strings.ERROR_NO_RAW_DATA);
		}
	}

	async function unlockSeedAndSign(
		sender: FoundIdentityAccount,
		qrInfo: QrInfo
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
				seedRef.tryBrainWalletSign.bind(seedRef),
				qrInfo
			);
		} else {
			const suriSuffix = constructSuriSuffix({
				derivePath: sender.path,
				password
			});
			await scannerStore.signSubstrateData(
				seedRef.trySubstrateSign.bind(seedRef),
				suriSuffix,
				qrInfo
			);
		}
	}

	try {
		const parsedData = await parseQrData();

		let unsignedData;
		if (isMultipartData(parsedData)) {
			unsignedData = await scannerStore.setPartData(
				parsedData.currentFrame,
				parsedData.frameCount,
				parsedData.partData
			);
			if (unsignedData === null) return;
		} else {
			unsignedData = parsedData;
		}
		const qrInfo = await scannerStore.setData(accounts, unsignedData);
		scannerStore.clearMultipartProgress();
		const { sender, type } = qrInfo;
		if (!sender)
			return showErrorMessage(
				strings.ERROR_TITLE,
				strings.ERROR_NO_SENDER_FOUND
			);
		if (sender.isLegacy) {
			if (type === 'transaction') {
				return navigation.navigate('AccountUnlockAndSign', {
					next: 'SignedTx'
				});
			} else {
				return navigation.navigate('AccountUnlockAndSign', {
					next: 'SignedMessage'
				});
			}
		}
		await unlockSeedAndSign(sender, qrInfo);
		if (type === 'transaction') {
			navigateToSignedTx(navigation);
		} else {
			navigateToSignedMessage(navigation);
		}
	} catch (e) {
		return showErrorMessage(strings.ERROR_TITLE, e.message);
	}
}
