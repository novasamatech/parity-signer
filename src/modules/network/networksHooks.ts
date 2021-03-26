/*import { getAllMetadata } from 'utils/db';
import { MetadataHandle } from 'types/metadata';

/*
export function useFullMetadataHook(
	metadataHandle: MetadataHandle
): [boolean, string] {
	const [metadataReady, setMetadataReady] = useState<boolean>(false);
	metadata = await getMetadata(metadataHandle);
	setMetadataReady(true);
	return [metadataReady, metadata];
}


function isSpecName(metadataHandle: MetadataHandle, _index, _array): bool {
	return metadataHandle.specName === this;
}

export function useKnownMetadataHook(
	networkName: string
): Array<MetadataHandle> {
	const allMetadata = getAllMetadata();
	return allMetadata.filter(isSpecName, networkName);
}*/
