import { useContext, useEffect, useState } from 'react';
import { GenericExtrinsicPayload } from '@polkadot/types';

import { ExtrinsicPayloadLatestVersion } from 'constants/chainData';
import { NetworksContext } from 'stores/NetworkContext';
import { RegistriesContext } from 'stores/RegistriesContext';

export function usePayloadDetails(
	rawPayload: Uint8Array | string | null,
	networkKey: string
): [boolean, GenericExtrinsicPayload | null] {
	const [payload, setPayload] = useState<GenericExtrinsicPayload | null>(null);
	const [isProcessing, setIsProcessing] = useState<boolean>(false);
	const { networks } = useContext(NetworksContext);
	const { getTypeRegistry } = useContext(RegistriesContext);

	useEffect(() => {
		setIsProcessing(true);
		// was this line useful for anything?
		//if (getTypeRegistry === null) return;
		console.log('generating payload card...');
		console.log(networkKey);
		console.log(networks.get(networkKey));
		console.log(networks.get(networkKey).metadata);
		const typeRegistry = getTypeRegistry(networks, networkKey, networks.get(networkKey).metadata);
		if (typeRegistry === null || typeof rawPayload === 'string') {
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
