import { useContext } from 'react';

import { TypeRegistry } from '@polkadot/types';
import { Metadata } from '@polkadot/metadata';
import { SubstrateNetworkKeys } from 'constants/networkSpecs';
import { NetworksContext } from 'stores/NetworkContext';

export const metadataHandleToKey = (
	metadataHandle: MetadataHandle
): string => {
	const metadataKey = metadataHandle.spec_name + '_v' + metadataHandle.spec_version;
	return metadataKey;
};

export const getRuntimeVersionFromRaw = (
	metadataRaw: string
): string => {
	const tempRegistry = new TypeRegistry();
	const metadata = new Metadata(tempRegistry, metadataRaw);

	for(const moduleRecord of metadata.asLatest.modules)
		if(moduleRecord.name == "System")
			for(constantRecord of moduleRecord.constants)
				if(constantRecord.name == "Version")
					runtimeVersion = constantRecord.value;
	return runtimeVersion;
}

//TODO: more elegance and auto-generation of this junk, as per issue #736
export async function initBuiltInNetworks(): Promise<void> {
	const { getNetwork } = useContext(NetworksContext);
/*	await saveMetadata(centrifugeMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.CENTRIFUGE)));
	await saveMetadata(centrifugeAmberMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.CENTRIFUGE_AMBER)));
	await saveMetadata(kusamaMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.KUSAMA)));
	await saveMetadata(westendMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.WESTEND)));
	await saveMetadata(edgewareMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.EDGEWARE)));
	await saveMetadata(kulupuMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.KULUPU)));
	await saveMetadata(polkadotMetaData, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.POLKADOT)));
	await saveMetadata(rococoMetadata, metadataHandleToKey(allNetworks.get(SubstrateNetworkKeys.ROCOCO)));*/
}

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
