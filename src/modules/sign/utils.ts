import { StackNavigationProp } from '@react-navigation/stack';

import { NETWORK_LIST } from 'constants/networkSpecs';
import text from 'modules/sign/texts';
import AccountsStore from 'stores/AccountsStore';
import ScannerStore from 'stores/ScannerStore';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { RootStackParamList } from 'types/routes';
import { TxRequestData } from 'types/scannerTypes';
import { isAddressString, isJsonString, rawDataToU8A } from 'utils/decoders';
import { getIdentityFromSender } from 'utils/identitiesUtils';
import { SeedRefClass } from 'utils/native';
import {
	unlockAndReturnSeedRef,
	unlockSeedPhraseWithPasswordAndReturnSeedRef
} from 'utils/navigationHelpers';
import { constructSuriSuffix } from 'utils/suri';

function getSeedRef(
	encryptedSeed: string,
	seedRefs: Map<string, SeedRefClass>
): SeedRefClass | undefined {
	if (seedRefs.has(encryptedSeed)) {
		return seedRefs.get(encryptedSeed)!;
	}
}

export async function processBarCode(
	showErrorMessage: any,
	txRequestData: TxRequestData,
	navigation: StackNavigationProp<RootStackParamList, 'QrScanner'>,
	accounts: AccountsStore,
	scannerStore: ScannerStore,
	seedRefs: Map<string, SeedRefClass>
): Promise<void> {
	async function parseQrData(): Promise<void> {
		if (isAddressString(txRequestData.data)) {
			throw new Error(text.ADDRESS_ERROR_MESSAGE);
		} else if (isJsonString(txRequestData.data)) {
			// Ethereum Legacy
			await scannerStore.setUnsigned(txRequestData.data);
		} else if (!scannerStore.isMultipartComplete()) {
			const strippedData = rawDataToU8A(txRequestData.rawData);
			if (strippedData === null) throw new Error(text.NO_RAW_DATA_ERROR);
			await scannerStore.setParsedData(strippedData, accounts, false);
		}
	}

	async function unlockSeedAndSign(): Promise<void> {
		const sender = scannerStore.getSender();
		if (!sender) throw new Error(text.NO_SENDER_FOUND_ERROR);
		const senderNetworkParams = NETWORK_LIST[sender.networkKey];
		// if it is legacy account
		const isEthereum = isEthereumNetworkParams(senderNetworkParams);
		if (sender.isLegacy) {
			navigation.navigate('AccountUnlockAndSign', {
				next: 'SignedMessage'
			});
			return;
		}

		// check if sender existed
		const senderIdentity = getIdentityFromSender(
			sender,
			accounts.state.identities
		);
		if (!senderIdentity) throw new Error(text.NO_SENDER_IDENTITY_ERROR);

		let seedRef = getSeedRef(sender.encryptedSeed, seedRefs);
		let password = '';

		// unlock and get Seed reference
		if (seedRef === undefined || !seedRef.isValid()) {
			if (sender.hasPassword) {
				//need unlock with password
				[
					password,
					seedRef
				] = await unlockSeedPhraseWithPasswordAndReturnSeedRef(
					navigation,
					senderIdentity
				);
			}
			seedRef = await unlockAndReturnSeedRef(navigation, senderIdentity);
		}

		// sign data
		if (isEthereum) {
			await scannerStore.signEthereumData(seedRef.tryBrainWalletSign);
		} else {
			const suriSuffix = constructSuriSuffix({
				derivePath: sender.path,
				password
			});
			await scannerStore.signSubstrateData(
				seedRef.trySubstrateSign,
				suriSuffix
			);
		}
	}

	try {
		await parseQrData();
		if (scannerStore.getUnsigned() === null) return;
		await scannerStore.setData(accounts);
		scannerStore.clearMultipartProgress();
		await unlockSeedAndSign();
		if (scannerStore.getType() === 'transaction') {
			navigation.navigate('SignedTx');
		} else {
			navigation.navigate('SignedMessage');
		}
	} catch (e) {
		return showErrorMessage(scannerStore, text.ERROR_TITLE, e.message);
	}
}
