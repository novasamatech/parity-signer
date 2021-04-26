import { MetadataHandle } from 'types/metadata';
import { generateMetadataHandle } from 'utils/native';

export const metadataHandleToKey = (metadataHandle: MetadataHandle): string => {
	if (metadataHandle.specVersion) {
		return metadataHandle.specName + '_v' + metadataHandle.specVersion;
	} else {
		return metadataHandle.hash;
	}
};

export async function getMetadataHandleFromRaw(
	metadataRaw: string
): Promise<MetadataHandle> {
	const metadataHandle = await generateMetadataHandle(metadataRaw);
	return metadataHandle;
}
