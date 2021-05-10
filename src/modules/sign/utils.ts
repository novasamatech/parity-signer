// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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
import { useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import { useContext } from 'react';

import strings from 'modules/sign/strings';
import { AccountsContext } from 'stores/AccountsContext';
import { NetworksContextState } from 'stores/NetworkContext';
import { ScannerContext } from 'stores/ScannerContext';
import { SeedRefsContext, SeedRefsState } from 'stores/SeedRefStore';
import { FoundIdentityAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkTypes';
import { RootStackParamList } from 'types/routes';
import {
	CompletedParsedData,
	EthereumParsedData,
	isMultiFramesInfo,
	isMultipartData,
	isNetworkParsedData,
	NetworkParsedData,
	ParsedData,
	QrInfo,
	SubstrateParsedData,
	TxRequestData
} from 'types/scannerTypes';
import {
	constructDataFromBytes,
	isAddressString,
	isJsonString,
	rawDataToU8A
} from 'utils/decoders';
import { getIdentityFromSender } from 'utils/identitiesUtils';
import { SeedRefClass } from 'utils/native';
import {
	previewTransactionAndApprove,
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

export function useProcessBarCode(
	showAlertMessage: (
		title: string,
		message: string,
		isSuccess?: boolean
	) => void,
	networksContextState: NetworksContextState
): (txRequestData: TxRequestData) => Promise<void> {
	const { allNetworks, networks } = networksContextState;
	const accountsStore = useContext(AccountsContext);
	const scannerStore = useContext(ScannerContext);
	const { seedRefs } = useContext<SeedRefsState>(SeedRefsContext);
	const navigation: StackNavigationProp<
		RootStackParamList,
		'QrScanner'
	> = useNavigation();

	async function parseQrData(
		txRequestData: TxRequestData
	): Promise<ParsedData> {
		if (isAddressString(txRequestData.data)) {
			throw new Error(strings.ERROR_ADDRESS_MESSAGE);
		} else if (isJsonString(txRequestData.data)) {
			// Add Network
			const parsedJsonData = JSON.parse(txRequestData.data);
			if (parsedJsonData.hasOwnProperty('genesisHash')) {
				return {
					action: 'addNetwork',
					data: parsedJsonData
				} as NetworkParsedData;
			}
			// Ethereum Legacy
			return parsedJsonData;
		} else if (!scannerStore.state.multipartComplete) {
			const strippedData = rawDataToU8A(txRequestData.rawData);
			if (strippedData === null) throw new Error(strings.ERROR_NO_RAW_DATA);
			const parsedData = await constructDataFromBytes(
				strippedData,
				false,
				networks
			);
			return parsedData;
		} else {
			throw new Error(strings.ERROR_NO_RAW_DATA);
		}
	}

	async function checkMultiFramesData(
		parsedData: SubstrateParsedData | EthereumParsedData
	): Promise<null | CompletedParsedData> {
		if (isMultipartData(parsedData)) {
			const multiFramesResult = await scannerStore.setPartData(
				parsedData.currentFrame,
				parsedData.frameCount,
				parsedData.partData,
				networksContextState
			);
			if (isMultiFramesInfo(multiFramesResult)) {
				return null;
			}
			//Otherwise all the frames are assembled as completed parsed data
			return multiFramesResult;
		} else {
			return parsedData;
		}
	}

	async function _unlockSeedAndSign(
		sender: FoundIdentityAccount,
		qrInfo: QrInfo
	): Promise<void> {
		const senderNetworkParams = allNetworks.get(sender.networkKey);
		if (!senderNetworkParams) throw new Error(strings.ERROR_NO_NETWORK);
		const isEthereum = isEthereumNetworkParams(senderNetworkParams);

		// 1. check if sender existed
		const senderIdentity = getIdentityFromSender(
			sender,
			accountsStore.state.identities
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
				qrInfo,
				networks
			);
		}
	}

	async function unlockAndNavigationToSignedQR(qrInfo: QrInfo): Promise<void> {
		const { sender, type } = qrInfo;
		if (type === 'transaction') await previewTransactionAndApprove(navigation);
		if (!sender)
			return showAlertMessage(
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
		const seedRef = getSeedRef(sender.encryptedSeed, seedRefs);
		const isSeedRefInvalid = seedRef && seedRef.isValid();

		await _unlockSeedAndSign(sender, qrInfo);
		const nextRoute = type === 'transaction' ? 'SignedTx' : 'SignedMessage';

		if (isSeedRefInvalid) {
			navigation.navigate(nextRoute);
		} else {
			navigation.replace(nextRoute);
		}
	}

	function addNewNetwork(networkParsedData: NetworkParsedData): void {
		networksContextState.addNetwork(networkParsedData);
		return showAlertMessage(
			strings.SUCCESS_TITLE,
			strings.SUCCESS_ADD_NETWORK + networkParsedData.data.title,
			true
		);
	}

	async function processBarCode(txRequestData: TxRequestData): Promise<void> {
		try {
			const parsedData = await parseQrData(txRequestData);
			if (isNetworkParsedData(parsedData)) {
				return addNewNetwork(parsedData);
			}
			const unsignedData = await checkMultiFramesData(parsedData);
			if (unsignedData === null) return;
			const qrInfo = await scannerStore.setData(
				accountsStore,
				unsignedData,
				networksContextState
			);
			await unlockAndNavigationToSignedQR(qrInfo);
			scannerStore.clearMultipartProgress();
		} catch (e) {
			return showAlertMessage(strings.ERROR_TITLE, e.message);
		}
	}

	return processBarCode;
}
