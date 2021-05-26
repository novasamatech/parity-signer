import { useContext, useEffect, useState } from 'react';
import { GenericExtrinsicPayload } from '@polkadot/types';

import { parseTransaction } from 'utils/native';
import { ExtrinsicPayloadLatestVersion } from 'constants/chainData';
import { NetworksContext } from 'stores/NetworkContext';

export function usePayloadDetails(
	rawPayload: Uint8Array | string | null,
	networkKey: string
): [boolean, GenericExtrinsicPayload | null] {
	const [payload, setPayload] = useState<GenericExtrinsicPayload | null>(null);
	const [isProcessing, setIsProcessing] = useState<boolean>(false);
	const { networks, getTypeRegistry } = useContext(NetworksContext);

	useEffect(() => {
		setIsProcessing(true);
		const typeRegistry = getTypeRegistry(networkKey);
		if (
			typeRegistry === null ||
			typeof rawPayload === 'string' ||
			!networks.get(networkKey) ||
			!networks.get(networkKey)!.metadata //2nd ! should not be needed
		) {
			setIsProcessing(false);
			return;
		} else {
			try {
				const generateCards = async function (encoded: string): Promise<void> {
					const extrinsicPayloadCards = await parseTransaction(
						encoded,
						'',
						'',
						''
					);
					setPayload(extrinsicPayloadCards);
					setIsProcessing(false);
				};
				generateCards(rawPayload);
			} catch (e) {
				//can't generate extrinsic payload, don't display.
				console.log('Payload details error', e);
			}
		}
	}, [rawPayload, networkKey, getTypeRegistry, networks]);

	return [isProcessing, payload];
}
