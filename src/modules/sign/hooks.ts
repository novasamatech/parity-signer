import { ExtrinsicPayloadLatestVersion } from 'constants/chainData';
import { useContext, useEffect, useState } from 'react';
import { RegistriesContext } from 'stores/RegistriesContext';

import { GenericExtrinsicPayload } from '@polkadot/types';

import { NetworksContext } from '../../context';

export function usePayloadDetails(rawPayload: Uint8Array | string | null, networkKey?: string): [boolean, GenericExtrinsicPayload | null] {
	const [payload, setPayload] = useState<GenericExtrinsicPayload | null>(null);
	const [isProcessing, setIsProcessing] = useState<boolean>(false);
	const { networks } = useContext(NetworksContext);
	const { getTypeRegistry } = useContext(RegistriesContext);

	useEffect(() => {
		setIsProcessing(true);
		if (getTypeRegistry === null || !networkKey) return;
		const typeRegistry = getTypeRegistry(networks, networkKey);

		if (typeRegistry === null || typeof rawPayload === 'string') {
			setIsProcessing(false);

			return;
		} else {
			try {
				const extrinsicPayload = typeRegistry.createType('ExtrinsicPayload',
					rawPayload,
					{ version: ExtrinsicPayloadLatestVersion });

				setPayload(extrinsicPayload);
				setIsProcessing(false);
			} catch (e) {
				//can't generate extrinsic payload, don't display.
				console.log('error', e);
			}
		}
	}, [rawPayload, networkKey, getTypeRegistry, networks]);

	return [isProcessing, payload];
}
