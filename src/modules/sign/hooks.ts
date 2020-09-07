import { useContext, useEffect, useState } from 'react';
import { GenericExtrinsicPayload } from '@polkadot/types';

import { ExtrinsicPayloadLatestVersion } from 'constants/chainData';
import { NetworksContext } from 'stores/NetworkContext';
import { RegistriesContext } from 'stores/RegistriesContext';

export function usePayloadDetails(
	dataToSign: Uint8Array | string,
	networkKey: string
): [boolean, GenericExtrinsicPayload | null] {
	const [payload, setPayload] = useState<GenericExtrinsicPayload | null>(null);
	const [isProcessing, setIsProcessing] = useState<boolean>(false);
	const { networks } = useContext(NetworksContext);
	const { getTypeRegistry } = useContext(RegistriesContext);

	useEffect(() => {
		setIsProcessing(true);
		const typeRegistry = getTypeRegistry(networks, networkKey);
		if (typeRegistry === null || typeof dataToSign === 'string') {
			setIsProcessing(false);
			return;
		} else {
			const extrinsicPayload = typeRegistry.createType(
				'ExtrinsicPayload',
				dataToSign,
				{
					version: ExtrinsicPayloadLatestVersion
				}
			);
			setPayload(extrinsicPayload);
			setIsProcessing(false);
		}
	}, [dataToSign, networkKey, getTypeRegistry, networks]);

	return [isProcessing, payload];
}
