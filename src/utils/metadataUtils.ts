import { MetadataHandle } from 'types/metadata';
import { generateMetadataHandle } from 'utils/native';

export const metadataStorage = 'signer_metadata_';

export const metadataHandleToKey = (metadataHandle: MetadataHandle): string => {
	if (metadataHandle.specVersion) {
		return (
			metadataStorage +
			metadataHandle.specName +
			'_v' +
			metadataHandle.specVersion
		);
	} else {
		return metadataStorage + metadataHandle.hash;
	}
};

export async function getMetadataHandleFromRaw(
	metadataRaw: string | null
): Promise<MetadataHandle> {
	const metadataHandle = await generateMetadataHandle(metadataRaw + '');
	return metadataHandle;
}
