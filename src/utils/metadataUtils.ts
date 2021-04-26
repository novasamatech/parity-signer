import { TypeRegistry } from '@polkadot/types';
import { Metadata } from '@polkadot/metadata';
import { expandMetadata } from '@polkadot/metadata/decorate';

import { MetadataHandle } from 'types/metadata';
import { blake2b, generateMetadataHandle } from 'utils/native';

export const metadataHandleToKey = (metadataHandle: MetadataHandle): string => {
	if (metadataHandle.specVersion) {
		return metadataHandle.specName + '_v' + metadataHandle.specVersion;
	} else {
		return metadataHandle.hash;
	}
};

async function getMetadataHash(metadataRaw: string): Promise<string> {
	return await blake2b(metadataRaw.substr(2));
}

export async function getMetadataHandleFromRaw(
	metadataRaw: string
): Promise<MetadataHandle> {
	metadataHandle = await generateMetadataHandle(metadataRaw);
	return metadataHandle;
}
