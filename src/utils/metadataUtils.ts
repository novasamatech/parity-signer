import { TypeRegistry } from '@polkadot/types';
import { Metadata } from '@polkadot/metadata';
import { expandMetadata } from '@polkadot/metadata/decorate';

import { allBuiltInMetadata } from 'constants/networkMetadataList';
import { saveMetadata } from 'utils/db';
import { MetadataHandle } from 'types/metadata';

export const metadataHandleToKey = (metadataHandle: MetadataHandle): string => {
	const metadataKey =
		metadataHandle.specName + '_v' + metadataHandle.specVersion;
	return metadataKey;
};

export const getMetadataHandleFromRaw = (
	metadataRaw: string
): MetadataHandle => {
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
		hash: metadataVersion.toString(),
		specName: metadataVersion.get('specName'),
		specVersion: metadataVersion.get('specVersion')
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
};

export async function populateMetadata(): Promise<void> {
	console.log('loading built-in metadata...');
	for (const metadataString of allBuiltInMetadata) {
		await saveMetadata(metadataString);
	}
}

//TODO: more elegance and auto-generation of this junk, as per issue #736
//export async function initBuiltInNetworks(): Promise<void> {
/*	await saveMetadata(centrifugeMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.CENTRIFUGE)));
	await saveMetadata(centrifugeAmberMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.CENTRIFUGE_AMBER)));
	await saveMetadata(kusamaMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.KUSAMA)));
	await saveMetadata(westendMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.WESTEND)));
	await saveMetadata(edgewareMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.EDGEWARE)));
	await saveMetadata(kulupuMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.KULUPU)));
	await saveMetadata(polkadotMetaData, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.POLKADOT)));
	await saveMetadata(rococoMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.ROCOCO)));*/
//}

/*
//const metadata = await api.rpc.state.getMetadata();


//console.log('all: ' + metadata.asLatest.modules[0].constants[4]);
console.log('value: ' + metadata.asLatest.modules[0].constants[4].value);

for(const moduleRecord of metadata.asLatest.modules)
  if(moduleRecord.name == "System")
    for(constantRecord of moduleRecord.constants)
      if(constantRecord.name == "Version")
        console.log('all: ' + constantRecord);

const metadata = await api.rpc.state.getMetadata();
const rtVersionFetch = await api.rpc.state.getRuntimeVersion();
var rtVersion = "";

for(const moduleRecord of metadata.asLatest.modules)
  if(moduleRecord.name == "System")
    for(constantRecord of moduleRecord.constants)
      if(constantRecord.name == "Version")
        rtVersion = constantRecord.value;
        
console.log('all: ' + rtVersion);

console.log('versiondata: ' + rtVersionFetch.specName);
console.log('versiondata: ' + rtVersionFetch.specVersion);
*/
