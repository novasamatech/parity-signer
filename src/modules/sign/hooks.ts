import { useContext, useEffect, useState } from 'react';
import { GenericExtrinsicPayload } from '@polkadot/types';

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
		// was this line useful for anything?
		//if (getTypeRegistry === null) return;
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
				const extrinsicPayload = typeRegistry.createType(
					'ExtrinsicPayload',
					rawPayload,
					{
						version: ExtrinsicPayloadLatestVersion
					}
				);
				setPayload(extrinsicPayload);
				setIsProcessing(false);
			} catch (e) {
				//can't generate extrinsic payload, don't display.
				console.log('Payload details error', e);
			}
		}
	}, [rawPayload, networkKey, getTypeRegistry, networks]);

	return [isProcessing, payload];
}
