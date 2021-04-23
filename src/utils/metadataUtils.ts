import { TypeRegistry } from '@polkadot/types';
import { Metadata } from '@polkadot/metadata';
import { expandMetadata } from '@polkadot/metadata/decorate';

import { MetadataHandle } from 'types/metadata';
import { blake2b } from 'utils/native';

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
	try {
		const registry = new TypeRegistry();
		const metadata = new Metadata(registry, metadataRaw);
		registry.setMetadata(metadata);
		const decorated = expandMetadata(registry, metadata);
		const metadataVersion = (decorated.consts.system.version as unknown) as Map<
			string,
			any
		>;
		(metadataVersion as unknown) as Map<string, any>;
		const metadataHandle: MetadataHandle = {
			hash: await getMetadataHash(metadataRaw),
			specName: metadataVersion.get('specName').toString(),
			specVersion: parseInt(metadataVersion.get('specVersion'), 10)
		};
		return metadataHandle;
		//this would be the proper way to do it
		/*
		for (const moduleRecord of metadata.asLatest.modules)
			if (moduleRecord.name === 'System')
				for (constantRecord of moduleRecord.constants)
					if (constantRecord.name === 'Version')
						runtimeVersion = constantRecord.value;
		//decode runtimeVersion;
      		*/
	} catch (e) {
		console.log(e);
		const metadataHandle: MetadataHandle = {
			hash: await getMetadataHash(metadataRaw),
			specName: '',
			specVersion: 0
		};
		return metadataHandle;
	}
}
