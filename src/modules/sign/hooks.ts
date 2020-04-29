import { NETWORK_LIST } from 'constants/networkSpecs';
import text from 'modules/sign/texts';
import ScannerStore from 'stores/ScannerStore';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';

export function useSender(scannerStore: ScannerStore) {
	const sender = scannerStore.getSender()!;
	const senderNetworkParams = NETWORK_LIST[sender.networkKey];
	// if it is legacy account
	const isEthereum = isEthereumNetworkParams(senderNetworkParams);
}
